# Autenticacao — goiptv merger (ZENITH4K)

Documento de referencia do fluxo de autenticacao implementado em
`crates/merger-core/src/auth.rs`. Os contratos foram extraidos do codigo Go do
painel (`C:\Users\ericl\Desktop\goiptv`), em especial `cliente_painel.go`,
`merger_login.go`, `sessions.go`, `merger_api.go` e `routes_merger_core.go`.

Nenhuma chamada real e feita nos testes: a interpretacao das respostas e uma
funcao pura (`interpret_login_body`) coberta por
`crates/merger-core/tests/auth_offline.rs`.

## Visao geral

| Papel (`PanelRole`) | Endpoint | Corpo |
| --- | --- | --- |
| Franquia / revenda (`Franchise`) | `POST /merger/api/cliente_login` | `{username, password, franquia?, codigo?}` |
| Admin mestre ou equipe (`Admin`) | `POST /merger/api/login` | `{user, pass, codigo?}` |

O papel vem de `ClientConfig::role` (`ZENITH_ROLE=cliente|admin`). O login de
equipe (`subadmin`) usa a **mesma rota** do admin mestre; o que diferencia e o
`role` devolvido pelo `whoami` (`subadmin`).

O cliente mantem um **cookie jar em memoria** (`reqwest::cookie::Jar`). Nada e
gravado em disco: reiniciar o processo perde a sessao e exige novo login.

## Fluxo

```
MergerClient::login()
        |
        +--> POST /merger/api/{cliente_login|login}   (sem retry)
        |         |
        |         +-- 429 ---------------------------> Err(RateLimited)
        |         +-- {ok:true} ---------------------> segue
        |         +-- {precisa_2fa:true} ------------> Ok(TwoFactorRequired)
        |         +-- {codigo_invalido} -------------> Err(InvalidTwoFactor)
        |         +-- {vencido:true|erro} -----------> Err(LoginRejected)
        |
        +--> GET /merger/api/whoami                   (cookie do painel)
        |         |
        |         +-- role "anon" -------------------> Err(SessionExpired)
        |         +-- role inesperado ---------------> Err(Forbidden)
        |
        +--> mark_authenticated(whoami) -------------> Ok(LoginOutcome::Ok)
```

`ensure_login()` reaproveita a sessao local por ate **23h** (o painel expira em
24h) e refaz o login quando necessario. Um desafio 2FA vira
`ApiError::TwoFactorRequired` — quem chama precisa resolver o codigo com
`login_with_code`.

## Payloads

### Franquia

```json
{
  "username": "minha_revenda",
  "password": "segredo",
  "franquia": "matriz"
}
```

- `franquia` e opcional. Ausente (ou vazio), o painel usa o proprio `username`
  como franquia.
- Quando informada, o painel valida que o usuario pertence a arvore da
  franquia; caso contrario responde
  `{"ok":false,"erro":"Usuário não pertence a esta franquia"}`.
- `codigo` so entra na segunda chamada (2FA).

### Admin

```json
{
  "user": "admin",
  "pass": "segredo"
}
```

`codigo` e opcional e so e enviado quando o desafio 2FA esta ativo para o admin
mestre. Login de equipe (`subadmin`) nao passa pelo gate de 2FA do mestre.

### Montagem no Rust

```rust
use merger_core::auth::build_login_body;
use merger_core::PanelRole;

let body = build_login_body(
    PanelRole::Franchise,
    "minha_revenda",
    "segredo",
    Some("matriz"),
    None,
);
assert_eq!(body["username"], "minha_revenda");
assert_eq!(body["franquia"], "matriz");
```

Campos opcionais vazios (`""` ou so espacos) sao omitidos. O corpo contem a
senha **em claro** (o painel usa HTTPS) e nunca deve ser logado.

## Respostas

### Sucesso

```json
{ "ok": true }
```

O painel responde `200` e seta o cookie de sessao. Em seguida o cliente chama
`GET /merger/api/whoami`; um `whoami` valido tem `role` diferente de `anon`:

```json
{
  "role": "cliente",
  "username": "minha_revenda",
  "franquia": "minha_revenda",
  "usuario_acesso": "...",
  "editor_ia_permitido": false,
  "assistente_liberado": false
}
```

Para admin/equipe, `role` e `admin` ou `subadmin` (equipe inclui
`abas_liberadas`).

