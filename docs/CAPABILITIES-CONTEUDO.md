# Capacidades — Conteúdo (pedidos, reportes, catálogo, tickets e avisos)

Este documento descreve os atalhos tipados de `merger-core` para o conteúdo do
painel **goiptv merger** usado pela franquia ZENITH4K. Todos os contratos foram
conferidos no código Go do painel (`C:\Users\ericl\Desktop\goiptv`) — nenhuma
chamada ao painel vivo foi feita para produzi-lo.

Arquivos cobertos:

| Módulo Rust | Fonte Go principal |
|---|---|
| `api/pedidos.rs` | `merger_api_pedidos.go`, `routes_revendas.go` |
| `api/reportes.rs` | `merger_api_reportes.go`, `routes_revendas.go` |
| `api/catalogo.rs` | `merger_api_categorias.go`, `cliente_canal_cat.go`, `catalogo_pendencias_api.go`, `merger_api_sync.go`, `merger_api_dashboard.go` |
| `api/tickets.rs` | `routes_tickets.go`, `tickets_suporte.go`, `tickets_admin.go` |
| `api/avisos.rs` | `routes_loja_pwa.go`, `avisos_handlers.go` |

## Convenções

- Todos os métodos devolvem `serde_json::Value` com o corpo JSON do painel.
- O painel costuma responder **HTTP 200** mesmo em erro de negócio:
  `{"ok": false, "erro": "..."}`. Confira `ok` antes de assumir sucesso.
- Erros de validação local (antes do request) usam `ApiError::Config` com uma
  mensagem em português. Eles evitam requests que o painel recusaria.
- As rotas abaixo não são destrutivas pelo allowlist de `client.rs`, então
  `confirm=true` não é necessário.
- Sessão: rotas `cliente/*` exigem login de franquia (`cliente_login`); rotas
  `admin/*` exigem login admin/equipe. O painel aplica feature gates
  (`pedidos_conteudo`, `reportes`, `tickets`, `dashboard` etc.) e pode
  responder 403 quando a aba está oculta para a franquia.

## 1. Pedidos de conteúdo

Rotas (fonte: `routes_revendas.go` linhas 94–99):

| Operação | Método/rota | Corpo | Resposta | Método Rust |
|---|---|---|---|---|
| Listar | `GET /merger/api/pedidos/listar` | — | `{"ok":true,"pedidos":[...],"creditos_pedidos":N}` | `pedidos_listar(&[])` |
| Criar | `POST /merger/api/pedidos/criar` | `tmdb_id`, `tipo`, `titulo`, `poster_url`, `audio`, `link_m3u`, `episodios_selecionados` | `{"ok":true,"creditos_pedidos":N}` | `pedidos_criar(body)` |
| Status (admin) | `POST /merger/api/pedidos/status` | `id`, `status`, `nota` | `{"ok":true}` | `pedidos_status(body)` |
| Transferir créditos | `POST /merger/api/pedidos/creditos/transferir` | `revenda_id`, `quantidade` | `{"ok":true,"creditos_pedidos":N}` | `pedidos_creditos_transferir(body)` |

### Tipos aceitos pelo painel

O handler `mergerAPIPedidosCriar` (`merger_api_pedidos.go`) só conhece estes
campos:

| Campo | Tipo | Regra |
|---|---|---|
| `tmdb_id` | inteiro | obrigatório, maior que zero |
| `titulo` | string | obrigatório, não vazio |
| `tipo` | string | `movie` ou `series`; qualquer outro valor vira `movie` |
| `audio` | string | `dublado`, `legendado` ou `ambos`; qualquer outro vira `dublado` |
| `poster_url` | string | opcional |
| `link_m3u` | string | opcional |
| `episodios_selecionados` | array de strings | opcional; usado em séries |

> **Não existe** `tipo: "canal"`, `tipo: "correcao"`, `nome`, `episodios` ou
> `prioridade` nessa rota. O painel ignora campos desconhecidos. Para canal ou
> correção de conteúdo, use **reportes** (seção 2).

Regras de negócio do painel:

- Cada pedido consome **1 crédito de pedido**, independentemente da quantidade
  de episódios.
- Um segundo pedido `pendente` para o mesmo `tmdb_id` do mesmo usuário é
  recusado (`Você já tem um pedido pendente para este conteúdo`).
- Ao rejeitar (`status: "rejeitado"`) um pedido que estava `pendente`, o painel
  devolve 1 crédito ao autor.
