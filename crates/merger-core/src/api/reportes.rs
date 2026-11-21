//! Reportes de canal fora do ar / correcao de conteudo.
//!
//! Rotas (fonte: `routes_revendas.go`, `merger_api_reportes.go`):
//! - `GET  /merger/api/reportes/listar`
//! - `POST /merger/api/reportes/criar`
//! - `POST /merger/api/reportes/status`

use serde_json::Value;

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

impl MergerClient {
    pub async fn reportes_listar(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::reportes_listar"))
    }

    /// Reporta um canal/filme com problema.
    pub async fn reportes_criar(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::reportes_criar"))
    }

    pub async fn reportes_status(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::reportes_status"))
    }
}
