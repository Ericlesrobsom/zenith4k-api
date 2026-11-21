//! Tickets de suporte.
//!
//! Rotas (fonte: `routes_tickets.go`):
//! - `GET  /merger/api/tickets/listar` (cliente)
//! - `GET  /merger/api/tickets/ver`
//! - `POST /merger/api/tickets/criar`
//! - `POST /merger/api/tickets/responder`
//! - `POST /merger/api/tickets/status`
//! - `POST /merger/api/tickets/anexo`
//! - `GET  /merger/api/admin/tickets/listar` (admin)

use serde_json::Value;

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

impl MergerClient {
    pub async fn tickets_listar(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::tickets_listar"))
    }

    pub async fn tickets_ver(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::tickets_ver"))
    }

    pub async fn tickets_criar(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::tickets_criar"))
    }

    pub async fn tickets_responder(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::tickets_responder"))
    }

    pub async fn tickets_status(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::tickets_status"))
    }
}