- A listagem não tem filtros por querystring: admin vê tudo, franquia vê a
  própria subárvore (`coletarDescendentes`). `creditos_pedidos` só é preenchido
  para sessão `cliente`.

### Exemplos — filme

```rust
use serde_json::json;

// Cria um pedido de filme (1 crédito).
let resposta = client
    .pedidos_criar(json!({
        "tmdb_id": 550,
        "titulo": "Clube da Luta",
        "tipo": "movie",
        "audio": "dublado",
        "poster_url": "https://image.tmdb.org/t/p/w500/poster.jpg"
    }))
    .await?;

// {"ok": true, "creditos_pedidos": 4}
println!("{resposta}");
```

### Exemplos — série com episódios

```rust
let resposta = client
    .pedidos_criar(json!({
        "tmdb_id": 1399,
        "titulo": "Game of Thrones",
        "tipo": "series",
        "audio": "ambos",
        "episodios_selecionados": ["S01E01", "S01E02", "S01E03"]
    }))
    .await?;
```

### Consultar e atualizar status

A rota de listagem não aceita filtro de status; filtre no cliente:

```rust
let lista = client.pedidos_listar(&[]).await?;
let pendentes: Vec<_> = lista["pedidos"]
    .as_array()
    .into_iter()
    .flatten()
    .filter(|p| p["status"] == "pendente")
    .collect();

if let Some(pedido) = pendentes.first() {
    let id = pedido["id"].as_i64().unwrap();
    // Rota de admin: pendente -> adicionado (ou rejeitado, que estorna 1 crédito).
    client
        .pedidos_status(json!({
            "id": id,
            "status": "adicionado",
            "nota": "Adicionado no lote 42"
        }))
        .await?;
}
```

### Transferir créditos de pedido

```rust
let resposta = client
    .pedidos_creditos_transferir(json!({
        "revenda_id": 123, // sub-revenda DIRETA do solicitante
        "quantidade": 5
    }))
    .await?;

// {"ok": true, "creditos_pedidos": <novo saldo do remetente>}
```

## 2. Reportes de conteúdo

Rotas (fonte: `routes_revendas.go` linhas 101–104):

| Operação | Método/rota | Corpo | Resposta | Método Rust |
|---|---|---|---|---|
| Listar | `GET /merger/api/reportes/listar` | — | `{"ok":true,"reportes":[...]}` | `reportes_listar(&[])` |
| Criar | `POST /merger/api/reportes/criar` | `fonte_id`, `stream_id_original`, `tipo`, `nome_conteudo`, `motivo`, `episodios_afetados` | `{"ok":true}` | `reportes_criar(body)` |
| Status (admin) | `POST /merger/api/reportes/status` | `id`, `status`, `nota` | `{"ok":true}` | `reportes_status(body)` |

- `tipo`: `live` (default), `movie` ou `series`.
- `stream_id_original` e `nome_conteudo` são obrigatórios.
- `fonte_id` é inteiro (default 0).
- `episodios_afetados` é string livre (usada em séries).
- Motivo é validado contra uma whitelist **por tipo** (strings exatas, com
  acento). O painel também aplica anti-spam de duplicata e teto diário.

### Whitelist de motivos

`live`:

- `O canal está travando`
- `O canal está sem áudio`
- `O canal não da imagem`
- `Link off/quebrado`
- `O link está trocado`

`movie`:

- `O filme não carrega`
- `O filme está sem áudio`
- `O filme não da imagem`
- `Link off/quebrado`
- `O link está trocado`

`series`:

- `O episódio não corresponde ao que está no link`
- `O episódio não funciona`
- `O episódio esta sem áudio`
- `O episódio não tem imagens`
- `Série completa off`
- `O link está trocado`

### Exemplo — canal fora do ar

```rust
let resposta = client
    .reportes_criar(json!({
        "fonte_id": 3,
        "stream_id_original": "1024",
        "tipo": "live",
        "nome_conteudo": "ESPN",
        "motivo": "O canal está travando",
        "episodios_afetados": ""
    }))
    .await?;

// {"ok": true}
```

### Exemplo — corrigir reporte (admin)

```rust
client
    .reportes_status(json!({
        "id": 88,
        "status": "corrigido",
        "nota": "Link substituído"
    }))
    .await?;
```

## 3. Catálogo, pendências, jogos, stats e dashboard

