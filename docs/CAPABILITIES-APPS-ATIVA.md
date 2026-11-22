# Apps Ativa (Ative App)

Ativacao **paga** por MAC, com saldo em R$ (`apps_ativa_saldo_cents`). O painel
debita o saldo antes de chamar o provedor e devolve (refund) se o provedor
recusar. **Nao confundir** com o Apps Ibo (`msproapp`), que e ativacao sem
custo.

Fonte dos contratos: `routes_apps_ativa.go`, `apps_ativa_ativar.go`,
`apps_ativa.go`, `apps_ativa_listar.go`, `apps_ativa_webhook.go`.

## API local

| Operacao | Endpoint | Rota no painel |
|---|---|---|
| Saldo | `GET /v1/apps-ativa/saldo` | `GET /merger/api/cliente/apps_ativa/saldo` |
| Catalogo | `GET /v1/apps-ativa/apps` | `GET /merger/api/apps_ativa/aplicativos` |
| Movimentacoes | `GET /v1/apps-ativa/historico` | `GET .../apps_ativa/historico` |
| Ativacoes/status | `GET /v1/apps-ativa/ativacoes?status=` | `GET .../apps_ativa/historico_ativacoes` |
| Ativar | `POST /v1/apps-ativa/ativar` | `POST .../apps_ativa/ativar` |

CLI: `apps-ativa-saldo`, `apps-ativa-apps`, `apps-ativa-historico`,
`apps-ativa-ativacoes`, `apps-ativa-ativar`.

## MAC: normalizado antes do debito

O painel **nao** normaliza o MAC: ele debita o saldo e manda o texto cru ao
provedor, que recusa o formato errado. Nossa API normaliza antes de qualquer
request:

| Entrada | Enviado ao painel |
|---|---|
| `ab516e667960` | `ab:51:6e:66:79:60` |
| `AB-51-6E-66-79-60` | `ab:51:6e:66:79:60` |
| `ab.51.6e.66.79.60` | `ab:51:6e:66:79:60` |
| `ab 51 6e 66 79 60` | `ab:51:6e:66:79:60` |
| `abc` | `400 mac_invalido` (sem debito) |
| `gg:51:6e:66:79:60` | `400 mac_invalido` (sem debito) |

Formato final: **12 hexadecimais, minusculo, dois pontos**. Separadores
aceitos: `:`, `-`, `.` e espaco. Qualquer outro caractere e recusado.

> Apps Ibo usa outro contrato (12 alfanumericos, **maiusculo**, aceita letras
> fora de A-F). Por isso existem dois helpers em `crates/merger-core/src/mac.rs`:
> `normalizar_mac_provedor` (Ative App) e `normalizar_mac_ibo` (msproapp).

## `key_app`

Campo opcional ("depende do app"). Validacao local:

- vazio = nao enviado (string vazia);
- maximo 128 caracteres;
- sem espacos/controle;
- apenas ASCII imprimivel.

Invalida -> `400 key_app_invalida`, antes do request e sem debito.

## Fluxo de ativacao

```json
{"app_uuid":"1c2e...","mac_app":"AB-51-6E-66-79-60","key_app":"123456","descricao":"cliente Joao"}
```

1. valida `app_uuid`, normaliza MAC, valida `key_app` e descricao;
2. painel resolve o preco em R$ no servidor (nunca confia no front);
3. painel valida saldo, debita e insere a ativacao `Solicitado`;
4. painel chama o provedor;
5. sucesso -> grava `id_cobranca` e devolve `ativacao_id`, `saldo_cents`;
6. falha -> refund automatico (`refunded: true`) e status `Erro`.

A API local traduz `{ok:false}` do painel (saldo insuficiente, provedor
recusou) em **409 `erro_negocio_painel`**, com o corpo original em
`error.details.panel`.

## Status preso em "Solicitado"

O painel **so** atualiza o status quando o Ative App chama o webhook:

```
POST /api/webhook/apps_ativa/{token}
```

- O token/URL ficam em `GET /merger/api/admin/apps_ativa/webhook_info`
  (admin-only) e podem ser rotacionados em `.../webhook_rotate`.
- O webhook mapeia `Ativado`/`Aprovado` (so atualiza) e
  `Reprovado`/`Incorreto` (atualiza + refund), com idempotencia.
- **Nao existe** rota de consulta/polling/refresh no escopo do cliente (nem do
  admin) para reconsultar o provedor. Se o webhook nao estiver configurado no
  Ative App, a ativacao fica `Solicitado` para sempre.

Para resolver o caso das ativacoes #3/#4: copiar a URL de `webhook_info` e
cadastrar no painel do Ative App. Um refresh sem webhook exigiria uma rota
nova no Go consultando a API de status do provedor.

### Cadastrar o webhook (admin)

O token e **global** (`config_geral.apps_ativa_webhook_token`), nao e por
franquia — por isso a rota e admin-only. Com sessao de admin:

