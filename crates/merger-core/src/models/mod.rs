//! Modelos de entrada/saida por dominio.
//!
//! Os corpos do painel sao JSON e variam por deploy. Os tipos daqui cobrem os
//! campos estaveis observados no codigo-fonte; campos extras continuam
//! acessiveis via `serde_json::Value`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Envelope de erro/ok devolvido por varios handlers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelReply {
    #[serde(default)]
    pub ok: Option<bool>,
    #[serde(default)]
    pub erro: Option<String>,
    #[serde(default)]
    pub msg: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Resultado paginado generico.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    #[serde(default)]
    pub data: Vec<T>,
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(default)]
    pub offset: Option<u64>,
    #[serde(default)]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: Value,
}
