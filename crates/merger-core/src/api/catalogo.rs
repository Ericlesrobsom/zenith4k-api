//! Catalogo: categorias, pendencias de correcao, jogos do dia, stats.
//!
//! Rotas (fonte: `routes_catalogo.go`, `routes_merger_core.go`):
//! - `GET  /merger/api/categorias`
//! - `GET  /merger/api/cliente/cat/canais|unidas`
//! - `GET  /merger/api/cliente/catalogo/pendencias`
//! - `GET  /merger/api/cliente/catalogo/pendencias/opcoes`
//! - `GET  /merger/api/cliente/catalogo/pendencias/previa`
//! - `POST /merger/api/cliente/catalogo/pendencias/revisar`
//! - `POST /merger/api/cliente/catalogo/pendencias/corrigir`
//! - `GET  /merger/api/cliente/agenda/jogos`
//! - `GET  /merger/api/stats`
//! - `GET  /merger/api/dashboard_financeiro`

use serde_json::Value;

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

impl MergerClient {
    /// Categorias visiveis.
    pub async fn categorias(&self) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::categorias"))
    }

    /// Canais de uma categoria.
    pub async fn cat_canais(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::cat_canais"))
    }

    /// Pendencias de catalogo (correcoes sugeridas).
    pub async fn catalogo_pendencias(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::catalogo_pendencias"))
    }

    /// Opcoes de correcao para uma pendencia.
    pub async fn catalogo_pendencia_opcoes(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented(
            "MergerClient::catalogo_pendencia_opcoes",
        ))
    }

    /// Previa da correcao.
    pub async fn catalogo_pendencia_previa(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented(
            "MergerClient::catalogo_pendencia_previa",
        ))
    }

    /// Revisa (marca) uma pendencia.
    pub async fn catalogo_pendencia_revisar(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented(
            "MergerClient::catalogo_pendencia_revisar",
        ))
    }

    /// Aplica a correcao de uma pendencia.
    pub async fn catalogo_pendencia_corrigir(&self, _body: Value) -> Result<Value> {
        Err(ApiError::NotImplemented(
            "MergerClient::catalogo_pendencia_corrigir",
        ))
    }

    /// Jogos do dia da agenda esportiva.
    pub async fn jogos_do_dia(&self, _query: &[(String, String)]) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::jogos_do_dia"))
    }

    /// Estatisticas do painel.
    pub async fn stats(&self) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::stats"))
    }

    /// Dashboard financeiro.
    pub async fn dashboard_financeiro(&self) -> Result<Value> {
        Err(ApiError::NotImplemented("MergerClient::dashboard_financeiro"))
    }
}
