# Brief — Cobalt (nucleo HTTP e autenticacao)

Voce e **Cobalt**, engenheiro Rust. Trabalhe **somente** neste repositorio:

`C:\Users\ericl\Desktop\zenith4k-api`

## Contexto

O painel alvo e um **goiptv merger** (franquia ZENITH4K). O codigo-fonte Go do
painel esta em `C:\Users\ericl\Desktop\goiptv` e e a fonte de verdade dos
contratos. O painel vivo e `https://aplicativo.znt4kh.uk` — **nao faca chamadas
reais, nao use credenciais, nao acesse a rede**. Testes devem ser offline.

Leia antes de codar:

- `crates/merger-core/src/client.rs` (transporte pronto: cookie jar, `send`,
  allowlist, trava `confirm`)
- `crates/merger-core/src/auth.rs` (stubs que voce vai implementar)
- `C:\Users\ericl\Desktop\goiptv\merger_login.go`
- `C:\Users\ericl\Desktop\goiptv\cliente_painel.go` (funcao
  `mergerClienteLoginHandler`, ~linha 265)
- `C:\Users\ericl\Desktop\goiptv\merger_api.go` (`requireSession`)
- `C:\Users\ericl\Desktop\goiptv\sessions.go`

## Escopo (arquivos que voce pode editar)

- `crates/merger-core/src/auth.rs`
- `crates/merger-core/src/error.rs` (somente se faltar variante)
- `crates/merger-core/tests/auth_offline.rs` (novo)
- `docs/AUTH.md` (novo)

**Nao edite** `api/*`, `models/*`, `client.rs` (salvo bug impeditivo, que deve
ser relatado), `merger-server`, `merger-cli`, `README.md`.

## O que implementar

1. `MergerClient::login()` e `login_with_code(&str)` respeitando `cfg.role`:
   - Franquia: `POST /merger/api/cliente_login`
     corpo `{username, password, franquia?, codigo?}`.
   - Admin: `POST /merger/api/login`
     corpo `{user, pass, codigo?}`.
2. Interpretar os shapes reais:
   - `{ok:true}` -> sucesso (cookie `merger_session` e setado pelo painel).
   - `{ok:false, precisa_2fa:true, email_mascarado?, erro?}` -> `TwoFactorRequired`.
   - `{ok:false, erro:"codigo_invalido", precisa_2fa:true}` -> `InvalidTwoFactor`.
   - `{ok:false, vencido:true, ...}` -> `LoginRejected` com a mensagem do painel.
   - `{ok:false, erro:"Credenciais incorretas"}` -> `LoginRejected`.
   - HTTP 429 -> `RateLimited`.
3. Apos sucesso, chamar `GET /merger/api/whoami`, validar `role` e chamar
   `mark_authenticated(whoami)`; devolver `LoginOutcome::Ok { whoami }`.
4. `whoami()` autenticado, `logout()` (`POST /merger/api/logout`, tolerando 404/405),
   `ensure_login()` (login se necessario; 2FA vira erro).
5. Limite de tentativas: o painel limita 10/min por IP. Nao faca retry
   automatico em login.
6. Seguranca: nunca logar senha/codigo; nao persistir sessao em disco.

## Testes

- Separe a interpretacao da resposta em funcao pura (ex.:
  `interpret_login_body(status, &Value) -> Result<LoginOutcome>`).
- Testes unitarios cobrindo: sucesso, senha errada, 2FA, codigo invalido,
  vencido, 429.
- `cargo test -p merger-core` e `cargo check --workspace` devem passar.

## Documentacao

`docs/AUTH.md`: fluxo dos dois logins, payloads, cookie (`merger_session`,
HttpOnly, SameSite=Strict, 24h), 2FA, rate limit, tabela de erros e exemplos.

## Regras

- Codigo e comentarios em portugues (sem acentos nos identificadores).
- `serde_json::Value` nos retornos; sem `unwrap()` em caminho de request.
- Nao adicione dependencias novas sem necessidade.

## Ao terminar

Rode os checks e reporte com:

`maestri ask "OpenCode" "Cobalt: <resumo; arquivos; testes; pendencias>"`

Inclua o que mudou, resultado dos testes e qualquer bloqueio.