```bash
# Ver URL + token (a URL usa o host da requisicao)
curl -s -b cookies-admin.txt \
  https://aplicativo.znt4kh.uk/merger/api/admin/apps_ativa/webhook_info

# Rotacionar (invalida a URL antiga; atualize no Ative App na hora)
curl -s -X POST -b cookies-admin.txt \
  https://aplicativo.znt4kh.uk/merger/api/admin/apps_ativa/webhook_rotate
```

Formato da URL: `https://<host>/api/webhook/apps_ativa/<token>` (token de 64
hex). Copie exatamente o que a rota devolver — o host vem da requisicao.

### Ativacoes antigas (#3/#4)

Cadastrar a URL **nao reprocessa** eventos passados: o Ative App so entrega
eventos novos. As antigas continuam `Solicitado` ate:

1. o provedor reenviar o webhook daquela cobranca (se tiver botao de
   reenvio/retry no painel deles); ou
2. alguem enviar um webhook sintetico com o token (abaixo); ou
3. uma rota nova de refresh no Go consultando o status no provedor.

> **Requisito duro:** o webhook casa **somente** por `id_cobranca`. Ativacao
> com `id_cobranca` vazio **nao pode** ser atualizada por webhook sintetico —
> o handler responde `id_cobranca obrigatorio` (400) ou
> `id_cobranca nao encontrado localmente` (200). Foi o caso das ativacoes
> #3/#4 reais: ambas ficaram com `id_cobranca` vazio.

Webhook sintetico (match por `id_cobranca`; `id_cobranca` e `status` sao
obrigatorios):

```bash
curl -s -X POST "https://aplicativo.znt4kh.uk/api/webhook/apps_ativa/<TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{"id_cobranca":"<ID_COBRANCA>","status":"Ativado"}'
```

- `Ativado`/`Aprovado`: so atualiza o status.
- `Reprovado`/`Incorreto`: atualiza **e faz refund real** de `valor_cents` na
  carteira (`origem='painel'`). So envie depois de confirmar que o provedor
  recusou e que o credito nao foi devolvido.
- Idempotencia: se a ativacao ja tem `webhook_recebido_em > 0` e status final,
  a entrega e ignorada (`ignored: "duplicado"`).
- Ativacao nao encontrada por `id_cobranca` responde 200 com `ok:false`.

> O token do webhook da acesso a qualquer ativacao. Trate como segredo de
> admin; se vazar, rotacione e atualize no Ative App.

### Ativacao criada sem `id_cobranca`

O painel so grava `id_cobranca` quando a resposta do provedor traz
`historico_id` (ver `solicitarAtivacaoResp` em `apps_ativa_ativar.go`). Se o
provedor aceitar a solicitacao mas responder em outro formato (ou sem id), a
ativacao nasce `Solicitado` e **nunca** sera casada pelo webhook.

Nossa API agora sinaliza isso na resposta da ativacao:

```json
{"ok":true,"ativacao_id":3,"id_cobranca":"","webhook_sincronizavel":false,
 "aviso":"ativacao criada sem id_cobranca; o webhook do Ative App nao conseguira atualizar o status desta ativacao"}
```

Para as ativacoes antigas sem id:

1. tentar o **retry/reenvio no painel do Ative App**; se ele exibir o
   `id_cobranca` da cobranca, o sintetico passa a funcionar;
2. se o provedor confirmar a recusa mas nao exibir o id, o estorno da #3
   depende do suporte do provedor ou de um ajuste manual de saldo — nao
   invente um id (o webhook nao casa e o saldo nao volta);
3. para **novas** ativacoes, a correcao definitiva e no Go: aceitar aliases
   (`historico_id`, `id`, `data.id`) na resposta do provedor e logar aviso
   quando `sucesso:true` vier sem id. Ainda nao implementado (mudanca no
   painel de producao, precisa de autorizacao).

## Erros

| Situacao | HTTP local | Codigo |
|---|---|---|
| MAC invalido | 400 | `mac_invalido` |
| `key_app` invalida | 400 | `key_app_invalida` |
| Corpo invalido | 400 | `parametros_invalidos` |
| Saldo insuficiente / provedor recusou | 409 | `erro_negocio_painel` |
| Sem sessao | 401 | `sessao_expirada` |
| Painel/provedor fora | 502 | `erro_painel` |

## Testes

- Unitarios: formatos de MAC do pedido, `abc`/`gg:...`, key com espaco,
  descricao truncada em 200, payload exato.
- Fake panel: ativacao guarda o MAC **normalizado** (`mac_recebido`), simula
  saldo insuficiente e recusa de provedor.
- E2E do fake: saldo -> catalogo -> ativar com `AB-51-6E-66-79-60` (chega
  `ab:51:6e:66:79:60`) -> ativacoes -> erro de negocio 409.
- E2E real pago: **nao executado** por esta API (custaria credito). O caso real
  do IBO PLAYER SMARTER (ativacao #4) foi validado manualmente pelo dono.
