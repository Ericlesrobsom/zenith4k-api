# Arquitetura

Como o `zenith4k-api` se organiza: um cliente HTTP do painel, uma API REST
local para agentes de IA e uma CLI para operacao manual.

```
+-------------------+        HTTP local         +----------------------+
| agente de IA      |  Bearer ZENITH_API_TOKEN  |  zenith4k-serve      |
| (curl, LangChain, | ------------------------> |  axum 0.8            |
|  MCP, script...)  |                           |  envelope + auditoria|
+-------------------+                           +----------+-----------+
                                                           |
                                                  Arc<MergerClient>
                                                           |
                                                +----------v-----------+
                                                |  merger-core         |
                                                |  reqwest + cookie jar|
                                                |  allowlist + confirm |
                                                +----------+-----------+
                                                           |
                                                           | HTTPS
                                                +----------v-----------+
                                                | painel goiptv merger |
                                                | (ZENITH4K)           |
                                                +----------------------+

+-------------------+       HTTPS direto
| zenith4k (CLI)    | ------------------------> painel (mesmo merger-core)
+-------------------+
```

## Camadas

### `merger-core`

Transporte unico para o painel:

- `MergerClient` mantem um **cookie jar** (sessao `merger_session`), timeout,
  user-agent e redirects limitados.
- `MergerClient::raw(RawRequest)` e o passthrough autenticado: valida o
  caminho contra a allowlist (`/merger/api/`, `/api/webhook/`, `/api/mcp`),
  aplica a trava `confirm` em operacoes destrutivas e normaliza a resposta em
  `RawResponse`.
- `MergerClient::login` / `login_with_code` cuidam do login de franquia ou
  admin e do 2FA.
- Os modulos `merger_core::api::*` sao atalhos tipados por dominio. O
  servidor e a CLI **nao** dependem deles: falam com o painel por `raw` como
  caminho unico, entao um atalho novo (ou ausente) nao muda o contrato local.

### `merger-server` (`zenith4k-serve`)

- `AppState` guarda `Arc<MergerClient>`, o token local, o diretorio de
  auditoria e o horario de boot.
- `app(state)` monta o `Router` (biblioteca separada do binario para testes
  com `tower::ServiceExt`).
- Handlers tipados traduzem JSON/query local para `RawRequest`, chamam o
  painel e embrulham o corpo no envelope. Nada de logica de negocio duplicada:
  o painel continua sendo a fonte de verdade.
- Chamadas passam por `AppState::call`, que faz **login sob demanda**: se o
  painel responder 401, chama `ensure_login()` e repete o request uma unica
  vez. Assim o processo sobe sem tocar na rede.

### `merger-cli` (`zenith4k`)

Cliente direto do painel (sem passar pelo servidor local), pensado para
diagnostico e operacao manual. `whoami`, `get`, `post`, `call`, comandos de
listagem e `login --codigo`.

## Fluxo de autenticacao

Existem **duas** autenticacoes independentes:

1. **Painel** (`merger-core`): `ZENITH_USER`/`ZENITH_PASS` viram
   `POST /merger/api/cliente_login` (franquia) ou `POST /merger/api/login`
   (admin). O painel responde com o cookie `merger_session`; o cookie jar
   guarda e reenvia. O agente local **nunca** recebe essa senha.
2. **API local** (`merger-server`): o agente envia
   `Authorization: Bearer $ZENITH_API_TOKEN`. Sem token configurado, o
   servidor aceita somente conexoes loopback e loga um aviso no boot (modo
   dev).

```
agente                servidor                 painel
  |  GET /v1/revendas   |                        |
  |-------------------->|  GET /merger/api/...   |
  |                     |----------------------->|
  |                     |<------ 401 ------------|
  |                     |  POST cliente_login    |
  |                     |----------------------->|
  |                     |<------ 200 + cookie ---|
  |                     |  GET /merger/api/...   |
  |                     |----------------------->|
  |<---- 200 envelope --|<------ 200 ------------|
```

O login sob demanda evita duas armadilhas: (a) subir o servidor sem
credenciais validas ainda no boot; (b) manter uma sessao expirada em cache. Se
o painel cair no meio, a resposta e `502 erro_painel` ou `502 erro_rede`.

### 2FA

Quando o painel exige codigo por e-mail, o login devolve 428
`dois_fatores_necessario`. O agente repete `POST /v1/login` com
`{"codigo":"123456"}`. Codigo invalido vira 401 `codigo_2fa_invalido`. O
codigo e a senha nunca sao logados nem auditados.

## Passthrough x endpoints tipados

| Necessidade | Use |
|---|---|
| Fluxo recorrente e documentado (listar revendas, criar cliente...) | endpoint tipado `/v1/...` |
| Rota nova, filtro exotico, diagnostico | `POST /v1/raw` |
| Automacao em massa com parametros do painel | `/v1/raw` (query livre) |
| Operacao destrutiva | qualquer um dos dois, com `confirm: true` |

Regras de decisao:

- O tipado existe para dar **nome, documentacao e validacao** ao fluxo. Ele
  nao substitui o passthrough: o painel tem centenas de rotas e o catalogo
  completo esta em `docs/ENDPOINTS.md`.
