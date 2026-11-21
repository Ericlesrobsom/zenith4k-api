# Brief — Onyx (servidor REST, CLI e documentacao)

Voce e **Onyx**, engenheiro Rust. Trabalhe **somente** neste repositorio:

`C:\Users\ericl\Desktop\zenith4k-api`

## Contexto

Painel alvo: **goiptv merger** (franquia ZENITH4K). O crate `merger-core` ja
tem o transporte e os atalhos por dominio (alguns ainda em stub — compile
assim mesmo). Seu trabalho e entregar a **API REST local para um agente de IA**
e a CLI, com documentacao.

**Nao acesse a rede, nao use credenciais, nao chame o painel vivo.**
Build e testes devem ser offline.

Leia:

- `crates/merger-core/src/{lib,config,error,client,raw,auth}.rs`
- `crates/merger-core/src/api/*.rs` (assinaturas disponiveis)
- `docs/ENDPOINTS.md`, `docs/live-shapes.json`, `README.md`
- Fonte Go (contratos): `C:\Users\ericl\Desktop\goiptv`

## Escopo (arquivos que voce pode editar)

- `crates/merger-server/**` (Cargo.toml + src/**, pode criar modulos)
- `crates/merger-cli/**`
- `docs/ARCHITECTURE.md`, `docs/AI-AGENT.md`, `docs/openapi.yaml` (novos)
- `README.md`
- `.github/workflows/ci.yml` (novo)
- `crates/merger-server/tests/**` (novo)

**Nao edite** `crates/merger-core/**` (se faltar um metodo, use
`client.raw(...)` no handler e anote a pendencia no relatorio).

## Servidor (`zenith4k-serve`)

- Estado: `Arc<MergerClient>` criado a partir de `ClientConfig::from_env()`,
  mas o login pode ser adiado ate a primeira chamada (`ensure_login`).
- Bind: `ZENITH_BIND` (default `127.0.0.1:8787`).
- Auth local: `Authorization: Bearer $ZENITH_API_TOKEN`. Sem token configurado,
  aceitar somente loopback e avisar no log (modo dev).
- Envelope consistente:
  - sucesso: `{"ok":true,"data":<body>,"meta":{"elapsed_ms":N}}`
  - erro: `{"ok":false,"error":{"code":"...","message":"..."}}`
- Mapeamento de `ApiError` -> HTTP:
  - `NotAuthenticated`/`SessionExpired` 401
  - `TwoFactorRequired` 428
  - `InvalidTwoFactor` 401
  - `Forbidden` 403
  - `NotFound` 404
  - `ConfirmationRequired` 409
  - `RateLimited` 429
  - `PathNotAllowed` 400
  - `Panel` 502
  - resto 500
- Endpoints minimos:
  - `GET /health` (publico)
  - `GET /v1/meta` (versao, origem, autenticado, sessao)
  - `POST /v1/login` (`{codigo?}`), `POST /v1/logout`, `GET /v1/whoami`
  - `POST /v1/raw` (`{method,path,query,body,confirm}`)
  - Tipados: `/v1/revendas`, `/v1/revendas/arvore`, `/v1/revendas/renovar`,
    `/v1/revendas/nota`, `/v1/revendas/unlimited`, `/v1/usuarios`,
    `/v1/usuarios/criar`, `/v1/usuarios/editar`, `/v1/usuarios/toggle`,
    `/v1/usuarios/renovar`, `/v1/usuarios/estender`, `/v1/usuarios/delete`,
    `/v1/planos`, `/v1/creditos`, `/v1/pedidos`, `/v1/pedidos/criar`,
    `/v1/reportes`, `/v1/reportes/criar`, `/v1/catalogo/pendencias`,
    `/v1/catalogo/jogos`, `/v1/tickets`, `/v1/avisos`, `/v1/stats`,
    `/v1/dashboard`
  - `GET /openapi.json` servindo `docs/openapi.yaml` via `include_str!`
- Auditoria local: JSONL em `./audit/api-AAAA-MM-DD.jsonl` com
  `{ts,method,path,status,elapsed_ms}` — **nunca** logar corpo (pode conter
  senha/token). Falha de auditoria nao derruba a request.
- CORS: permitir qualquer origem (API local, token obrigatorio).

## CLI (`zenith4k`)

Comandos com `clap`:

- `whoami`
- `get <path> [-q chave=valor ...]`
- `post <path> --json '<json>'` (usar `--json` ou arquivo)
- `call --json '<RawRequest>'` (passthrough completo, com `confirm`)
- `revendas`, `usuarios [-q ...]`, `pedidos`, `reportes`, `catalogo`,
  `tickets`, `avisos`, `stats`, `dashboard` (saida JSON pretty)
- `login --codigo <codigo>` (quando 2FA)

## Documentacao

- `docs/ARCHITECTURE.md`: camadas, fluxo de auth, decisao de passthrough x
  tipado, seguranca (confirm, allowlist, auditoria).
- `docs/AI-AGENT.md`: como um agente descobre e chama a API — lista de
  ferramentas, exemplos `curl`, fluxo de erro 401/428/409, regras de ouro
  (nunca apagar sem confirmacao), payloads.
- `docs/openapi.yaml`: OpenAPI 3.1 valido cobrindo `/health` e `/v1/*`.
- `README.md`: quickstart real, variaveis, exemplos de `curl`.
- CI: ubuntu-latest, `cargo fmt --check`, `cargo clippy -D warnings`,
  `cargo test --workspace`.

## Testes

- `cargo check --workspace`, `cargo test --workspace` offline.
- Teste de integracao do servidor com `tower::ServiceExt` ou subir em porta
  efemera e chamar `/health` e `/v1/meta` (sem painel real: `/health` e
  `/v1/meta` nao devem exigir login).
- Nao use `unwrap()` em handler.

## Regras

- Codigo e comentarios em portugues (sem acentos nos identificadores).
- Nao adicione dependencias alem das do workspace; se precisar, justifique no
  relatorio e adicione ao `workspace.dependencies`.

## Ao terminar

Rode os checks e reporte com:

`maestri ask "OpenCode" "Onyx: <resumo; arquivos; testes; pendencias>"`
