//! Pedidos de conteudo (filmes, series, canais, correcoes).
//!
//! Rotas (fonte: `routes_revendas.go`, `merger_api_pedidos.go`):
//! - `GET  /merger/api/pedidos/listar`
//! - `POST /merger/api/pedidos/criar`
//! - `POST /merger/api/pedidos/status`
//! - `POST /merger/api/pedidos/creditos/transferir`
//! - `GET  /merger/api/admin/pedidos/creditos` (admin)

use serde_json::Value;

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

impl MergerClient {
    /// Lista pedidos de conteudo da franquia.
    pub async fn pedidos_listar(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::pedidos_listar"))
    }

    /// Cria um pedido (filme/serie/canal/correcao).
    pub async fn pedidos_criar(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::pedidos_criar"))
    }

    /// Atualiza o status de um pedido.
    pub async fn pedidos_status(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::pedidos_status"))
    }

    /// Transfere creditos de pedidos entre revendas da arvore.
    pub async fn pedidos_creditos_transferir(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented(
            "MergerClient::pedidos_creditos_transferir",
        ))
    }
}