| Operação | Método/rota | Query/corpo | Resposta | Método Rust |
|---|---|---|---|---|
| Categorias | `GET /merger/api/categorias` | `tipo`, `servidor` | array de `{virtual_id,nome,ordem,total,bloqueada,...}` | `categorias()` / `categorias_filtradas(query)` |
| Canais da categoria | `GET /merger/api/cliente/cat/canais` | `vid`, `tipo` | array de `{fonte_id,stream_id_original,nome,orfao}` | `cat_canais(query)` |
| Pendências | `GET /merger/api/cliente/catalogo/pendencias` | — | `{"ok":true,"pendencias":[...],"total":N}` | `catalogo_pendencias(&[])` |
| Revisar | `GET .../pendencias/revisar` | `id` | `{"ok":true,"pendencia":{...}}` | `catalogo_pendencia_revisar(json!({"id":N}))` |
| Opções | `GET .../pendencias/opcoes` | `id`, `q` | `{"ok":true,"opcoes":[...],"mais":bool}` | `catalogo_pendencia_opcoes(query)` |
| Prévia | `POST .../pendencias/previa` | `id`, `revisao`, `acao`, `destino_id`, `fonte_id` | `{"ok":true,"token":"...","alteracoes":[],"conflitos":[]}` | `catalogo_pendencia_previa(body)` |
| Corrigir | `POST .../pendencias/corrigir` | `token` | `{"ok":true,"resolvidas":N,"pendentes":M}` | `catalogo_pendencia_corrigir(body)` |
| Jogos do dia | `GET /merger/api/cliente/agenda/jogos` | — | array da agenda (`time1`,`time2`,`horario`,...) | `jogos_do_dia(&[])` |
| Stats | `GET /merger/api/stats` | — | `{canais,filmes,series,bloqueados,fontes,...}` | `stats()` |
| Dashboard | `GET /merger/api/dashboard_financeiro` | `offset` | objeto do dashboard (séries, cards, ranking) | `dashboard_financeiro()` / `dashboard_financeiro_com_offset(n)` |

Notas de contrato:

- `categorias` aceita `tipo` (`live` default) e `servidor` (0/ausente =
  Principal). O helper `consulta_categorias` valida o tipo.
- `cat/canais` usa `vid` (obrigatório) e `tipo` (default `live`).
- Pendências exigem sessão **cliente titular** (franquia raiz); sub-revenda
  recebe 403 `apenas_franquia`.
- A prévia devolve um **token válido por 10 minutos**, vinculado ao usuário,
  ao cadastro e ao estado conferido. `corrigir` consome o token; token expirado
  devolve 409 `previa_obsoleta`.
- Ações de prévia: `substituir` (exige `destino_id`) ou `remover`.
- `dashboard_financeiro` aceita `offset` em blocos de 30 dias, limitado a 24
  pelo painel. O helper aplica o mesmo clamp.
- `jogos_do_dia` é um proxy: o painel busca a agenda esportiva externa e
  devolve o JSON reescrito com URLs de imagem internas. A rota não tem filtros
  documentados.

### Exemplo — listar categorias de filmes

```rust
use merger_core::api::catalogo::consulta_categorias;

let query = consulta_categorias(Some("movie"), None)?;
let categorias = client.categorias_filtradas(&query).await?;

for categoria in categorias.as_array().into_iter().flatten() {
    println!(
        "{} ({} itens)",
        categoria["nome"].as_str().unwrap_or("?"),
        categoria["total"].as_u64().unwrap_or(0)
    );
}
```

### Exemplo — fluxo completo de uma pendência

```rust
use merger_core::api::catalogo::{consulta_pendencia_opcoes, normalizar_pendencia_previa};
use serde_json::json;

// 1) Lista e escolhe uma pendência.
let lista = client.catalogo_pendencias(&[]).await?;
let pendencia = &lista["pendencias"][0];
let id = pendencia["id"].as_i64().unwrap();
let revisao = pendencia["revisao"].as_str().unwrap_or("").to_string();

// 2) Revisa (detalhe) e busca opções de destino.
let detalhe = client
    .catalogo_pendencia_revisar(json!({ "id": id }))
    .await?;
let opcoes = client
    .catalogo_pendencia_opcoes(&consulta_pendencia_opcoes(id, Some("globo"))?)
    .await?;

// 3) Prévia (não altera dados) — devolve token temporário.
let previa = client
    .catalogo_pendencia_previa(json!({
        "id": id,
        "revisao": revisao,
        "acao": "substituir",
        "destino_id": opcoes["opcoes"][0]["id"].clone(),
        "fonte_id": 0
    }))
    .await?;

// 4) Aplica a correção usando o token.
let token = previa["token"].as_str().unwrap();
let resultado = client
    .catalogo_pendencia_corrigir(json!({ "token": token }))
    .await?;
```

