//! Usuarios / clientes finais (linhas Xtream gerenciadas pela franquia).
//!
//! Rotas (fonte: `routes_sigma.go`, `sigma_*.go`):
//! - `GET  /merger/api/cliente/sigma_users`
//! - `GET  /merger/api/cliente/sigma_users/revendas`
//! - `POST /merger/api/cliente/sigma_users/criar`
//! - `POST /merger/api/cliente/sigma_users/editar`
//! - `POST /merger/api/cliente/sigma_users/toggle`
//! - `POST /merger/api/cliente/sigma_users/renovar`
//! - `POST /merger/api/cliente/sigma_users/estender`
//! - `POST /merger/api/cliente/sigma_users/delete`
//! - `POST /merger/api/cliente/sigma_users/telas`
//! - `POST /merger/api/cliente/sigma_users/valor`
//! - `POST /merger/api/cliente/sigma_users/adultos`
//! - `POST /merger/api/cliente/sigma_users/contato`

use serde_json::Value;

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

impl MergerClient {
    /// Lista/busca clientes. `query` aceita os filtros da UI (busca, revenda,
    /// status, pagina, limite...).
    pub async fn usuarios_list(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_list"))
    }

    /// Lista apenas as revendas (para selects/filtros).
    pub async fn usuarios_revendas(&self) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_revendas"))
    }

    /// Cria uma linha/cliente.
    pub async fn usuarios_criar(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_criar"))
    }

    /// Edita dados de um cliente.
    pub async fn usuarios_editar(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_editar"))
    }

    /// Liga/desliga o cliente.
    pub async fn usuarios_toggle(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_toggle"))
    }

    /// Renova o cliente com um plano.
    pub async fn usuarios_renovar(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_renovar"))
    }

    /// Estende o vencimento sem trocar o plano.
    pub async fn usuarios_estender(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_estender"))
    }

    /// Remove um cliente. Exige `confirm=true`.
    pub async fn usuarios_delete(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_delete"))
    }

    /// Ajusta numero de telas.
    pub async fn usuarios_telas(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_telas"))
    }

    /// Ajusta valor de cobranca.
    pub async fn usuarios_valor(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_valor"))
    }

    /// Liga/desliga conteudo adulto.
    pub async fn usuarios_adultos(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_adultos"))
    }

    /// Atualiza contato/whatsapp.
    pub async fn usuarios_contato(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::usuarios_contato"))
    }
}
