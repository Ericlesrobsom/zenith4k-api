//! Revendas / sub-revendas (arvore da franquia).
//!
//! Rotas (fonte: `routes_revendas.go`, `merger_api_revendas.go`):
//! - `GET  /merger/api/cliente/revendas`
//! - `GET  /merger/api/cliente/revendas/arvore`
//! - `POST /merger/api/cliente/revendas/renovar`
//! - `POST /merger/api/cliente/revendas/nota`
//! - `POST /merger/api/cliente/revenda_unlimited`
//! - `GET  /merger/api/cliente/transfer_revenda/destinos`

use serde_json::Value;

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

impl MergerClient {
    /// Lista as revendas visiveis para a franquia autenticada.
    pub async fn revendas_list(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::revendas_list"))
    }

    /// Arvore hierarquica de revendas.
    pub async fn revendas_tree(&self) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::revendas_tree"))
    }

    /// Renova manualmente uma sub-revenda.
    pub async fn revendas_renovar(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::revendas_renovar"))
    }

    /// Grava uma nota interna na revenda.
    pub async fn revendas_nota(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::revendas_nota"))
    }

    /// Liga/desliga o modo ilimitado de uma revenda.
    pub async fn revenda_unlimited(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::revenda_unlimited"))
    }

    /// Destinos possiveis para transferir uma revenda.
    pub async fn transfer_revenda_destinos(&self) -> Result<Value> {
        Err(ApiError::NotImplemented(
            "MergerClient::transfer_revenda_destinos",
        ))
    }
}
