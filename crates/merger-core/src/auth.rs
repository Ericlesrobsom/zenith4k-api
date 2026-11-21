//! Autenticacao: login de franquia/admin, 2FA, whoami e logout.
//!
//! Contratos (fonte: `C:\Users\ericl\Desktop\goiptv`):
//! - `POST /merger/api/cliente_login` — `cliente_painel.go`
//! - `POST /merger/api/login` — `merger_login.go`
//! - `GET  /merger/api/whoami` — `cliente_painel.go`
//! - `POST /merger/api/logout` — `routes_merger_core.go` (handler aceita
//!   qualquer metodo e responde com redirect; toleramos 404/405)
//!
//! O cookie de sessao e `merger_session` (HttpOnly, SameSite=Strict, 24h) e e
//! gerenciado automaticamente pelo cookie jar do `MergerClient`.
//!
//! Seguranca:
//! - senha e codigo 2FA nunca sao registrados em log nem persistidos em disco;
//! - o login nao faz retry automatico (o painel limita 10 tentativas/min por IP);
//! - apos o login validamos o `role` devolvido pelo `whoami` antes de marcar a
//!   sessao como autenticada (fail-closed).

use chrono::{Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::client::MergerClient;
use crate::config::PanelRole;
use crate::error::{ApiError, Result};

/// Login de franquia / revenda.
pub const PATH_LOGIN_FRANQUIA: &str = "/merger/api/cliente_login";
/// Login de admin mestre ou de equipe (subadmin).
pub const PATH_LOGIN_ADMIN: &str = "/merger/api/login";
/// Validacao da sessao corrente.
pub const PATH_WHOAMI: &str = "/merger/api/whoami";
/// Encerramento da sessao.
pub const PATH_LOGOUT: &str = "/merger/api/logout";

/// TTL da sessao no painel (cookie `Max-Age=86400` + entry em memoria).
const SESSAO_TTL_HORAS: i64 = 24;
/// Renova a sessao um pouco antes do TTL para evitar corrida com a expiracao.
const SESSAO_RENOVAR_HORAS: i64 = 23;

/// Resultado do login.
///
/// A interpretacao pura da resposta usa todos os estados; o metodo
/// [`MergerClient::login`] aplica a politica do cliente: rejeicoes viram
/// [`ApiError::LoginRejected`] e um desafio 2FA continua sendo um `Ok` (o
/// chamador decide se pede o codigo ou aborta — [`MergerClient::ensure_login`]
/// aborta).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "estado", rename_all = "snake_case")]
pub enum LoginOutcome {
    /// Credenciais aceitas; `whoami` e o corpo de `GET /merger/api/whoami`.
    ///
    /// Na funcao pura [`interpret_login_body`] o campo vem como `null`; o
    /// metodo `login` o preenche depois de consultar o painel.
    Ok { whoami: Value },
    /// O painel aceitou a senha e enviou (ou tentou enviar) um codigo 2FA.
    TwoFactorRequired { email_mascarado: Option<String> },
    /// O painel recusou o login (credenciais, franquia ou vencimento).
    Rejected { erro: String, vencido: bool },
}

/// Monta o corpo JSON do login conforme o papel configurado.
///
/// - Franquia: `{username, password, franquia?, codigo?}`
/// - Admin: `{user, pass, codigo?}`
///
/// Campos opcionais vazios sao omitidos (o painel Go trata ausente e `""` da
/// mesma forma). O corpo nunca deve ser logado.
pub fn build_login_body(
    role: PanelRole,
    username: &str,
    password: &str,
    franquia: Option<&str>,
    codigo: Option<&str>,
) -> Value {
    let codigo = codigo.map(str::trim).filter(|c| !c.is_empty());
    match role {
        PanelRole::Franchise => {
            let mut body = json!({
                "username": username,
                "password": password,
            });
            if let Some(f) = franquia.map(str::trim).filter(|f| !f.is_empty()) {
                body["franquia"] = Value::String(f.to_string());
            }
            if let Some(c) = codigo {
                body["codigo"] = Value::String(c.to_string());
            }
            body
        }
        PanelRole::Admin => {
            let mut body = json!({
                "user": username,
                "pass": password,
            });
            if let Some(c) = codigo {
                body["codigo"] = Value::String(c.to_string());
            }
            body
        }
    }
}

/// Interpreta a resposta de um endpoint de login sem tocar na rede.
///
/// Shapes reais do painel:
///
/// | status | corpo | resultado |
/// | --- | --- | --- |
/// | 200 | `{"ok":true}` | `Ok(LoginOutcome::Ok { whoami: null })` |
/// | 200 | `{"ok":false,"precisa_2fa":true,"email_mascarado":"..."}` | `Ok(TwoFactorRequired)` |
/// | 200 | `{"ok":false,"precisa_2fa":true,"erro":"codigo_invalido"}` | `Err(InvalidTwoFactor)` |
/// | 200 | `{"ok":false,"precisa_2fa":true,"erro":"falha_envio"}` | `Err(Panel)` |
/// | 200 | `{"ok":false,"vencido":true,...}` | `Ok(Rejected { vencido: true, .. })` |
/// | 200 | `{"ok":false,"erro":"Credenciais incorretas"}` | `Ok(Rejected { vencido: false, .. })` |
/// | 429 | qualquer | `Err(RateLimited)` |
/// | outro | qualquer | `Err(Panel)` |
///
/// A funcao e publica para permitir testes offline e para que outros
/// componentes (servidor/CLI) possam reutilizar a mesma interpretacao.
pub fn interpret_login_body(status: u16, body: &Value) -> Result<LoginOutcome> {
    if status == 429 {
        return Err(ApiError::RateLimited {
            retry_after_secs: None,
        });
    }
    if !(200..300).contains(&status) {
        return Err(ApiError::Panel {
            status,
            body: descricao_body(body),
        });
    }
    if !body.is_object() {
        return Err(ApiError::Panel {
            status,
            body: "resposta de login sem JSON".to_string(),
        });
    }

    if body.get("ok").and_then(Value::as_bool).unwrap_or(false) {
        return Ok(LoginOutcome::Ok {
            whoami: Value::Null,
        });
    }

    let precisa_2fa = body
        .get("precisa_2fa")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let erro = body
        .get("erro")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let email_mascarado = body
        .get("email_mascarado")
        .and_then(Value::as_str)
        .map(str::to_string);

    if precisa_2fa {
        // Anti-oraculo do painel: codigo errado, expirado ou estourado tem o
        // MESMO shape; aqui viram o mesmo erro local.
        if erro == "codigo_invalido" {
            return Err(ApiError::InvalidTwoFactor);
        }
        // O painel nao conseguiu enviar o e-mail: nao ha codigo para digitar.
        if erro == "falha_envio" {
            return Err(ApiError::Panel {
                status,
                body: "o painel nao conseguiu enviar o codigo 2FA".to_string(),
            });
        }
        return Ok(LoginOutcome::TwoFactorRequired { email_mascarado });
    }

    let vencido = body
        .get("vencido")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if vencido {
        let msg = if erro.is_empty() {
            mensagem_vencimento(body)
        } else {
            erro.to_string()
        };
        return Ok(LoginOutcome::Rejected {
            erro: msg,
            vencido: true,
        });
    }

    let erro = if erro.is_empty() {
        "login recusado pelo painel".to_string()
    } else {
        erro.to_string()
    };
    Ok(LoginOutcome::Rejected {
        erro,
        vencido: false,
    })
}

/// Mensagem local quando o painel bloqueia por vencimento sem mandar `erro`.
fn mensagem_vencimento(body: &Value) -> String {
    if body
        .get("painel_franquia")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        "o painel da sua franquia venceu; fale com seu fornecedor".to_string()
    } else if body
        .get("painel_admin")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        "seu painel venceu; fale com o administrador".to_string()
    } else {
        "acesso bloqueado por vencimento; regularize para entrar".to_string()
    }
}