O `login` valida o papel antes de marcar a sessao:

| Login | `role` aceito |
| --- | --- |
| Franquia | `cliente` |
| Admin | `admin` ou `subadmin` |

Qualquer outro valor (inclusive `anon`) falha com `ApiError::Forbidden`, e o
cliente tenta derrubar a sessao inesperada no painel antes de retornar.

### 2FA

Primeira chamada, sem `codigo`:

```json
{
  "ok": false,
  "precisa_2fa": true,
  "email_mascarado": "d***@exemplo.com"
}
```

Isso **nao e erro**: `login()` devolve
`LoginOutcome::TwoFactorRequired { email_mascarado }`. O chamador pede o codigo
ao usuario e chama `login_with_code`.

Codigo errado, expirado ou com tentativas estouradas (o painel usa a mesma
resposta anti-oraculo):

```json
{
  "ok": false,
  "precisa_2fa": true,
  "erro": "codigo_invalido",
  "email_mascarado": "d***@exemplo.com"
}
```

→ `ApiError::InvalidTwoFactor`.

Falha real de envio de e-mail:

```json
{
  "ok": false,
  "precisa_2fa": true,
  "erro": "falha_envio",
  "email_mascarado": "d***@exemplo.com"
}
```

→ `ApiError::Panel` (nao ha codigo para digitar; tentar de novo pode gerar
outro envio, respeitando o cooldown do painel).

Observacoes do painel:

- Codigo valido vale uma vez; o cooldown de reenvio **nao** invalida o codigo
  anterior.
- Kill-switch no servidor: `config_geral.login_2fa_off=1` desliga o gate.
- A chave reservada do admin mestre (`admin:master`) nunca e consultada pelo
  trilho de cliente.

### Rejeicoes

Credenciais erradas, usuario desabilitado, usuario fora da franquia:

```json
{ "ok": false, "erro": "Credenciais incorretas" }
```

Vencimento (mensalista ou franquia raiz):

```json
{
  "ok": false,
  "vencido": true,
  "pagamento": { "...": "..." }
}
```

```json
{
  "ok": false,
  "vencido": true,
  "painel_admin": true,
  "erro": "Seu painel venceu. Fale com o administrador para renovar."
}
```

```json
{
  "ok": false,
  "vencido": true,
  "painel_franquia": true,
  "erro": "O painel da sua franquia venceu. Fale com seu fornecedor."
}
```

A funcao pura devolve `LoginOutcome::Rejected { erro, vencido }` preservando a
mensagem do painel. `login()` converte esse estado em
`ApiError::LoginRejected(erro)` (o brief trata rejeicao como erro de login).
Quando o painel nao manda `erro`, usamos uma mensagem local de acordo com
`painel_admin` / `painel_franquia`.

## Cookie de sessao

| Atributo | Valor |
| --- | --- |
| Nome | `merger_session` |
| Valor | token hexadecimal aleatorio (32 bytes de entropia, 64 caracteres) |
| `Path` | `/` |
| `Max-Age` | `86400` (24h) |
| `HttpOnly` | sim |
| `SameSite` | `Strict` |
| `Secure` | ausente (o painel nao marca; use sempre HTTPS) |

O cookie e gerenciado pelo jar do `MergerClient`; nao e exposto em
`SessionState`. A sessao server-side tambem expira em 24h e e mantida em
memoria pelo painel — reiniciar o painel derruba todas as sessoes.

## Logout

`POST /merger/api/logout` (o handler Go aceita qualquer metodo, limpa o cookie e
redireciona). O cliente:

- limpa o estado local em sucesso;
- tolera `404` (rota ausente) e `405` (metodo nao aceito);
- tolera `401` (sessao ja expirada);
- em falha de rede devolve o erro e **mantem** o estado, pois a sessao no
  painel pode continuar valida.

## Rate limit e bloqueios

- O painel limita **10 tentativas de login por minuto por IP** (as duas rotas).
- Estouro → HTTP `429` com corpo `{"erro":"rate_limited"}` →
  `ApiError::RateLimited`. O cliente **nao faz retry automatico**; respeite
  `retry_after_secs` quando o painel enviar `Retry-After` ou
  `x-ratelimit-reset`.
- O login tambem passa por allowlist de IP do painel; IP nao permitido → `403`.
- Falhas repetidas de franquia podem gerar auto-bloqueio de IP pela franquia
  raiz (tabela `franquia_ips_bloqueados`); a resposta continua generica
  (`Credenciais incorretas`) para nao vazar o bloqueio.

