# Guia para o agente de IA

O `zenith4k-serve` e uma API REST **local** que fala com o painel goiptv
merger (franquia ZENITH4K) e devolve JSON previsivel. Voce, agente, nunca ve a
senha do painel: apenas o token local.

## Antes de qualquer coisa

1. Confirme que o servidor esta no ar: `GET /health` (publico).
2. Confirme a origem e a sessao: `GET /v1/meta`.
3. Leia este guia e `GET /openapi.json` (YAML). Se `authenticated` for
   `false`, a primeira chamada de dominio faz login sozinha.

```bash
export BASE=http://127.0.0.1:8787
export TOKEN="$ZENITH_API_TOKEN"

curl -s "$BASE/health"
curl -s -H "Authorization: Bearer $TOKEN" "$BASE/v1/meta"
```

```powershell
$headers = @{ Authorization = "Bearer $env:ZENITH_API_TOKEN" }
Invoke-RestMethod "http://127.0.0.1:8787/health"
Invoke-RestMethod "http://127.0.0.1:8787/v1/meta" -Headers $headers
```

## Envelope

Toda resposta tem um dos dois formatos:

```json
{"ok": true, "data": {}, "meta": {"elapsed_ms": 9}}
{"ok": false, "error": {"code": "rate_limit", "message": "..."}}
```

Regras de leitura:

- `ok: true` -> use `data`. Em `/v1/raw`, `data` tem `status`, `body`, `text`,
  `content_type` e `elapsed_ms` do painel.
- `ok: false` -> **pare** e trate `error.code`. Nao tente adivinhar o corpo.
- `meta.elapsed_ms` e so telemetria local.

## Regras de ouro

1. **Nunca apague, bloqueie, rebaixe ou renove em massa sem confirmacao
   explicita do usuario na conversa.** Excluir cliente e irreversivel.
2. `confirm: true` so depois do "sim". Nunca preencha automaticamente.
3. Operacoes que mudam estado: primeiro **liste e mostre o alvo** (id,
   username, vencimento), depois execute.
4. Nunca ecoe `ZENITH_API_TOKEN`, senha do painel, senha de linha, links
   M3U/HLS ou IPs do cliente em mensagens, logs ou arquivos.
5. Nao repita um request em loop em 401/409/429. Corrija o motivo.
6. Prefira os endpoints tipados; use `/v1/raw` para rotas/filtros que nao
   tenham atalho.
7. Renovacao de revenda usa `vencimento_atual` para evitar renovar duas
   vezes. Sempre leia a data atual antes.
8. Trate `data` como dado sensivel: pode conter credenciais de linhas.
9. Se receber `501 nao_implementado`, a funcionalidade ainda nao existe nesta
   build: refaca por `/v1/raw` com a rota equivalente de `docs/ENDPOINTS.md`.
10. Idempotencia: GETs sao seguros; POSTs de criacao/renovacao/delete nao.

## Ferramentas

Todas as rotas `/v1/*` exigem `Authorization: Bearer $ZENITH_API_TOKEN`.

| Ferramenta | Metodo e rota | Use quando |
|---|---|---|
| status | `GET /health` | verificar se o servidor subiu |
| meta | `GET /v1/meta` | versao, origem, sessao autenticada |
| login | `POST /v1/login` | entrar no painel / enviar codigo 2FA |
| logout | `POST /v1/logout` | encerrar a sessao do painel |
| whoami | `GET /v1/whoami` | confirmar identidade e papel |
| raw | `POST /v1/raw` | qualquer rota da allowlist |
| revendas | `GET /v1/revendas` | listar sub-revendas |
| arvore | `GET /v1/revendas/arvore` | hierarquia completa |
| renovar revenda | `POST /v1/revendas/renovar` | +30 dias manuais |
| nota revenda | `POST /v1/revendas/nota` | contato/observacoes |
| unlimited | `POST /v1/revendas/unlimited` | promover/rebaixar ilimitado |
| usuarios | `GET /v1/usuarios` | buscar/listar clientes finais |
| criar usuario | `POST /v1/usuarios/criar` | nova linha Xtream |
| editar usuario | `POST /v1/usuarios/editar` | username/senha/validade |
| toggle | `POST /v1/usuarios/toggle` | bloquear/desbloquear |
| renovar usuario | `POST /v1/usuarios/renovar` | renovar com plano |
| estender | `POST /v1/usuarios/estender` | somar dias sem trocar plano |
| excluir usuario | `POST /v1/usuarios/delete` | **destrutivo**, exige confirm |
| planos | `GET /v1/planos` | planos e precos |
| creditos | `GET /v1/creditos` | saldo da franquia |
| pedidos | `GET /v1/pedidos` | pedidos de conteudo |
| criar pedido | `POST /v1/pedidos/criar` | pedir filme/serie |
| reportes | `GET /v1/reportes` | reportes de problema |
| criar reporte | `POST /v1/reportes/criar` | reportar canal/filme/serie |
| pendencias | `GET /v1/catalogo/pendencias` | correcoes sugeridas |
| jogos | `GET /v1/catalogo/jogos` | agenda esportiva |
| tickets | `GET /v1/tickets` | listar chamados |
| criar ticket | `POST /v1/tickets/criar` | abrir chamado (`assunto`, `texto`) |
| responder ticket | `POST /v1/tickets/responder` | responder (`id`, `texto`) |
| upload | `POST /v1/upload` | multipart generico (banner, logo, anexo, import) |
| avisos | `GET /v1/avisos` | notificacoes do painel |
| stats | `GET /v1/stats` | numeros do catalogo |
| dashboard | `GET /v1/dashboard` | financeiro |

