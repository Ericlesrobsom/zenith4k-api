//! # merger-core
//!
//! Cliente Rust para o painel **goiptv merger** — a mesma familia do painel
//! da franquia ZENITH4K (`https://aplicativo.znt4kh.uk`).
//!
//! O crate oferece tres camadas:
//!
//! 1. [`MergerClient::raw`] — passthrough autenticado para **qualquer** rota
//!    `/merger/api/*` ou `/api/webhook/*`, com allowlist de caminho e trava de
//!    confirmacao para operacoes destrutivas.
//! 2. Modulos tipados em [`api`] — atalhos ergonomicos para os fluxos do dia a
//!    dia (revendas, usuarios, planos, pedidos, reportes, catalogo, tickets,
//!    avisos).
//! 3. [`MergerClient::login`] — login de franquia (`/merger/api/cliente_login`)
//!    ou de admin (`/merger/api/login`), com suporte a 2FA por e-mail.
//!
//! ## Exemplo
//!
//! ```no_run
//! use merger_core::{ClientConfig, MergerClient};
//!
//! # async fn run() -> merger_core::Result<()> {
//! let cfg = ClientConfig::from_env()?;
//! let client = MergerClient::new(cfg)?;
//! let outcome = client.login().await?;
//! println!("login: {:?}", outcome);
//! let who = client.whoami().await?;
//! println!("logado como {:?}", who);
//! let stats = client.raw_get("/merger/api/stats", &[]).await?;
//! println!("{}", stats.body);
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod raw;

pub use client::MergerClient;
pub use config::{ClientConfig, PanelRole};
pub use error::{ApiError, Result};
