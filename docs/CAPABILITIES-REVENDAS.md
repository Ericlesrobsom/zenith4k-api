# Capacidades — Revendas, usuarios e planos

Contratos conferidos **no codigo Go do painel** (`C:\Users\ericl\Desktop\goiptv`),
nao no painel vivo. Nenhuma chamada de rede foi feita para produzir este
documento; os shapes de resposta vem de `docs/live-shapes.json` (capturas
antigas, sem dados).

Modulos Rust cobertos:

- `crates/merger-core/src/api/revendas.rs`
- `crates/merger-core/src/api/usuarios.rs`
- `crates/merger-core/src/api/planos.rs`

Todos os metodos devolvem `serde_json::Value` com o corpo JSON do painel
(ou o texto cru quando a resposta nao e JSON) e passam pelo transporte
autenticado `MergerClient::send`, que aplica allowlist de rota e a trava de
`confirm` para operacoes destrutivas.

## Indice

- [Transporte e travas](#transporte-e-travas)
- [Revendas](#revendas)
- [Usuarios / clientes finais](#usuarios--clientes-finais)
- [Planos e creditos](#planos-e-creditos)
- [Diferencas entre o brief e o Go](#diferencas-entre-o-brief-e-o-go)
- [Campos sensiveis](#campos-sensiveis)
- [Erros esperados](#erros-esperados)

## Transporte e travas

- Rotas aceitas: apenas prefixos `/merger/api/`, `/api/webhook/` e `/api/mcp`.
- Metodos `DELETE` e caminhos com `/delete`, `/remove`, `/reset`, `/massa`,
  `/executar` etc. exigem `confirm = true`.
- `401` derruba o estado local (`SessionExpired`); `403` vira `Forbidden`;
  `404` vira `NotFound`; `429` vira `RateLimited`; `5xx` vira `Panel`.
- Os helpers `build_*` sao funcoes puras (sem rede) e validam o shape antes
  do request. Erros de shape viram `ApiError::Config`.

## Revendas

> **Atencao ao caminho:** a criacao de revenda **nao** e `/revendas/criar`.
> O painel usa a **colecao** `/merger/api/cliente/revendas` com o metodo
> `POST` (a mesma rota da listagem). O frontend oficial (`12d_criar_revenda_pagina.js`)
> faz exatamente isso. Testar `/revendas/criar` cai no catch-all do painel e
> pode devolver `200` vazio — nao significa que a rota exista.

| Operacao | Metodo + rota | Corpo / query | Resposta |
|---|---|---|---|
| `revendas_list` | `GET /merger/api/cliente/revendas` | query repassada (o painel ignora) | array de revendas |
| `revendas_criar` | `POST /merger/api/cliente/revendas` | `{username, password, credits, unlimited, pode_criar_*, pode_conectar_whatsapp}` | `{ok, username, usuario_acesso}` |
| `revendas_criar_com_vencimento` | `POST` + `POST /revendas/nota` | corpo acima + `dias_painel` | resposta acima + `vencimento_painel` |
| `revendas_editar` | `PUT /merger/api/cliente/revendas` | `{id, usuario_acesso?/password?/transfer?/set_pode_*?}` | `{ok}` |
| `revendas_toggle` | `PATCH /merger/api/cliente/revendas` | `{id, enabled}` | `{ok, afetados}` |
| `revendas_remover` | `DELETE /merger/api/cliente/revendas?id=N` | `confirm=true` | `{ok, removidos, linhas_apagadas}` |
| `revendas_tree` | `GET /merger/api/cliente/revendas/arvore` | — | array hierarquico (`sub_revendas`) |
| `revendas_renovar` | `POST /merger/api/cliente/revendas/renovar` | `{id, vencimento_atual}` | `{ok, id, username, vencimento_painel, ultima_renovacao_manual}` |
| `transfer_revenda_destinos` | `GET /merger/api/cliente/transfer_revenda/destinos` | `origem` | lista de pais possiveis |
| `transfer_revenda` | `POST /merger/api/cliente/transfer_revenda` | `{origem, novo_pai, confirmar_cross?}` | `{ok}` |
| `convite` | `GET /merger/api/cliente/convite` | — | `{ok, codigo, link}` |
| `convite_regenerar` | `POST /merger/api/cliente/convite/regenerar` | — | `{ok, codigo, link}` |
| `convite_adesao_pendentes` | `GET /merger/api/cliente/convite/adesao/pendentes` | — | `{ok, pedidos}` |
| `convite_adesao_confirmar` | `POST /merger/api/cliente/convite/adesao/confirmar` | `{pedido_id}` | resposta da adesao |
| `convite_mensalista_config` | `GET /merger/api/cliente/convite/mensalista-config` | — | `{ok, config, piso, ...}` |
| `convite_mensalista_config_salvar` | `POST` idem | `{habilitado, curva?}` | `{ok}` |
| `revendas_nota_ler` | `GET /merger/api/cliente/revendas/nota?id=` | query `id` | `{ok, username, contato_*, vencimento}` |
| `revendas_nota` | `POST /merger/api/cliente/revendas/nota` | `{id? ou username?, contato_*?, vencimento?}` | `{ok, ...}` |
| `revenda_unlimited` | `POST /merger/api/cliente/revenda_unlimited` | `{id, novo_estado, saldo_rebaixamento?}` | `{ok, ...}` |
| `transfer_revenda_destinos` | `GET /merger/api/cliente/transfer_revenda/destinos?origem=` | query `origem` | `{destinos: [username...]}` |
| `transfer_revenda` | `POST /merger/api/cliente/transfer_revenda` | `{origem, novo_pai, confirmar_cross?}` | resultado da transferencia |

### `revendas_list`

Devolve as sub-revendas diretas da franquia autenticada. Shape observado
(`docs/live-shapes.json`): array de objetos com `id`, `username`,
`usuario_acesso`, `credits`, `unlimited`, `enabled`, `created_at`,
`ultimo_login_em`, `ultimo_login_ip`, `n_sigma`, `pode_criar_revenda`,
`pode_criar_ilimitado`, `pode_conectar_whatsapp`, `mensalista`,
`contato_nome`, `contato_whatsapp`, `contato_email`, `contato_telegram`,
`vencimento_painel`, `ultima_renovacao_manual`.

```rust
// A franquia precisa estar autenticada (client.login().await?).
let revendas = client.revendas_list(&[]).await?;
for revenda in revendas.as_array().into_iter().flatten() {
    println!(
        "{} creditos={} vence={}",
        revenda["username"], revenda["credits"], revenda["vencimento_painel"]
    );
}
```

### `revendas_criar` / `revendas_criar_com_vencimento`

Cria uma sub-revenda filha da franquia autenticada. O painel valida:

- `username` unico (identidade/acesso);
- `password` com no minimo 4 caracteres;
- `credits` minimo **10** quando `unlimited = 0`;
- `unlimited = 1` exige que o criador tambem seja ilimitado e tenha
  `pode_criar_ilimitado`;
- `pode_criar_ilimitado` e forcado a `0` em revenda finita;
- debita os creditos iniciais do criador quando ele nao e ilimitado;
- grava o evento `revenda.criada` na auditoria.

O `POST` **nao aceita vencimento**. `revendas_criar_com_vencimento` faz a
criacao e, em seguida, grava `vencimento` via `revendas/nota` (a mesma
sequencia da UI). Se a segunda etapa falhar, a resposta traz
`vencimento_aplicado: false` e `aviso` — a revenda **foi criada**.

```rust
use serde_json::json;

let nova = client
    .revendas_criar_com_vencimento(
        json!({
            "username": "revenda_x",
            "password": "senha-forte",
            "credits": 100,
            "pode_criar_revenda": 1,
        }),
        30,
    )
    .await?;
// {"ok":true,"username":"revenda_x","vencimento_painel":"2026-12-21",...}
```

Na API local: `POST /v1/revendas/criar` (o campo `dias_painel` tem default
**30**; use `0` para nao mexer no vencimento).

### `revendas_editar`

`PUT` com **um grupo por chamada**, igual ao painel:

| Grupo | Corpo |
|---|---|
| Acesso | `{id, usuario_acesso, password?}` |
| Senha | `{id, password}` |
| Permissoes | `{id, set_pode_criar_revenda?, set_pode_criar_ilimitado?, set_pode_conectar_whatsapp?}` |
| Creditos | `{id, transfer}` (minimo 10; debita do pai se ele nao for ilimitado) |

Misturar grupos devolve `ApiError::Config` antes de qualquer request.

### `revendas_toggle` / `revendas_remover`

- `revendas_toggle(id, enabled)` usa `PATCH` e afeta a revenda + subarvore.
- `revendas_remover(id, confirm)` usa `DELETE ?id=N`; e **destrutivo**
  (apaga as linhas Xtream da subarvore) e exige `confirm = true`. A trava de
  exclusao do painel ainda pode recusar mesmo com a confirmacao local.

### `revendas_tree`

Arvore completa com `sub_revendas` aninhadas e contagens agregadas
(`n_sigma`, `n_ativos`, `n_testes`). O painel ja soma os niveis filhos no pai.

```rust
let arvore = client.revendas_tree().await?;
```

### `revendas_renovar`

Renova o **painel** da sub-revenda por 30 dias, sem cobrar creditos.
O Go decodifica com `DisallowUnknownFields`: o helper monta exatamente
`{id, vencimento_atual}` e descarta campos extras. `vencimento_atual` deve ser
a data exibida na lista (`YYYY-MM-DD`) ou `""` para pular a conferencia de
concorrencia. Se a data mudou desde a leitura, o painel responde conflito e
**nao** soma outro periodo.

```rust
use serde_json::json;

let resultado = client
    .revendas_renovar(json!({
        "id": 42,
        "vencimento_atual": "2026-11-30"
    }))
    .await?;
```

### `revendas_nota` / `revendas_nota_ler`

A nota guarda contato e observacoes internas da revenda. O alvo pode ser
`id` (inteiro) **ou** `username` (a tela de criacao so tem o username).
Strings vazias limpam o campo no painel; campos ausentes ficam intocados.

```rust
let nota = client.revendas_nota_ler(42).await?;

client
    .revendas_nota(json!({
        "username": "sub01",
        "contato_nome": "Fulano",
        "contato_whatsapp": "5511999999999",
        "contato_observacoes": "cobrar na segunda"
    }))
    .await?;
```

### `revenda_unlimited`

`novo_estado = 1` promove a revenda a ilimitada; `0` rebaixa. Promover exige
que o operador tambem seja ilimitado (`unlimited_credits=1` e
`pode_criar_ilimitado=1`) no painel. No rebaixamento, `saldo_rebaixamento`
define quantos creditos a revenda recebe de volta (default `0`).

```rust
let resposta = client
    .revenda_unlimited(json!({
        "id": 42,
        "novo_estado": 1,
        "saldo_rebaixamento": 0
    }))
    .await?;
```

> **Aviso:** rebaixar uma revenda ilimitada corta o acesso dela ao modo
> ilimitado e pode zerar a criacao de linhas. Confirme o efeito antes de
> chamar. A rota nao e classificada como destrutiva pelo transporte (nao ha
> `/delete` no caminho), entao a confirmacao e responsabilidade do chamador.
>
> **Saldo no rebaixamento:** se `novo_estado = 0` e `saldo_rebaixamento` for
> omitido, o cliente le o saldo atual da revenda (`credits`) e reaplica esse
> valor, em vez de mandar `0` (o painel zeraria tudo). Um `0` **explicito** e
> tratado como perda intencional. Se a leitura do saldo falhar, o metodo
> devolve `ApiError::Config` pedindo o valor explicito.

### `transfer_revenda_destinos` / `transfer_revenda`

O painel exige `origem` nos destinos e so aceita chamadas da **franquia raiz**.
O retorno dos destinos e uma lista de usernames validos para `novo_pai`.
`confirmar_cross` existe apenas no fluxo admin de outra franquia; no cliente
ele fica `false` e a rota executa a movimentacao intra-franquia.

```rust
let destinos = client.transfer_revenda_destinos("sub01").await?;
let resposta = client
    .transfer_revenda(json!({
        "origem": "sub01",
        "novo_pai": "sub02"
    }))
    .await?;
```

> **Aviso (operacao estrutural/destrutiva):** mover a raiz de uma franquia
> para outra descarta recursos de marca/operacao (loja, WhatsApp, IA,
> Telegram, cupons). O painel nao pede `confirm`; a checagem e apenas de
> permissao (franquia raiz). Trate como irreversivel e audite fora do cliente.

### Convite de revenda e adesao de mensalista

Sao **dois fluxos diferentes**, ambos no escopo da franquia:

| Fluxo | Rotas | Efeito |
|---|---|---|
| Convite de revenda | `GET /cliente/convite`, `POST /cliente/convite/regenerar` | link publico `/cadastro/<codigo>`; o cadastro cria a sub-revenda **ativa com 0 creditos** sob a franquia, sem debitar o pai |
| Adesao de mensalista | `GET/POST /cliente/convite/mensalista-config`, `GET /cliente/convite/adesao/pendentes`, `POST /cliente/convite/adesao/confirmar` | oferta de mensalista no convite; a confirmacao registra o recebimento manual de uma adesao |

```rust
let link = client.convite().await?;          // {"ok":true,"link":"https://.../cadastro/..."}
let novo = client.convite_regenerar().await?; // invalida o link anterior

let pendentes = client.convite_adesao_pendentes().await?;
if let Some(pedido) = pendentes["pedidos"].as_array().and_then(|p| p.first()) {
    let id = pedido["pedido_id"].as_str().unwrap_or_default();
    client.convite_adesao_confirmar(id).await?;
}
```

> O convite de revenda **nao tem aprovacao pendente**: quem se cadastra pelo
> link ja nasce ativo. `adesao/pendentes` pertence ao fluxo de **mensalista**
> (pagamento manual), nao ao convite de revenda.
>
> `convite_adesao_confirmar` registra um recebimento. Confirme apenas depois
> de conferir o pagamento; a operacao e auditada no painel.

## Usuarios / clientes finais

Todas as rotas usam o prefixo `/merger/api/cliente/sigma_users`.

| Operacao | Metodo + rota | Corpo / query | Resposta |
|---|---|---|---|
| `usuarios_list` | `GET /sigma_users` | `page`, `per_page`, `status`, `search`, `dono`, `escopo`, `servidor`, `servidor_cat`, `vence`, `vence_de`, `vence_ate`, `plano` | `{data, page, pages, per_page, total, contagens}` |
| `usuarios_revendas` | `GET /sigma_users/revendas` | — | array `{username, parent_user, contato_nome}` |
| `usuarios_criar` | `POST /sigma_users/criar` | **query**: `username`, `password`, `max_connections?`, `dias?`, `is_trial?`, `bouquet?`, `servidor_id?` | `{ok, username}` |
| `usuarios_editar` | `POST /sigma_users/editar` | JSON `{user_id, novo_username?, nova_senha?, validade?}` | `{ok}` |
| `usuarios_toggle` | `POST /sigma_users/toggle` | **query**: `id`, `acao` (`bloquear`/`desbloquear`) | `{ok}` |
| `usuarios_renovar` | `POST /sigma_users/renovar` | JSON `{user_id, plano_id}` | `{ok, ...}` |
| `usuarios_estender` | `POST /sigma_users/estender` | JSON `{user_id, dias}` (1..3650) | `{ok, ...}` |
| `usuarios_delete` | `POST /sigma_users/delete` | **query**: `id` + `confirm = true` local | `{ok}` |
| `usuarios_telas` | `POST /sigma_users/telas` | JSON `{user_id, telas}` | `{ok, ...}` |
| `usuarios_valor` | `POST /sigma_users/valor` | JSON `{user_id, valor}` | `{ok, valor_personalizado}` |
| `usuarios_adultos` | `POST /sigma_users/adultos` | **query**: `id`, `acao` (`ligar`/`desligar`), `plano_destino_id?` | resultado da troca |
| `usuarios_contato` | `POST /sigma_users/contato` | JSON `{user_id, ...}` | `{ok}` |
| `usuarios_contato_ler` | `GET /sigma_users/contato?id=` | query `id` | `{ok, username, password, contato_*}` |

### `usuarios_list`

A UI usa paginacao por `page` (1-based) e `per_page` (20, 50 ou 100 — o painel
ignora outros valores). `status` aceita os chips do painel (`todos`, `ativo`,
`bloqueado`, `expirado`, `teste`). `dono` aceita `meus`, `todos` ou o username
de uma revenda da subarvore; `escopo=arvore` inclui a subarvore do dono.

```rust
let query = vec![
    ("page".to_string(), "1".to_string()),
    ("per_page".to_string(), "50".to_string()),
    ("status".to_string(), "ativo".to_string()),
    ("search".to_string(), "fulano".to_string()),
];
let pagina = client.usuarios_list(&query).await?;
let total = pagina["total"].as_u64().unwrap_or(0);
```

> **Aviso:** a resposta inclui `password` em texto claro, `link_m3u`,
> `link_hls` e `link_renovacao` (contem usuario/senha na URL), alem de
> `online_ip`/`online_isp`/`online_ua`. Nao logue o JSON cru nem devolva esses
> campos para clientes nao autorizados.

### `usuarios_criar`

O painel le **query string** nesta rota (nao corpo JSON). O cliente aceita um
objeto JSON ergonomico e traduz para query:

```rust
let criado = client
    .usuarios_criar(serde_json::json!({
        "username": "cliente1",
        "password": "senha123",
        "max_connections": 2,
        "dias": 30,
        "bouquet": "COMPLETO C/ ADULTOS",
        "servidor_id": 1
    }))
    .await?;
```

`dias > 0` define a validade; `is_trial = 1` cria teste de 4h quando `dias`
esta ausente. O teto de telas da franquia e validado no painel
(`telas_invalida` com `teto` no corpo).

> **Aviso:** por contrato do painel, `username` e `password` viajam na **query
> string** desta rota. Evite logar a URL completa em proxies/APMs e nao
> reutilize a senha do painel como senha da linha.

### `usuarios_editar`

Troca username, senha e/ou validade sem debitar creditos e sem mexer em plano,
telas ou bouquet. `validade` aceita `YYYY-MM-DD` ou `vitalicio` (zera a
expiracao). Linha de teste so pode encurtar/ajustar a data; para virar paga,
use `usuarios_renovar`.

```rust
client
    .usuarios_editar(serde_json::json!({
        "user_id": 123,
        "novo_username": "cliente1b",
        "nova_senha": "nova-senha",
        "validade": "2027-01-31"
    }))
    .await?;
```

### `usuarios_toggle`

Bloqueia/desbloqueia a linha. O Go le `id` e `acao` da **query**; o helper
recusa `acao` diferente de `bloquear`/`desbloquear` para nao desbloquear por
engano. Em linha remota, o painel tambem propaga o estado ao XUI.

```rust
client
    .usuarios_toggle(serde_json::json!({"id": 123, "acao": "bloquear"}))
    .await?;
```

### `usuarios_renovar` / `usuarios_estender`

`renovar` troca o plano, empilha a nova duracao sobre o vencimento atual e
debita `creditos_consumo` do plano. `estender` so soma dias e cobra o
proporcional por tela/dia; nao aceita linha de teste.

```rust
client
    .usuarios_renovar(serde_json::json!({"user_id": 123, "plano_id": 4}))
    .await?;

client
    .usuarios_estender(serde_json::json!({"user_id": 123, "dias": 30}))
    .await?;
```

### `usuarios_delete`

> **Operacao destrutiva e irreversivel pelo cliente.** Exige
> `confirm = true`; sem isso o metodo devolve
> `ApiError::ConfirmationRequired` **sem enviar request**. O painel ainda
> aplica as travas de exclusao (linha de teste, politica dual raiz/revenda),
> copia a linha para a lixeira, apaga a linha no XUI remoto e remove o wrapper
> Maxplayer antes do `DELETE` local.

```rust
// 1) confirme com o usuario na conversa/UI
// 2) so entao:
let resposta = client
    .usuarios_delete(serde_json::json!({"id": 123}), true)
    .await?;
```

### `usuarios_telas` / `usuarios_valor`

`telas` respeita o teto da franquia e cobra proporcional ao aumentar (linha
ativa). `valor` grava o preco personalizado; `valor <= 0` limpa o custom e
volta ao valor do plano.

```rust
client
    .usuarios_telas(serde_json::json!({"user_id": 123, "telas": 3}))
    .await?;

client
    .usuarios_valor(serde_json::json!({"user_id": 123, "valor": 29.90}))
    .await?;
```

### `usuarios_adultos`

Alterna o bouquet `COMPLETO C/ ADULTOS` <-> `COMPLETO S/ ADULTOS` sem renovar
nem consumir creditos. Em linha remota, escolha o plano irmao com
`plano_destino_id` (o painel valida mesmo painel/remoto).

```rust
client
    .usuarios_adultos(serde_json::json!({
        "id": 123,
        "acao": "ligar",
        "plano_destino_id": 9
    }))
    .await?;
```

### `usuarios_contato` / `usuarios_contato_ler`

Atualiza contato, observacoes e senha. Strings vazias **limpam** o campo;
`nova_senha` vazia e ignorada. O GET devolve a senha atual em texto claro.

```rust
let atual = client.usuarios_contato_ler(123).await?;

client
    .usuarios_contato(serde_json::json!({
        "user_id": 123,
        "contato_nome": "Fulano",
        "contato_whatsapp": "",
        "observacoes": "prefere contato por WhatsApp"
    }))
    .await?;
```

> **Aviso:** o GET de contato expoe `password`. Trate como credencial.

## Planos e creditos

| Operacao | Metodo + rota | Corpo / query | Resposta |
|---|---|---|---|
| `planos_list` | `GET /merger/api/cliente/planos` | — | array de planos |
| `planos_list_incluindo_ocultos` | `GET /merger/api/cliente/planos?incluir_ocultos=1` | query `incluir_ocultos=1` | array de planos |
| `planos_creditos` | `POST /merger/api/cliente/planos/creditos` | `{plano_id, creditos}` | `{ok, creditos, creditos_padrao, creditos_rede}` |
| `planos_creditos_remover` | `DELETE /merger/api/cliente/planos/creditos?plano_id=` | query `plano_id` + `confirm = true` | `{ok, creditos, padrao, remover:true}` |
| `creditos` | `GET /merger/api/cliente/creditos` | — | `{ok, credits, unlimited}` |
| `criar_linha` | `POST /merger/api/cliente/criar_linha` | JSON `{username, password, plano_id, ...}` | `{ok, ...}` |

### `planos_list` / `planos_list_incluindo_ocultos`

O shape observado inclui `id`, `nome`, `tipo`, `duracao_valor`,
`duracao_unidade`, `max_telas`, `creditos_consumo`, `creditos_padrao`,
`creditos_rede`, `valor_reais`, `com_adultos`, `ativo`, `ordem`, `painel`,
`remoto_id`, `package_id_remoto`, `cor_icone`, `herdado`, `oculto`. Para
sub-revenda aparece tambem `valor_minimo` e, quando ha override de preco,
`valor_override`.

```rust
let planos = client.planos_list().await?;
let todos = client.planos_list_incluindo_ocultos().await?;
```

### `planos_creditos` / `planos_creditos_remover`

Ajusta o credito do plano para a arvore inteira da franquia raiz (somente a
franquia principal pode; sub-revenda recebe 403). O helper valida
`0 <= creditos <= 100`. A remocao usa `DELETE` e devolve o plano ao
valor padrao; por ser `DELETE`, o transporte exige `confirm = true`.

```rust
let ajuste = client.planos_creditos(4, 2).await?;

let removido = client.planos_creditos_remover(4, true).await?;
```

> **Aviso:** alterar o credito de um plano muda o custo de **todas** as
> renovacoes/criacoes da arvore. A remocao e reversivel (volta ao padrao), mas
> continua exigindo `confirm` por usar `DELETE`.

### `creditos`

Saldo consolidado da franquia autenticada:

```rust
let saldo = client.creditos().await?;
if saldo["credits"].as_i64().unwrap_or(0) <= 0 && !saldo["unlimited"].as_bool().unwrap_or(false) {
    eprintln!("sem creditos para criar/renovar");
}
```

### `criar_linha`

Cria a linha Xtream ja debitando o plano e, se o plano tem servidor remoto
(`remoto_id > 0`), cria primeiro no XUI e aborta o banco local em caso de
falha. Contato e `valor_personalizado` sao opcionais.

```rust
let linha = client
    .criar_linha(serde_json::json!({
        "username": "cliente1",
        "password": "senha123",
        "plano_id": 4,
        "valor_personalizado": 29.90,
        "contato_nome": "Fulano",
        "contato_whatsapp": "5511999999999",
        "servidor_id": 1,
        "renovacao_qr": false
    }))
    .await?;
```

## Diferencas entre o brief e o Go

O brief pedia para confirmar metodo/query/corpo no Go. Achados que mudaram a
implementacao:

1. `planos/creditos` **nao aceita GET** (405). So existe `POST {plano_id,
   creditos}` e `DELETE ?plano_id=`. Por isso `planos_creditos` recebe
   `(plano_id, creditos)` e `planos_creditos_remover` cobre o `DELETE`.
2. `sigma_users/criar`, `toggle`, `delete` e `adultos` leem **query string**,
   nao corpo JSON. Os metodos aceitam um objeto JSON e os helpers `build_*`
   traduzem para query.
3. `transfer_revenda/destinos` exige `origem`; a assinatura recebe a origem.
4. A listagem de `sigma_users` usa `page`/`per_page` (nao `offset`/`limit`) e
   filtra por `dono`, `search`, `status`, `servidor`, `plano`, `vence*`.
5. `revendas/renovar` usa `DisallowUnknownFields`: o corpo precisa ser
   exatamente `{id, vencimento_atual}`.
6. `revendas_nota` aceita `id` **ou** `username` como alvo; a leitura usa
   `GET .../nota?id=`.

## Campos sensiveis

| Campo | Onde aparece | Risco |
|---|---|---|
| `password` | `sigma_users` (lista, contato e criacao) | credencial Xtream em texto claro; na criacao vai na query string |
| `link_m3u`, `link_hls`, `link_renovacao` | `sigma_users` lista | URL com usuario/senha embutidos |
| `online_ip`, `ultimo_login_ip` | `sigma_users`, `revendas` | IP do cliente/revenda (PII) |
| `online_ua`, `online_isp` | `sigma_users` | fingerprint e operadora |
| `contato_*` | `revendas`, `sigma_users` | nome, WhatsApp, e-mail, Telegram |
| `valor_personalizado` | `sigma_users` | dado financeiro do cliente |
| `credits`, `unlimited` | `revendas`, `creditos` | saldo comercial da franquia |

Recomendacoes: nao logar respostas cruas; mascarar links e senha ao exibir;
nao persistir sessoes nem esses campos em disco; restringir o acesso a quem
opera a franquia.

## Erros esperados

| Situacao | Resultado |
|---|---|
| `confirm` ausente em `usuarios_delete`/`planos_creditos_remover` | `ApiError::ConfirmationRequired(rota)` sem request |
| corpo/query fora do shape | `ApiError::Config(mensagem)` sem request |
| sessao expirada (HTTP 401) | `ApiError::SessionExpired` |
| sub-revenda tentando operar rota de raiz | `ApiError::Forbidden(corpo)` |
| rota inexistente/versao diferente | `ApiError::NotFound` |
| muitas chamadas | `ApiError::RateLimited { retry_after_secs }` |
| falha do painel (HTTP 5xx) | `ApiError::Panel { status, body }` |

Os metodos devolvem `{ok:false, erro:...}` do proprio painel como `Value`
normal quando o status HTTP e 200 — sempre confira `ok` antes de assumir
sucesso (por exemplo, creditos insuficientes e `telas_invalida` chegam assim).
