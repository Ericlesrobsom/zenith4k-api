//! Planos, creditos e criacao de linha.
//!
//! Rotas (fonte: `routes_revendas.go`, `merger_api_planos.go`,
//! `merger_api_cliente_painel.go`):
//! - `GET  /merger/api/cliente/planos`
//! - `GET  /merger/api/cliente/planos/creditos`
//! - `GET  /merger/api/cliente/creditos`
//! - `POST /merger/api/cliente/criar_linha`

use serde_json::Value;

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

impl MergerClient {
    /// Planos visiveis para a franquia.
    pub async fn planos_list(&self) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::planos_list"))
    }

    /// Creditos por plano.
    pub async fn planos_creditos(&self) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::planos_creditos"))
    }

    /// Saldo consolidado da franquia.
    pub async fn creditos(&self) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::creditos"))
    }

    /// Cria uma linha Xtream (cliente final).
    pub async fn criar_linha(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::criar_linha"))
    }
}