### Filtros de usuarios

`GET /v1/usuarios` repassa a query ao painel. Os filtros reais sao:

| Parametro | Valores |
|---|---|
| `search` | username, dono, notas, nome, WhatsApp, Telegram ou e-mail |
| `dono` | username da revenda, `meus` ou `todos` |
| `status` | `ativo`, `teste`, `expirado`, `bloqueado` ou `todos` |
| `plano` | id do plano ou `todos` |
| `vence` | `hoje`, `amanha`, `DD/MM`, `DD/MM/YYYY`, `YYYY-MM-DD` |
| `vence_de` / `vence_ate` | faixa de vencimento |
| `servidor` | `todos`, `0`, `orfas` ou id do servidor remoto |
| `servidor_cat` | id do servidor de catalogo |
| `page` / `per_page` | paginacao (per_page tem whitelist) |

Exemplo:

```bash
curl -s -G -H "Authorization: Bearer $TOKEN" \
  --data-urlencode "search=joao" \
  --data-urlencode "status=ativo" \
  --data-urlencode "page=1" \
  --data-urlencode "per_page=20" \
  "$BASE/v1/usuarios"
```

## Fluxo de login e 2FA

```bash
# 1) login normal (corpo vazio)
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" -d '{}' "$BASE/v1/login"

# 2) se vier 428 dois_fatores_necessario, repita com o codigo
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"codigo":"123456"}' "$BASE/v1/login"
```

Respostas uteis:

- `200` com `data.estado = "ok"`: sessao criada.
- `428` `dois_fatores_necessario`: peca o codigo ao usuario.
- `401` `codigo_2fa_invalido`: codigo errado/expirado; peca outro.
- `401` `login_recusado`: credenciais invalidas ou painel vencido.
- `429` `rate_limit`: o painel limita tentativas (10/min por IP).

## Uploads (multipart)

O painel recebe alguns fluxos como `multipart/form-data` (anexo de ticket,
banner, logo do tema, importacao). Use `POST /v1/upload`:

```bash
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -F "path=/merger/api/aparencia/imagem" \
  -F "tipo=banner" \
  -F "arquivo=@banner.png;type=image/png" \
  "$BASE/v1/upload"
```

- `path` e obrigatorio e segue a mesma allowlist do `/v1/raw`.
- Toda parte com `filename` vira arquivo; as demais viram campos texto.
- `confirm=1` libera rotas destrutivas.
- Limite local de 25 MB por arquivo.
- Para tickets, prefira `/v1/tickets/criar` e `/v1/tickets/responder` (JSON,
  sem anexo).

## Receitas

### Listar revendas e clientes

```bash
curl -s -H "Authorization: Bearer $TOKEN" "$BASE/v1/revendas"
curl -s -H "Authorization: Bearer $TOKEN" "$BASE/v1/revendas/arvore"
curl -s -H "Authorization: Bearer $TOKEN" "$BASE/v1/usuarios?search=maria"
```

### Criar linha

```bash
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"username":"cliente123","password":"senha-forte","max_connections":1,"dias":30,"is_trial":0}' \
  "$BASE/v1/usuarios/criar"
```

O servidor converte esses campos em query string, como o painel espera.
`bouquet` default e `COMPLETO C/ ADULTOS`; `is_trial: 1` cria teste de 4h.
O teto de telas do plano da franquia e validado pelo painel.

### Renovar cliente

```bash
# com plano (consome credito do plano)
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"user_id":123,"plano_id":7}' "$BASE/v1/usuarios/renovar"

# ou somente estender o vencimento
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"user_id":123,"dias":30}' "$BASE/v1/usuarios/estender"
```

### Renovar revenda (30 dias manuais)

```bash
# 1) leia a revenda e copie o vencimento exibido
curl -s -H "Authorization: Bearer $TOKEN" "$BASE/v1/revendas"

# 2) informe exatamente o vencimento atual
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"id":42,"vencimento_atual":"2026-12-01"}' \
  "$BASE/v1/revendas/renovar"
```