impl MergerClient {
    /// Login usando as credenciais do `ClientConfig`.
    ///
    /// Respeita `cfg.role`:
    /// - [`PanelRole::Franchise`] → `POST /merger/api/cliente_login`
    /// - [`PanelRole::Admin`] → `POST /merger/api/login` (admin mestre ou equipe)
    ///
    /// Em sucesso consulta `whoami`, valida o `role` esperado, marca a sessao
    /// como autenticada e devolve `LoginOutcome::Ok { whoami }`.
    ///
    /// Um desafio 2FA e devolvido como `LoginOutcome::TwoFactorRequired`; use
    /// [`MergerClient::login_with_code`] com o codigo recebido. Nao ha retry
    /// automatico (limite do painel: 10 tentativas/min por IP).
    pub async fn login(&self) -> Result<LoginOutcome> {
        self.login_inner(None).await
    }

    /// Login enviando tambem o codigo 2FA.
    ///
    /// Codigo vazio ou so com espacos equivale a [`MergerClient::login`] (o
    /// painel reenvia/gera um novo codigo). Codigo invalido ou expirado
    /// devolve [`ApiError::InvalidTwoFactor`].
    pub async fn login_with_code(&self, codigo: &str) -> Result<LoginOutcome> {
        let codigo = codigo.trim();
        let codigo = if codigo.is_empty() {
            None
        } else {
            Some(codigo)
        };
        self.login_inner(codigo).await
    }

