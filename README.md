# zenith4k-api

API em **Rust** para operar o painel da franquia **ZENITH4K** (goiptv merger)
de ponta a ponta: autenticar, consultar e editar revendas, clientes, planos,
creditos, pedidos de conteudo, reportes de canal, catalogo, jogos, tickets,
avisos e configuracoes.

O projeto tem tres entregas:

| Pacote | Binario | O que e |
|---|---|---|
| `merger-core` | — | Cliente HTTP: login/sessao, passthrough autenticado e atalhos por dominio |
| `merger-server` | `zenith4k-serve` | API REST local em JSON (documentada em OpenAPI) para um agente de IA consumir |
| `merger-cli` | `zenith4k` | CLI para operacao manual e diagnostico |

> Repositorio privado. Nao commite credenciais. Use variaveis de ambiente
> (ou exporte as linhas de um `.env` local, que e ignorado pelo Git).

## Inicio rapido

Pre-requisitos: Rust stable (edition 2021) e acesso ao painel.

```bash
cp .env.example .env
# edite ZENITH_URL, ZENITH_USER, ZENITH_PASS e ZENITH_API_TOKEN
```

Os dois binarios carregam `.env` automaticamente (sem sobrescrever variaveis
ja definidas no ambiente). Em CI/producao, exporte as variaveis normalmente.

```bash
cargo run -p merger-server --bin zenith4k-serve
```

O servidor sobe em `http://127.0.0.1:8787` e exige
`Authorization: Bearer $ZENITH_API_TOKEN` nas rotas `/v1/*`.

```bash
curl -s http://127.0.0.1:8787/health
# {"ok":true,"data":{"service":"zenith4k-api",...},"meta":{"elapsed_ms":0}}

curl -s -H "Authorization: Bearer $ZENITH_API_TOKEN" \
  http://127.0.0.1:8787/v1/meta

# a primeira chamada de dominio faz login no painel sozinha
curl -s -H "Authorization: Bearer $ZENITH_API_TOKEN" \
  http://127.0.0.1:8787/v1/revendas
```

CLI (fala direto com o painel, sem passar pelo servidor local):

```bash
cargo run -p merger-cli -- whoami
cargo run -p merger-cli -- revendas
cargo run -p merger-cli -- usuarios -q search=maria -q status=ativo
cargo run -p merger-cli -- stats
```

## Docker

O `docker-compose.yml` sobe a API publicada **somente em `127.0.0.1:8787`**
(o bind dentro do container e `0.0.0.0:8787`), com auditoria em `./audit` e
healthcheck TCP via bash.

```bash
cp .env.example .env    # preencha ZENITH_* (o compose usa env_file)
docker compose up -d --build
curl -s http://127.0.0.1:8787/health
```

O runtime da imagem nao traz `curl`; o healthcheck usa
`/dev/tcp/127.0.0.1/8787` (bash do `debian:bookworm-slim`).

Imagem publicada no GHCR pelo workflow `.github/workflows/docker.yml` quando
uma tag `v*` e enviada (ou por `workflow_dispatch`):

```bash
docker pull ghcr.io/ericlesrobsom/zenith4k-api:0.1.0
```

`workflow_dispatch` publica a tag `edge` (build da `main`) para testar antes de
marcar uma versao. O pacote herda a visibilidade do repositorio (privado);
autentique no GHCR antes do pull.

## Validar no painel real