### Exemplo — stats e dashboard

```rust
let stats = client.stats().await?;
println!(
    "canais={} filmes={} series={}",
    stats["canais"], stats["filmes"], stats["series"]
);

// Janela atual.
let atual = client.dashboard_financeiro().await?;

// Duas janelas de 30 dias atrás (o painel clampa em 24).
let anterior = client.dashboard_financeiro_com_offset(2).await?;
```

## 4. Tickets de suporte

Rotas (fonte: `routes_tickets.go`):

| Operação | Método/rota | Corpo/query | Resposta | Método Rust |
|---|---|---|---|---|
| Listar (cliente) | `GET /merger/api/cliente/tickets/listar` | — | `{"ok":true,"tickets":[...]}` | `tickets_listar(&[])` |
| Ver (cliente) | `GET /merger/api/cliente/tickets/ver` | `id` | `{"ok":true,"ticket":{...,"mensagens":[]}}` | `tickets_ver(&consulta_ticket_id(id)?)` |
| Criar (cliente) | `POST /merger/api/cliente/tickets/criar` | multipart: `assunto`, `texto`, `imagem?` | `{"ok":true,"id":N}` | `tickets_criar(body, imagem?)` |
| Responder (cliente) | `POST /merger/api/cliente/tickets/responder` | multipart: `id`, `texto`, `imagem?` | `{"ok":true}` | `tickets_responder(body, imagem?)` |
| Fechar (cliente) | `POST /merger/api/cliente/tickets/fechar` | `id` | `{"ok":true}` | `tickets_fechar(id)` |
| Listar (admin) | `GET /merger/api/admin/tickets/listar` | `status` | `{"ok":true,"tickets":[...],"aguardando":N}` | `tickets_listar_admin(query)` |
| Ver (admin) | `GET /merger/api/admin/tickets/ver` | `id` | `{"ok":true,"ticket":{...}}` | `tickets_ver_admin(query)` |
| Responder (admin) | `POST /merger/api/admin/tickets/responder` | multipart: `id`, `texto`, `imagem?` | `{"ok":true}` | `tickets_responder_admin(body, imagem?)` |
| Status (admin) | `POST /merger/api/admin/tickets/status` | `id`, `status` | `{"ok":true}` | `tickets_status(body)` |

Regras do painel:

- Ticket é privado do **autor** e do admin; a franquia não vê o ticket da
  sub-revenda.
- Status: `aberto` → `respondido` (admin responde) → `fechado`. Responder como
  cliente reabre um ticket fechado; responder como admin marca `respondido`.
- Limites: 5 chamados não fechados por autor, assunto de 120 caracteres, texto
  de 4000, até 20 anexos por ticket.
- O anexo é reprocessado como JPEG no servidor e servido autenticado por
  `GET /merger/api/tickets/anexo?msg=N` (nunca é estático).

### Criação/resposta multipart

As rotas `criar` e `responder` (cliente e admin) usam `multipart/form-data`.
O handler chama `ParseMultipartForm` **mesmo quando não há imagem**; um POST
JSON é recusado com `upload_invalido`. Por isso o cliente usa
`MergerClient::send_multipart`:

```rust
use merger_core::raw::UploadFile;

let imagem = std::fs::read("print.jpg")?;
let anexo = UploadFile::new("imagem", "print.jpg", imagem).mime("image/jpeg");

client
    .tickets_criar(
        serde_json::json!({ "assunto": "Canal fora do ar", "texto": "Desde 20h" }),
        Some(anexo),
    )
    .await?;
```

- `tickets_criar(body, imagem?)` e `tickets_responder(body, imagem?)` enviam o
  formulário de verdade; `imagem` é opcional.
- `POST /v1/tickets/criar` e `/v1/tickets/responder` expõem o fluxo JSON (sem
  anexo) para o agente de IA.
- `POST /v1/upload` encaminha **qualquer** multipart para o painel (banner,
  logo, anexo, importação).
- `GET /merger/api/tickets/anexo` continua sendo um GET binário: use
  `MergerClient::raw` e trate `RawResponse.text` (o JPEG não é decodificado
  como JSON).

### Exemplo — listar e fechar um chamado