## Tabela de erros

| `ApiError` | Origem | Significado |
| --- | --- | --- |
| `RateLimited { retry_after_secs }` | HTTP 429 | 10/min por IP estourado; nao repetir em loop |
| `TwoFactorRequired { email_mascarado }` | `ensure_login` | 2FA pendente; use `login_with_code` |
| `InvalidTwoFactor` | `codigo_invalido` | codigo errado/expirado/estourado |
| `LoginRejected(msg)` | `vencido` / `ok:false` | credenciais, franquia ou vencimento |
| `Forbidden(msg)` | HTTP 403 ou `role` inesperado | IP bloqueado ou papel divergente |
| `SessionExpired` | `whoami` com `role:"anon"` | cookie venceu/foi derrubado |
| `NotAuthenticated` | `whoami` sem login local | chame `login()`/`ensure_login()` |
| `Panel { status, body }` | 5xx, status inesperado, corpo sem JSON, `falha_envio` | falha do painel |
| `NotFound` | HTTP 404 | rota ausente (logout tolera) |
| `Http` | reqwest | rede/timeout/TLS |

## Exemplos

### Login de franquia

```rust
use merger_core::{ApiError, ClientConfig, MergerClient, PanelRole};

# async fn run() -> merger_core::Result<()> {
let mut cfg = ClientConfig::new("https://painel.exemplo", "minha_revenda", "segredo")?;
cfg.role = PanelRole::Franchise;
cfg.franquia = Some("matriz".into());

let client = MergerClient::new(cfg)?;
match client.login().await? {
    merger_core::auth::LoginOutcome::Ok { whoami } => {
        println!("entrou como {:?}", whoami["role"]);
    }
    merger_core::auth::LoginOutcome::TwoFactorRequired { email_mascarado } => {
        println!("codigo enviado para {email_mascarado:?}");
    }
    merger_core::auth::LoginOutcome::Rejected { erro, .. } => {
        // Na pratica `login()` ja converte isto em ApiError::LoginRejected.
        println!("recusado: {erro}");
    }
}
# Ok(())
# }
```

### Login com 2FA

```rust
# async fn run(client: merger_core::MergerClient) -> merger_core::Result<()> {
match client.login().await {
    Ok(merger_core::auth::LoginOutcome::TwoFactorRequired { email_mascarado }) => {
        println!("envie o codigo recebido em {email_mascarado:?}");
        let out = client.login_with_code("123456").await?;
        assert!(matches!(out, merger_core::auth::LoginOutcome::Ok { .. }));
    }
    Ok(_) => {}
    Err(merger_core::ApiError::InvalidTwoFactor) => {
        println!("codigo invalido ou expirado");
    }
    Err(e) => return Err(e),
}
# Ok(())
# }
```

### Sessao em um servidor/CLI

```rust
# async fn run(client: merger_core::MergerClient) -> merger_core::Result<()> {
// Faz login so se ainda nao houver sessao (ou se ela estiver perto do TTL).
match client.ensure_login().await {
    Ok(()) => {}
    Err(merger_core::ApiError::TwoFactorRequired { email_mascarado }) => {
        eprintln!("2FA necessario em {email_mascarado:?}; rode o login com --codigo");
        return Ok(());
    }
    Err(e) => return Err(e),
}

let stats = client.raw_get("/merger/api/stats", &[]).await?;
println!("{}", stats.body);
# Ok(())
# }
```

### Logout

```rust
# async fn run(client: merger_core::MergerClient) -> merger_core::Result<()> {
client.logout().await?;
assert!(!client.is_authenticated());
# Ok(())
# }
```

## Seguranca

- Senha e codigo 2FA nunca sao logados (nem em `tracing`, nem em erros).
- Nao ha persistencia de sessao em disco; o cookie mora so no jar em memoria.
- O login nao tem retry automatico (evita amplificar o rate limit e bloqueios).
- O papel devolvido pelo `whoami` e validado antes de `mark_authenticated`.
- `SessionState` guarda apenas `username`, `role`, `logged_in_at` e o ultimo
  `whoami` (o painel nao devolve senha).
- O passthrough `raw` continua restrito a allowlist de caminhos e a trava
  `confirm` para operacoes destrutivas.