1. Preencha `.env` (`ZENITH_URL`, `ZENITH_USER`, `ZENITH_PASS`).
2. Rode o smoke test **somente leitura**:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\smoke-live.ps1
```

O script faz login, consulta identidade, revendas, usuarios, planos, creditos,
pedidos, reportes, avisos, stats e dashboard. Nao cria, edita nem apaga nada e
nao imprime a senha.

Para testar sem o painel real, suba o painel falso e use as credenciais
`teste`/`teste`:

```bash
node scripts/fake-panel.mjs 18999
# em outro terminal, com ZENITH_URL=http://127.0.0.1:18999
```

## Variaveis de ambiente

| Variavel | Obrigatoria | Default | Uso |
|---|---|---|---|
| `ZENITH_URL` | sim | — | URL do painel (`https://aplicativo.znt4kh.uk`) |
| `ZENITH_USER` | sim | — | usuario da franquia |
| `ZENITH_PASS` | sim | — | senha da franquia |
| `ZENITH_ROLE` | nao | `cliente` | `cliente` (franquia) ou `admin` |
| `ZENITH_FRANQUIA` | nao | — | franquia raiz no login de sub-revenda |
| `ZENITH_TIMEOUT_SECS` | nao | `30` | timeout do cliente HTTP |
| `ZENITH_API_TOKEN` | recomendado | — | token local das rotas `/v1/*` |
| `ZENITH_BIND` | nao | `127.0.0.1:8787` | bind do servidor |
| `ZENITH_AUDIT_DIR` | nao | `audit` | diretorio dos JSONL de auditoria |
| `RUST_LOG` | nao | `info` | filtro de log (`tracing`) |

Sem `ZENITH_API_TOKEN`, o servidor entra em **modo dev**: aceita somente
conexoes loopback e avisa no log. Nao exponha o bind publico nesse modo.

## API local

Envelope consistente:

```json
{"ok": true, "data": {}, "meta": {"elapsed_ms": 12}}
{"ok": false, "error": {"code": "confirmacao_necessaria", "message": "..."}}
```

Rotas principais (lista completa em [`docs/openapi.yaml`](docs/openapi.yaml)):

| Metodo | Rota | Uso |
|---|---|---|
| GET | `/health` | status publico (nao toca no painel) |
| GET | `/openapi.json` | OpenAPI 3.1 (YAML) |
| GET | `/v1/meta` | versao, origem do painel, sessao |
| POST | `/v1/login` | login (com `codigo` para 2FA) |
| POST | `/v1/logout` | encerra a sessao |
| GET | `/v1/whoami` | identidade no painel |
| POST | `/v1/raw` | passthrough com allowlist e `confirm` |
| GET | `/v1/revendas` | lista revendas |
| GET | `/v1/revendas/arvore` | arvore de revendas |
| POST | `/v1/revendas/renovar` | renova revenda (30 dias) |
| POST | `/v1/revendas/nota` | contato/nota |
| POST | `/v1/revendas/unlimited` | promove/rebaixa ilimitado |
| GET | `/v1/usuarios` | busca clientes finais |
| POST | `/v1/usuarios/criar` | cria linha |
| POST | `/v1/usuarios/editar` | edita linha |
| POST | `/v1/usuarios/toggle` | bloqueia/desbloqueia |
| POST | `/v1/usuarios/renovar` | renova com plano |
| POST | `/v1/usuarios/estender` | estende vencimento |
| POST | `/v1/usuarios/delete` | remove (exige `confirm: true`) |
| GET | `/v1/planos`, `/v1/creditos` | planos e saldo |
| GET | `/v1/pedidos`, POST `/v1/pedidos/criar` | pedidos de conteudo |
| GET | `/v1/reportes`, POST `/v1/reportes/criar` | reportes |
| GET | `/v1/catalogo/pendencias`, `/v1/catalogo/jogos` | catalogo |
| GET | `/v1/tickets`, POST `/v1/tickets/criar`, `/v1/tickets/responder` | suporte |
| GET | `/v1/avisos` | notificacoes do painel |
| POST | `/v1/upload` | multipart para rotas de anexo/banner/importacao |
| GET | `/v1/stats`, `/v1/dashboard` | numeros do painel |

### Exemplos com curl

