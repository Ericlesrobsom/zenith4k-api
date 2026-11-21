# Brief — Kestrel (conteudo, pedidos, reportes e suporte)

Voce e **Kestrel**, engenheiro Rust. Trabalhe **somente** neste repositorio:

`C:\Users\ericl\Desktop\zenith4k-api`

## Contexto

Painel alvo: **goiptv merger** (franquia ZENITH4K). Fonte de verdade dos
contratos: `C:\Users\ericl\Desktop\goiptv` (Go).
**Nao acesse a rede, nao use credenciais, nao chame o painel vivo.**

Leia antes de codar:

- `crates/merger-core/src/client.rs` (`send(method, path, query, body, confirm)`
  e `pub(crate)`)
- Stubs: `crates/merger-core/src/api/{pedidos,reportes,catalogo,tickets,avisos}.rs`
- `docs/ENDPOINTS.md` e `docs/live-shapes.json`
- Go: `routes_revendas.go`, `merger_api_pedidos.go`, `merger_api_reportes.go`,
  `routes_catalogo.go`, `merger_api_catalogo.go`, `routes_tickets.go`,
  `routes_loja_pwa.go`, `routes_merger_core.go` (agenda/jogos)

## Escopo (arquivos que voce pode editar)

- `crates/merger-core/src/api/pedidos.rs`
- `crates/merger-core/src/api/reportes.rs`
- `crates/merger-core/src/api/catalogo.rs`
- `crates/merger-core/src/api/tickets.rs`
- `crates/merger-core/src/api/avisos.rs`
- `crates/merger-core/tests/domain_conteudo.rs` (novo)
- `docs/CAPABILITIES-CONTEUDO.md` (novo)

**Nao edite** `client.rs`, `auth.rs`, `error.rs`, `api/mod.rs`, `models/mod.rs`,
`api/revendas.rs`, `api/usuarios.rs`, `api/planos.rs`, `merger-server`,
`merger-cli`, `README.md`.

## O que implementar

Substitua cada `ApiError::NotImplemented` por chamadas reais usando
`self.send(...)` e devolvendo o corpo JSON. Confirme metodo/query/corpo no Go.

### Pedidos de conteudo (`api/pedidos.rs`)

- `GET /merger/api/pedidos/listar`
- `POST /merger/api/pedidos/criar` (filme, serie, canal, correcao)
- `POST /merger/api/pedidos/status`
- `POST /merger/api/pedidos/creditos/transferir`

Descubra no Go os tipos aceitos (`tipo`, `nome`, `canal`, `episodios`,
prioridade etc.) e deixe isso documentado no rustdoc.

### Reportes de canal/filme (`api/reportes.rs`)

- `GET /merger/api/reportes/listar`
- `POST /merger/api/reportes/criar`
- `POST /merger/api/reportes/status`

### Catalogo (`api/catalogo.rs`)

- `GET /merger/api/categorias`
- `GET /merger/api/cliente/cat/canais`
- Pendencias: `GET /merger/api/cliente/catalogo/pendencias`,
  `GET .../pendencias/opcoes`, `GET .../pendencias/previa`,
  `POST .../pendencias/revisar`, `POST .../pendencias/corrigir`
- `GET /merger/api/cliente/agenda/jogos` (jogos do dia)
- `GET /merger/api/stats`
- `GET /merger/api/dashboard_financeiro`

### Tickets (`api/tickets.rs`)

Leia `routes_tickets.go` — as rotas de cliente diferem das de admin. Implemente
listar/ver/criar/responder/status (e anexo se fizer sentido; upload pode ficar
como pendencia documentada).

### Avisos (`api/avisos.rs`)

- `GET /merger/api/avisos`
- `POST /merger/api/avisos/lida`

## Testes

- Funcoes puras de montagem de query/corpo com testes offline.
- `cargo test -p merger-core` e `cargo check --workspace` devem passar.

## Documentacao

`docs/CAPABILITIES-CONTEUDO.md`: tabela por operacao (metodo, rota, corpo,
resposta, exemplo Rust) e uma secao "pedidos de filme/canal" com exemplos
concretos de criacao e consulta de status.

## Regras

- Codigo e comentarios em portugues (sem acentos nos identificadores).
- `serde_json::Value` nos retornos; sem `unwrap()` em request.
- Nao adicione dependencias.

## Ao terminar

Rode os checks e reporte com:

`maestri ask "OpenCode" "Kestrel: <resumo; arquivos; testes; pendencias>"`