```rust
use merger_core::api::tickets::{consulta_ticket_id, consulta_tickets_admin};
use serde_json::json;

// Cliente: lista os próprios chamados.
let meus = client.tickets_listar(&[]).await?;

// Abre a thread (marca como lido).
let id = meus["tickets"][0]["id"].as_i64().unwrap();
let thread = client.tickets_ver(&consulta_ticket_id(id)?).await?;

// Fecha o chamado.
client.tickets_fechar(id).await?;

// Admin: fila de abertos e mudança de status.
let fila = client
    .tickets_listar_admin(&consulta_tickets_admin(Some("aberto")))
    .await?;
client
    .tickets_status(json!({ "id": id, "status": "fechado" }))
    .await?;
```

## 5. Avisos

| Operação | Método/rota | Corpo/query | Resposta | Método Rust |
|---|---|---|---|---|
| Feed | `GET /merger/api/avisos` | `limit` (1–100) | `{"avisos":[...],"nao_lidos":N,"visao":"cliente"|"admin"}` | `avisos(query)` |
| Marcar lido | `POST /merger/api/avisos/lida` | `{"id":N}` ou `{"todos":true}` | `{"ok":true,"nao_lidos":N}` | `aviso_lida(body)` |

- O feed **nunca** é gateado; receber aviso é canal.
- `limit` fora de 1–100 cai no default 20. `consulta_avisos` omite o parâmetro
  inválido e deixa o painel aplicar o default.
- Na visão de gestão (`visao: "admin"`), a lista é completa e `nao_lidos` é
  sempre 0 (gestão não tem caixa de entrada própria).
- Em `lida`, se `todos: true` vier junto com `id`, `todos` vence — mesma
  precedência do Go.

### Exemplo

```rust
use merger_core::api::avisos::consulta_avisos;
use serde_json::json;

let feed = client.avisos(&consulta_avisos(Some(50))).await?;
println!("não lidos: {}", feed["nao_lidos"]);

if let Some(aviso) = feed["avisos"].as_array().and_then(|a| a.first()) {
    let id = aviso["id"].as_i64().unwrap();
    client.aviso_lida(json!({ "id": id })).await?;
}

// Zera o contador de tudo que a sessão enxerga.
client.aviso_lida(json!({ "todos": true })).await?;
```

## 6. Pendências e limites conhecidos

| Item | Situação |
|---|---|
| Tickets `criar`/`responder` (cliente e admin) | Implementado via `send_multipart`; anexo opcional como `UploadFile`. |
| `GET /merger/api/tickets/anexo` | JPEG binário; use `raw` (o corpo não é JSON). Upload genérico em `/v1/upload`. |
| `pedidos` com `tipo` canal/correção | Não existe no painel. Use `reportes_criar` (`live`/`movie`/`series` + motivo). |
| Filtros em `pedidos/listar` | A rota ignora query; filtre no cliente. |
| `jogos_do_dia` | Query repassada, mas a rota não documenta filtros. |
| Visibilidade de feature gates | O painel pode responder 403 se a aba estiver oculta para a franquia; o crate não tenta contornar gates. |
| Paginação | Nenhuma dessas rotas expõe paginação server-side (exceto limites internos como `limit` de avisos). |

## 7. Erros mais comuns

| Situação | Resposta do painel |
|---|---|
| Sem crédito de pedido | `{"ok":false,"erro":"Sem créditos de pedido disponíveis"}` |
| Pedido duplicado pendente | `{"ok":false,"erro":"Você já tem um pedido pendente para este conteúdo"}` |
| Motivo fora da whitelist | `{"ok":false,"erro":"Motivo inválido"}` |
| Reporte duplicado/teto | `{"ok":false,"erro":"Você já tem um reporte pendente..."}` / mensagem do teto |
| Revenda destino inválida | `{"ok":false,"erro":"Revenda não encontrada ou não autorizado"}` |
| Pendência de outra sub-revenda | HTTP 403 `{"ok":false,"erro":"A revisão pertence à franquia responsável pela organização.","codigo":"apenas_franquia"}` |
| Token de prévia expirado | HTTP 409 `{"ok":false,"erro":"A revisão expirou ou mudou...","codigo":"previa_obsoleta"}` |
| Ticket inexistente | HTTP 404 `{"ok":false,"erro":"nao_encontrado"}` |
| Aviso indisponível | HTTP 404 `{"ok":false,"erro":"aviso_indisponivel"}` |