    async fn login_inner(&self, codigo: Option<&str>) -> Result<LoginOutcome> {
        let cfg = self.config();
        let (path, esperado) = match cfg.role {
            PanelRole::Franchise => (PATH_LOGIN_FRANQUIA, "cliente"),
            PanelRole::Admin => (PATH_LOGIN_ADMIN, "admin/subadmin"),
        };
        let body = build_login_body(
            cfg.role,
            &cfg.username,
            &cfg.password,
            cfg.franquia.as_deref(),
            codigo,
        );

        // `send` ja converte 401/403/404/429/5xx em erros; qualquer um deles
        // encerra o login sem retry.
        let resp = self.send("POST", path, &[], Some(&body), false).await?;
        let outcome = interpret_login_body(resp.status, &resp.body)?;

        match outcome {
            LoginOutcome::Ok { .. } => {
                let whoami = self.whoami().await?;
                if let Err(e) = validar_role(cfg.role, &whoami) {
                    // Defesa em profundidade: nao deixamos um cookie de papel
                    // inesperado no jar. Melhor esforco para derrubar a sessao.
                    let _ = self.logout().await;
                    return Err(e);
                }
                self.mark_authenticated(whoami.clone());
                let username = whoami.get("username").and_then(serde_json::Value::as_str);
                tracing::debug!(role = esperado, username, "login concluido");
                Ok(LoginOutcome::Ok { whoami })
            }
            LoginOutcome::TwoFactorRequired { email_mascarado } => {
                Ok(LoginOutcome::TwoFactorRequired { email_mascarado })
            }
            LoginOutcome::Rejected { erro, .. } => {
                self.mark_logged_out();
                Err(ApiError::LoginRejected(erro))
            }
        }
    }

    /// `GET /merger/api/whoami` autenticado.
    ///
    /// O painel responde `200 {"role":"anon"}` quando o cookie nao vale mais;
    /// nesse caso devolvemos [`ApiError::NotAuthenticated`] (nunca houve login
    /// local) ou [`ApiError::SessionExpired`] (a sessao local expirou) e
    /// limpamos o estado em memoria.
    pub async fn whoami(&self) -> Result<Value> {
        let era_autenticado = self.is_authenticated();
        let resp = self.send("GET", PATH_WHOAMI, &[], None, false).await?;
        if !resp.ok {
            return Err(ApiError::Panel {
                status: resp.status,
                body: snippet(&resp.text),
            });
        }
        let role = resp
            .body
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("anon");
        if role == "anon" {
            self.mark_logged_out();
            return Err(if era_autenticado {
                ApiError::SessionExpired
            } else {
                ApiError::NotAuthenticated
            });
        }
        Ok(resp.body)
    }

