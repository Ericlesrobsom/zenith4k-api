//! Autenticacao: login de franquia/admin, 2FA, whoami e logout.
//!
//! Contratos (fonte: `C:\Users\ericl\Desktop\goiptv`):
//! - `POST /merger/api/cliente_login` — `cliente_painel.go`
//! - `POST /merger/api/login` — `merger_login.go`
//! - `GET  /merger/api/whoami` — `merger_whoami.go` (ou equivalente)
//! - `POST /merger/api/logout` — `merger_login.go`
//!
//! O cookie de sessao e `merger_session` (HttpOnly, SameSite=Strict, 24h) e e
//! gerenciado automaticamente pelo cookie jar do `MergerClient`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

/// Resultado do login.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "estado", rename_all = "snake_case")]
pub enum LoginOutcome {
    Ok { whoami: Value },
    TwoFactorRequired { email_mascarado: Option<String> },
    Rejected { erro: String, vencido: bool },
}

impl MergerClient {
    /// Login usando as credenciais do `ClientConfig`.
    pub async fn login(&self) -> Result<LoginOutcome> {
        Err(ApiError::NotImplemented("MergerClient::login"))
    }

    /// Login enviando tambem o codigo 2FA.
    pub async fn login_with_code(&self, _codigo: &str) -> Result<LoginOutcome> {
        Err(ApiError::NotImplemented("MergerClient::login_with_code"))
    }

    /// `GET /merger/api/whoami` autenticado.
    pub async fn whoami(&self) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::whoami"))
    }

    /// `POST /merger/api/logout`.
    pub async fn logout(&self) -> Result<()> {
        Err(ApiError::NotImplemented("MergerClient::logout"))
    }

    /// Garante uma sessao valida (faz login se necessario).
    pub async fn ensure_login(&self) -> Result<()> {
        Err(ApiError::NotImplemented("MergerClient::ensure_login"))
    }
}