- O tipado **nao** inventa campos. Os nomes sao os do painel (snake_case).
  Onde o painel le query string em vez de JSON (criar/toggle/delete de
  usuarios), o servidor converte o corpo em query automaticamente.
- `confirm` e um campo **local**: o servidor o remove do corpo antes de
  repassar (o painel rejeita campos extras em algumas rotas, como
  `revendas/renovar`).
- Se um atalho de `merger-core` faltar, o handler usa `client.raw(...)` e o
  caso fica registrado como pendencia; nao se edita o painel para acomodar o
  cliente. (Os atalhos ja existem hoje, mas o servidor prefere o passthrough
  para manter um unico caminho de request.)

## Envelope e erros

Sucesso:

```json
{"ok": true, "data": {"...": "corpo do painel"}, "meta": {"elapsed_ms": 12}}
```

Erro:

```json
{"ok": false, "error": {"code": "confirmacao_necessaria", "message": "..."}}
```

| `ApiError` | HTTP | `code` |
|---|---|---|
| `NotAuthenticated` | 401 | `nao_autenticado` |
| `SessionExpired` | 401 | `sessao_expirada` |
| `TwoFactorRequired` | 428 | `dois_fatores_necessario` |
| `InvalidTwoFactor` | 401 | `codigo_2fa_invalido` |
| `Forbidden` | 403 | `sem_permissao` |
| `NotFound` | 404 | `nao_encontrado` |
| `ConfirmationRequired` | 409 | `confirmacao_necessaria` |
| `RateLimited` | 429 | `rate_limit` |
| `PathNotAllowed` | 400 | `caminho_nao_permitido` |
| `Panel` | 502 | `erro_painel` |
| `Http` | 502 | `erro_rede` |
| `LoginRejected` | 401 | `login_recusado` |
| `NotImplemented` | 501 | `nao_implementado` |
| `Config` / `Json` / `Url` | 500 | erro interno |

`LoginRejected` (401) e `NotImplemented` (501) nao estavam na tabela do brief;
foram mapeados para o status mais fiel ao significado, e o restante segue
`500` como pedido.

Fallbacks (`404 rota_nao_encontrada`, `405 metodo_nao_permitido`) e rejeicoes
de JSON (`400 json_invalido`) usam o mesmo envelope de erro.

## Seguranca

- **Token local**: comparado em tempo constante. Sem token, somente peer
  loopback; requests sem `ConnectInfo` sao recusadas para nao abrir o servico
  por engano atras de proxy.
- **Allowlist de caminho**: impede SSRF para hosts externos (`://` e `..` sao
  rejeitados) e limita o passthrough aos prefixos do painel.
- **Trava `confirm`**: DELETE, delete/remover/excluir/reset/massa... exigem
  `confirm: true`. A checagem acontece **antes** de qualquer I/O com o painel.
- **Auditoria**: `./audit/api-AAAA-MM-DD.jsonl` grava somente
  `{ts,method,path,status,elapsed_ms}`. Corpo, query e cabecalhos ficam de
  fora (podem conter senha, token, IP de cliente ou link de linha). Falha de
  auditoria gera `warn` e nao derruba a request.
- **CORS**: liberado (API local), mas o token continua obrigatorio. Se for
  expor o servidor, coloque-o atras de um proxy com TLS e mantenha o bind em
  loopback.
- **Segredos**: `ZENITH_PASS` e `ZENITH_API_TOKEN` vem do ambiente/.env
  (ignorado pelo Git). O repositorio nao persiste sessao em disco.

## Operacao

Variaveis:

Os binarios carregam `.env` automaticamente (via `dotenvy`, sem sobrescrever
variaveis ja definidas); em CI/producao use o ambiente.

| Variavel | Uso |
|---|---|
| `ZENITH_URL` | URL do painel (obrigatoria) |
| `ZENITH_USER` / `ZENITH_PASS` | credenciais do painel (obrigatorias) |
| `ZENITH_ROLE` | `cliente` (default) ou `admin` |
| `ZENITH_FRANQUIA` | franquia raiz no login de sub-revenda (opcional) |
| `ZENITH_TIMEOUT_SECS` | timeout do cliente (default 30) |
| `ZENITH_API_TOKEN` | token exigido nas rotas `/v1/*` |
| `ZENITH_BIND` | bind do servidor (default `127.0.0.1:8787`) |
| `ZENITH_AUDIT_DIR` | diretorio da auditoria (default `audit`) |
| `RUST_LOG` | filtro de log (`tracing`) |

Os arquivos de auditoria sao diarios; a retencao e responsabilidade do
operador (o diretorio `audit/` e ignorado pelo Git).

## Limitacoes conhecidas

- `GET /openapi.json` serve **YAML** (conteudo de `docs/openapi.yaml`),
  apesar da extensao `.json`; e o formato pedido no brief. Use
  `Accept: application/yaml` ou um parser YAML.
- O upload multipart (`/v1/upload`) tem limite de 25 MB por arquivo e tambem
  respeita allowlist e `confirm`.
- Nao ha paginacao propria: offset/limit/page seguem o painel.