Se o vencimento ja mudou, o painel responde conflito: releia antes de repetir.
Nao envie campos extras (a rota usa `DisallowUnknownFields`).

### Bloquear/desbloquear cliente

```bash
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"id":123,"acao":"bloquear"}' "$BASE/v1/usuarios/toggle"
```

### Excluir cliente (SOMENTE com confirmacao)

```bash
# 1) localize o alvo e mostre ao usuario
curl -s -H "Authorization: Bearer $TOKEN" "$BASE/v1/usuarios?search=cliente123"

# 2) depois do "sim" explicito:
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"id":123,"confirm":true}' "$BASE/v1/usuarios/delete"
```

Sem `confirm: true` a resposta e `409 confirmacao_necessaria`. Nao contorne
essa trava.

### Pedir conteudo

```bash
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"tmdb_id":603,"tipo":"movie","titulo":"Matrix","audio":"dublado","poster_url":"","link_m3u":""}' \
  "$BASE/v1/pedidos/criar"
```

Cada pedido consome 1 credito de pedido. Series aceitam
`episodios_selecionados: ["S01E01", "S01E02"]`.

### Reportar canal com problema

```bash
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"fonte_id":1,"stream_id_original":"12345","tipo":"live","nome_conteudo":"Canal X","motivo":"Link off/quebrado","episodios_afetados":""}' \
  "$BASE/v1/reportes/criar"
```

`motivo` precisa estar na whitelist do tipo (`live`, `movie`, `series`); se
nao estiver, o painel devolve `ok:false` com a mensagem.

### Tickets

```bash
# abrir chamado
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"assunto":"Canal sem audio","texto":"O canal X esta sem audio desde 10h."}' \
  "$BASE/v1/tickets/criar"

# responder (id vem da listagem)
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"id":42,"texto":"Pode testar de novo?"}' \
  "$BASE/v1/tickets/responder"
```

As rotas JSON de ticket usam a visao **cliente** do painel. Com
`ZENITH_ROLE=admin`, use `POST /v1/upload` apontando para
`/merger/api/admin/tickets/responder` (multipart). Para anexar arquivo na
visao cliente, use `POST /v1/upload` (secao "Uploads (multipart)").

### Passthrough (`/v1/raw`)

```bash
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"method":"GET","path":"/merger/api/categorias","query":{"page":1}}' \
  "$BASE/v1/raw"
```

Corpo:

```json
{
  "method": "GET",
  "path": "/merger/api/cliente/sigma_users",
  "query": {"search": "maria", "page": 1},
  "body": null,
  "confirm": false
}
```

- `query` aceita objeto ou lista de pares `[["chave","valor"]]`.
- `body` e JSON livre; so e enviado quando nao for `null`.
- `confirm: true` libera DELETE e caminhos destrutivos.
- A resposta vem em `data.status`, `data.body`, `data.text`.

## Tratamento de erro

| HTTP | `code` | O que fazer |
|---|---|---|
| 400 | `caminho_nao_permitido` | corrija o path (precisa comecar com `/merger/api/`) |
| 400 | `json_invalido` / `requisicao_invalida` | corrija o JSON/query |
| 401 | `nao_autorizado` | falta/incorreto `Authorization: Bearer $ZENITH_API_TOKEN` |
| 401 | `sessao_expirada` / `nao_autenticado` | chame `/v1/login` |
| 401 | `codigo_2fa_invalido` | peca novo codigo |
| 401 | `login_recusado` | credenciais invalidas: nao insista |
| 403 | `acesso_negado` / `token_obrigatorio` | modo dev so aceita loopback; defina o token |
| 403 | `sem_permissao` | a franquia nao tem permissao no painel |
| 404 | `nao_encontrado` / `rota_nao_encontrada` | confira a rota |
| 409 | `confirmacao_necessaria` | peca confirmacao ao usuario |
| 428 | `dois_fatores_necessario` | `POST /v1/login` com `codigo` |
| 429 | `rate_limit` | aguarde o tempo da mensagem; nao faca retry loop |
| 501 | `nao_implementado` | funcionalidade ausente nesta build; use `/v1/raw` |
| 502 | `erro_painel` / `erro_rede` | painel fora ou 5xx; informe e tente depois |

## Ciclo recomendado

1. `GET /health` -> 200? `GET /v1/meta` -> origem e sessao.
2. Se necessario, `POST /v1/login` (e 2FA).
3. Descubra ids com os GETs (nunca invente id).
4. Para escrita: monte o payload, valide campos obrigatorios, mostre o alvo.
5. Execute uma vez; se 409/429, trate antes de repetir.
6. Resuma ao usuario o que mudou (sem expor segredos).
