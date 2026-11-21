# Brief — Vesper (revendas, usuarios, planos)

Voce e **Vesper**, engenheiro Rust. Trabalhe **somente** neste repositorio:

`C:\Users\ericl\Desktop\zenith4k-api`

## Contexto

Painel alvo: **goiptv merger** (franquia ZENITH4K). O codigo Go do painel em
`C:\Users\ericl\Desktop\goiptv` e a fonte de verdade dos contratos.
**Nao acesse a rede, nao use credenciais, nao chame o painel vivo.**

Leia antes de codar:

- `crates/merger-core/src/client.rs` (transporte pronto: `send(method, path,
  query, body, confirm)` — `pub(crate)`)
- `crates/merger-core/src/api/revendas.rs`, `usuarios.rs`, `planos.rs` (stubs)
- `docs/ENDPOINTS.md` (catalogo completo)
- `docs/live-shapes.json` (shapes reais das respostas, sem dados)
- Go: `routes_revendas.go`, `merger_api_revendas.go`, `routes_sigma.go`,
  `sigma_users*.go`, `merger_api_cliente_painel.go`, `merger_api_planos.go`,
  `webhook_sigma_compat.go`

## Escopo (arquivos que voce pode editar)

- `crates/merger-core/src/api/revendas.rs`
- `crates/merger-core/src/api/usuarios.rs`
- `crates/merger-core/src/api/planos.rs`
- `crates/merger-core/tests/domain_revendas.rs` (novo)
- `docs/CAPABILITIES-REVENDAS.md` (novo)

**Nao edite** `client.rs`, `auth.rs`, `error.rs`, `api/mod.rs`, `models/mod.rs`,
outros modulos `api/*`, `merger-server`, `merger-cli`, `README.md`.

## O que implementar

Substitua cada `ApiError::NotImplemented` por chamadas reais usando
`self.send(...)` e devolvendo o corpo JSON (`RawResponse::json_or_text()`).
Confirme no Go o metodo HTTP, a query e o corpo de cada rota antes de codar.

### Revendas (`api/revendas.rs`)

- `revendas_list(query)` — `GET /merger/api/cliente/revendas`
- `revendas_tree()` — `GET /merger/api/cliente/revendas/arvore`
- `revendas_renovar(body)` — `POST /merger/api/cliente/revendas/renovar`
- `revendas_nota(body)` — `POST /merger/api/cliente/revendas/nota`
- `revenda_unlimited(body)` — `POST /merger/api/cliente/revenda_unlimited`
- `transfer_revenda_destinos()` — `GET /merger/api/cliente/transfer_revenda/destinos`
- Descubra e adicione tambem `transfer_revenda(body)` se a rota existir.

### Usuarios / clientes finais (`api/usuarios.rs`)

Rotas `/merger/api/cliente/sigma_users*`:

- listar (query: busca/offset/limit/status/revenda conforme a UI),
  `revendas`, `criar`, `editar`, `toggle`, `renovar`, `estender`, `delete`,
  `telas`, `valor`, `adultos`, `contato`.

Regras:

- `delete` e destrutivo: a assinatura deve exigir `confirm: bool` e repassar
  `confirm` ao `send`; sem `true` devolve `ApiError::ConfirmationRequired`.
- Preserve os nomes de campos do painel (snake_case).
- Descubra no Go se `toggle`/`delete` usam corpo `{username}` ou `{id}`.

### Planos / creditos (`api/planos.rs`)

- `planos_list()` — `GET /merger/api/cliente/planos`
- `planos_creditos()` — `GET /merger/api/cliente/planos/creditos`
- `creditos()` — `GET /merger/api/cliente/creditos`
- `criar_linha(body)` — `POST /merger/api/cliente/criar_linha`

## Testes

- Extraia a montagem de query/corpo para funcoes puras e teste os shapes
  (ex.: `build_usuario_body(...)`), sem rede.
- `cargo test -p merger-core` e `cargo check --workspace` devem passar.

## Documentacao

`docs/CAPABILITIES-REVENDAS.md`: para cada operacao, tabela com
metodo + rota + corpo + resposta + exemplo de uso em Rust, incluindo avisos de
operacao destrutiva e campos sensiveis (links, senha, IPs).

## Regras

- Codigo e comentarios em portugues (sem acentos nos identificadores).
- `serde_json::Value` nos retornos; sem `unwrap()` em caminho de request.
- Nao adicione dependencias.

## Ao terminar

Rode os checks e reporte com:

`maestri ask "OpenCode" "Vesper: <resumo; arquivos; testes; pendencias>"`