```bash
TOKEN="$ZENITH_API_TOKEN"
BASE=http://127.0.0.1:8787

# login sob demanda (o /v1/revendas ja loga; /v1/login explicito quando 2FA)
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" -d '{}' "$BASE/v1/login"

# buscar clientes
curl -s -G -H "Authorization: Bearer $TOKEN" \
  --data-urlencode "search=maria" --data-urlencode "page=1" \
  "$BASE/v1/usuarios"

# criar linha
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"username":"cliente123","password":"senha-forte","max_connections":1,"dias":30}' \
  "$BASE/v1/usuarios/criar"

# passthrough para qualquer rota da allowlist
curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"method":"GET","path":"/merger/api/stats"}' "$BASE/v1/raw"
```

Operacoes destrutivas exigem `confirm: true` no corpo (ou no RawRequest).
Sem isso a resposta e `409 confirmacao_necessaria`.

## CLI

```bash
cargo run -p merger-cli -- --help
cargo run -p merger-cli -- get /merger/api/stats
cargo run -p merger-cli -- get /merger/api/cliente/sigma_users -q search=maria
cargo run -p merger-cli -- post /merger/api/pedidos/criar --json '{"tmdb_id":603,"titulo":"Matrix"}'
cargo run -p merger-cli -- call --json '{"method":"GET","path":"/merger/api/categorias","query":{"page":1}}'
cargo run -p merger-cli -- login --codigo 123456
```

Revendas (paridade com o menu "Acoes"):

```bash
cargo run -p merger-cli -- revendas
cargo run -p merger-cli -- revendas-criar --json '{"username":"revenda_x","password":"senha-forte","credits":100}' --dias 30
cargo run -p merger-cli -- revendas-editar --json '{"id":7,"transfer":50}'
cargo run -p merger-cli -- revendas-toggle 7 --disable
cargo run -p merger-cli -- revendas-renovar 7
cargo run -p merger-cli -- revendas-nota --json '{"id":7,"contato_nome":"Fulano"}'
cargo run -p merger-cli -- revendas-unlimited --json '{"id":7,"novo_estado":1}'
cargo run -p merger-cli -- revendas-remover 7 --confirm
cargo run -p merger-cli -- revendas-transferir-destinos sub01
cargo run -p merger-cli -- revendas-convite
cargo run -p merger-cli -- revendas-convite-pendentes
```

`post`/`call` aceitam `--file caminho.json` no lugar de `--json`, e `--confirm`
para liberar operacoes destrutivas. A saida e JSON pretty no stdout; logs vao
para o stderr.

## Seguranca

- O agente consome a API local; a senha do painel fica somente no servidor.
- Token local comparado em tempo constante; sem token, so loopback.
- Allowlist de caminho no passthrough (bloqueia URL absoluta e `..`).
- `confirm: true` obrigatorio para DELETE e rotas destrutivas, conferido
  **antes** de qualquer chamada ao painel.
- Auditoria em `./audit/api-AAAA-MM-DD.jsonl` grava apenas
  `{ts,method,path,status,elapsed_ms}` — nunca corpo, query ou cabecalhos.
- CORS liberado (API local), mas o token continua obrigatorio.

Detalhes em [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

## Testes e checks

Tudo offline (nenhum teste chama o painel):

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

O CI (`.github/workflows/ci.yml`) roda os tres em `ubuntu-latest`.

## Documentacao

- [`docs/AI-AGENT.md`](docs/AI-AGENT.md) — guia do agente: ferramentas,
  exemplos, fluxo de erro e regras de ouro.
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — camadas, auth, envelope,
  seguranca e operacao.
- [`docs/openapi.yaml`](docs/openapi.yaml) — contrato OpenAPI 3.1.
- [`docs/ENDPOINTS.md`](docs/ENDPOINTS.md) — catalogo completo de rotas do
  painel (817 rotas).
- [`docs/AUTH.md`](docs/AUTH.md) — fluxo de login/2FA do painel.
- [`docs/CAPABILITIES-REVENDAS.md`](docs/CAPABILITIES-REVENDAS.md) e
  [`docs/CAPABILITIES-CONTEUDO.md`](docs/CAPABILITIES-CONTEUDO.md) — operacoes
  por dominio com exemplos.
