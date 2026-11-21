//! Avisos e notificacoes do painel.
//!
//! Rotas (fonte: `routes_loja_pwa.go`):
//! - `GET  /merger/api/avisos`
//! - `POST /merger/api/avisos/lida`

use serde_json::Value;

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

impl MergerClient {
    pub async fn avisos(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::avisos"))
    }

    /// Marca um aviso como lido.
    pub async fn aviso_lida(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::aviso_lida"))
    }
}