    /// `POST /merger/api/logout`.
    ///
    /// Tolera 404/405 (rota ausente ou metodo nao aceito) e sessao ja expirada;
    /// em todos esses casos o estado local e limpo. Falha de rede mantem o
    /// estado (a sessao no painel pode continuar viva).
    pub async fn logout(&self) -> Result<()> {
        match self.send("POST", PATH_LOGOUT, &[], None, false).await {
            Ok(_) => {
                self.mark_logged_out();
                Ok(())
            }
            Err(ApiError::NotFound) | Err(ApiError::SessionExpired) => {
                self.mark_logged_out();
                Ok(())
            }
            // 405 chega aqui como `Ok` (o `send` so trata 401/403/404/429/5xx),
            // entao ja foi aceito acima.
            Err(e) => Err(e),
        }
    }

    /// Garante uma sessao valida (faz login se necessario).
    ///
    /// Considera a sessao valida por ate 23h (TTL do painel: 24h); depois disso
    /// refaz o login. Um desafio 2FA vira [`ApiError::TwoFactorRequired`] — o
    /// chamador precisa resolver o codigo via [`MergerClient::login_with_code`].
    pub async fn ensure_login(&self) -> Result<()> {
        // Um login por vez: chamadas concorrentes no boot nao podem consumir
        // duas tentativas do limite de 10/min do painel.
        let _guard = self.login_lock().lock().await;
        let sess = self.session();
        if sess.authenticated {
            let recente = sess
                .logged_in_at
                .map(|at| {
                    Utc::now().signed_duration_since(at)
                        < ChronoDuration::hours(SESSAO_RENOVAR_HORAS)
                })
                .unwrap_or(false);
            if recente {
                return Ok(());
            }
        }
        if sess.authenticated {
            tracing::debug!(
                ttl_horas = SESSAO_TTL_HORAS,
                renovar_em = SESSAO_RENOVAR_HORAS,
                "sessao perto do TTL; refazendo login"
            );
        }

        match self.login().await? {
            LoginOutcome::Ok { .. } => Ok(()),
            LoginOutcome::TwoFactorRequired { email_mascarado } => {
                Err(ApiError::TwoFactorRequired { email_mascarado })
            }
            // Defensivo: `login` ja converte Rejected em erro.
            LoginOutcome::Rejected { erro, .. } => Err(ApiError::LoginRejected(erro)),
        }
    }
}

/// Valida o `role` do whoami contra o login que foi feito.
///
/// - franquia → `cliente`
/// - admin → `admin` ou `subadmin` (login de equipe usa a mesma rota)
///
/// Qualquer outro valor (inclusive `anon` ou campo ausente) falha fechado.
fn validar_role(esperado: PanelRole, whoami: &Value) -> Result<()> {
    let role = whoami.get("role").and_then(Value::as_str).unwrap_or("");
    let ok = match esperado {
        PanelRole::Franchise => role == "cliente",
        PanelRole::Admin => role == "admin" || role == "subadmin",
    };
    if ok {
        return Ok(());
    }
    let esperado_txt = match esperado {
        PanelRole::Franchise => "franquia (role cliente)",
        PanelRole::Admin => "admin (role admin ou subadmin)",
    };
    Err(ApiError::Forbidden(format!(
        "login de {esperado_txt} devolveu role inesperado {role:?}"
    )))
}

fn descricao_body(body: &Value) -> String {
    if body.is_null() {
        "resposta sem JSON".to_string()
    } else {
        snippet(&body.to_string())
    }
}

fn snippet(text: &str) -> String {
    const MAX: usize = 600;
    if text.chars().count() <= MAX {
        text.to_string()
    } else {
        let head: String = text.chars().take(MAX).collect();
        format!("{head}...")
    }
}
