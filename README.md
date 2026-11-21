# zenith4k-api

API em **Rust** para operar o painel da franquia **ZENITH4K** (goiptv merger) de
ponta a ponta: autenticar, consultar e editar revendas, clientes, planos,
creditos, pedidos de conteudo, reportes de canal, catalogo, jogos, tickets,
avisos e configuracoes.

O projeto tem tres entregas:

| Pacote | O que e |
|---|---|
| `merger-core` | Cliente HTTP: login/sessao, passthrough autenticado e atalhos por dominio |
| `zenith4k-serve` | API REST local em JSON (documentada em OpenAPI) para um agente de IA consumir |
| `zenith4k` | CLI para operacao manual e diagnostico |

> Repositorio privado. Nao commite credenciais. Use `.env` (ignorado pelo Git)
> ou variaveis de ambiente.

## Inicio rapido

```bash
cp .env.example .env
# edite ZENITH_PASS e ZENITH_API_TOKEN

cargo run -p merger-cli -- whoami
cargo run -p merger-server
```

O servidor sobe em `http://127.0.0.1:8787` e exige
`Authorization: Bearer $ZENITH_API_TOKEN` nas rotas `/v1/*`.

## Camadas

1. **Atalhos tipados** (`client.revendas_list()`, `client.usuarios_criar(...)`)
   para os fluxos do dia a dia.
2. **Passthrough cru** `client.raw(RawRequest)` — qualquer rota
   `/merger/api/*` ou `/api/webhook/*`, com allowlist de caminho, cookie jar e
   trava `confirm=true` para operacoes destrutivas.
3. **REST local** — o agente de IA chama a API local; o servidor injeta a sessao
   do painel. Nenhuma credencial do painel precisa ser exposta ao agente.

## Documentacao

- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- [`docs/AUTH.md`](docs/AUTH.md)
- [`docs/ENDPOINTS.md`](docs/ENDPOINTS.md) — catalogo completo de rotas
- [`docs/CAPABILITIES.md`](docs/CAPABILITIES.md) — o que da para fazer, com exemplos
- [`docs/AI-AGENT.md`](docs/AI-AGENT.md) — como um agente consome a API
- [`docs/openapi.yaml`](docs/openapi.yaml)
