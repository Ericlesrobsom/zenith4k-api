# Apps Ibo (msproapp)

Modulo de ativacao por MAC do `msproapp.top`. **Nao confundir** com o
`apps_ativa` (produto pago, com saldo em R$): Apps Ibo e o modulo de ativacao
sem custo, compartilhado entre a franquia e as revendas liberadas.

Fonte dos contratos: `routes_msproapp.go`, `msproapp_clientes.go`,
`msproapp_contas.go`, `msproapp_posse_api.go`, `msproapp_payload.go`.

## API local

| Operacao | Endpoint | Rota no painel |
|---|---|---|
| Contas visiveis | `GET /v1/apps-ibo/contas` | `GET /merger/api/cliente/msproapp_contas` |
| Apps da conta | `GET /v1/apps-ibo/apps?conta=36` | `GET .../msproapp/aplicativos?conta=` |
| Clientes | `GET /v1/apps-ibo/clientes?conta=36` | `GET .../msproapp/clientes?conta=` |
| Ativar MAC | `POST /v1/apps-ibo/ativar` | `POST .../msproapp/clientes?conta=&geracao=` |
| Excluir | `DELETE /v1/apps-ibo/clientes/{id}?conta=36&confirm=true` | `DELETE .../msproapp/cliente?id=&conta=&geracao=` |
| Atribuir a revenda | `POST /v1/apps-ibo/clientes/{id}/atribuir?conta=36` | `POST .../msproapp/clientes/atribuir?conta=&geracao=` |

## `geracao` (concorrencia)

A conta tem um inteiro `geracao` que muda quando a conexao e reaberta. Toda
**mutacao** exige o valor atual na query. O cliente Rust:

1. le `geracao` em `msproapp_contas` antes de cada mutacao;
2. envia `?conta=&geracao=`;
3. se o painel responder `409 geracao_alterada`, refaz a leitura e o request
   **uma vez**;
4. se ainda falhar, devolve `409 conta_desatualizada`.

O chamador nunca gerencia `geracao`.

## Ativacao

```json
{
  "conta": 36,
  "app_id": "10",
  "mac": "02:11:22:33:44:55",
  "modo": "1",
  "server_name": "QA API",
  "price": "0",
  "expire_date": "2026-12-31",
  "m3u8_list": "http://exemplo/get.php?username=...&password=..."
}
```

- `modo`: `"1"` M3U8 (default) ou `"0"` XTeam. Tambem aceita `m3u8`/`xteam`.
- `app_id` e o `ranking_app_id` da lista viva (`GET /apps`).
- `mac`: 12 caracteres alfanumericos; aceita `:`, `-`, `.` e espaco.
- `price`: aceita `30`, `30,50` e `1.234,56`; normalizado para ponto decimal.
- `expire_date`: `AAAA-MM-DD`.
- No modo M3U8, `m3u8_list` e obrigatoria.
- No modo XTeam, `xteam_username` e `xteam_password` sao obrigatorios e
  `dns` e opcional.
- App `11` (ZONE X) aceita `username_login`/`password_login`.
- `is_trial: true` vira `is_trial: 1`.

O corpo enviado ao painel tem as **19 chaves** do formulario original, com
string vazia/null nos campos do modo nao usado — o Laravel valida presenca.
Resposta de sucesso:

```json
{"ok":true,"msg":"cliente_criado","mac":"02:11:22:33:44:55","id":"4801338","id_local":"..."}
```

## Exclusao e atribuicao

- Exclusao e destrutiva: exige `confirm=true` na API local (o painel nao pede
  esse parametro, a trava e nossa).
- Atribuicao so funciona para a **franquia dona**; a revenda recebe 403 do
  painel. O `dono` e o username da revenda; o cliente resolve o `dono_id` na
  lista `donos` devolvida pela conta.

## Sessao de revenda (fase 2)

Quando a sessao e de uma **sub-revenda**:

- `GET /v1/apps-ibo/contas` devolve **so as contas que o painel liberou para
  ela** (vinculo nominal em `msproapp_conta_revendas` ou o atalho
  `compartilhado=1` da franquia raiz). A API nao amplia nada: mescla
  `minhas` + `disponiveis` exatamente como veio.
- Contas globais do admin (`franquia_user` vazio) sao visiveis para todos;
  contas de outra franquia nunca aparecem.
- `pode_atribuir` vem `false`; `POST /v1/apps-ibo/clientes/{id}/atribuir`
  devolve **403 `sem_permissao`** (o painel tambem responde `apenas_franquia`).
- Ativar, listar e excluir dentro da conta liberada continuam permitidos,
  sujeitos ao feature gate `msproapp` da revenda.
- O e-mail e a lista `liberadas` da conta nunca saem no resumo da revenda.

## Anti-leak

- `GET /v1/apps-ibo/contas` devolve **apenas** `id`, `apelido`, `conectada` e
  `painel`. `cofre_uuid`, e-mail e credenciais nunca saem.
- `GET /v1/apps-ibo/clientes` remove `cofre_uuid` e `geracao`.
- A listagem do painel ja e um resumo sem `m3u8_list`; o `m3u8_list` (com
  usuario/senha do cliente final) so viaja no POST de ativacao.
- O servidor local nao loga corpo de request; a auditoria grava apenas
  metodo/caminho/status.

## Erros

| Situacao | HTTP local | Codigo |
|---|---|---|
| Corpo invalido (MAC, data, preco, modo) | 400 | `parametros_invalidos` |
| Conta nao visivel | 404 | `nao_encontrado` |
| `geracao` mudou duas vezes | 409 | `conta_desatualizada` |
| Exclusao sem `confirm` | 409 | `confirmacao_necessaria` |
| Painel externo fora/recusou | 502 | `erro_painel` |

## Testes

- Fake panel (`scripts/fake-panel.mjs`) simula a primeira mutacao com
  `409 geracao_alterada` para exercitar o retry automatico.
- E2E do fake: ativar -> listar (1) -> atribuir -> excluir -> listar (0).
- E2E real: ativar com MAC descartavel, listar, excluir e confirmar lista
  vazia (feito com a conta 36; cliente de teste removido).
