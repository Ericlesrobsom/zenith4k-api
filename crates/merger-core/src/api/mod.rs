//! Atalhos tipados por dominio. Cada modulo adiciona `impl MergerClient`.
//!
//! Todos os metodos devolvem `serde_json::Value` (o corpo JSON do painel) e
//! deixam o cliente generico [`crate::MergerClient::raw`] cobrir qualquer rota
//! que ainda nao tenha atalho.

pub mod apps_ibo;
pub mod avisos;
pub mod catalogo;
pub mod pedidos;
pub mod planos;
pub mod reportes;
pub mod revendas;
pub mod tickets;
pub mod usuarios;
