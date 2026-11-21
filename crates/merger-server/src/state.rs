//! Estado compartilhado do servidor e regra de login sob demanda.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use chrono::{DateTime, Utc};
use merger_core::error::ApiError;
use merger_core::raw::{RawRequest, RawResponse, RawUpload};
use merger_core::{ClientConfig, MergerClient};
use tokio::sync::Mutex;

/// Estado imutavel compartilhado por todos os handlers.
#[derive(Clone)]
pub struct AppState {
    /// Cliente do painel (uma sessao unica por processo).
    pub client: Arc<MergerClient>,
    /// Token exigido de clientes locais; `None` ativa o modo dev (loopback).
    pub api_token: Option<String>,
    /// Diretorio onde os arquivos `api-AAAA-MM-DD.jsonl` sao gravados.
    pub audit_dir: PathBuf,
    /// Momento em que o processo subiu.
    pub started_at: DateTime<Utc>,
    boot: Instant,
    /// Serializa logins automaticos (varias requests no boot, um login so).
    login_lock: Arc<Mutex<()>>,
}

impl AppState {
    pub fn new(
        client: MergerClient,
        api_token: Option<String>,
        audit_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            client: Arc::new(client),
            api_token,
            audit_dir: audit_dir.into(),
            started_at: Utc::now(),
            boot: Instant::now(),
            login_lock: Arc::new(Mutex::new(())),
        }
    }

    /// Monta o estado a partir das variaveis `ZENITH_*`.
    pub fn from_env() -> anyhow::Result<Self> {
        let cfg = ClientConfig::from_env()?;
        let client = MergerClient::new(cfg)?;
        let api_token = std::env::var("ZENITH_API_TOKEN")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let audit_dir = std::env::var("ZENITH_AUDIT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("audit"));
        Ok(Self::new(client, api_token, audit_dir))
    }

    /// Executa uma chamada autenticada no painel.
    ///
    /// O login e adiado ate a primeira chamada: se o painel responder 401,
    /// faz login e repete o request **uma** vez. Erros locais (caminho fora da
    /// allowlist, `confirm` ausente) acontecem antes de qualquer I/O.
    pub async fn call(&self, req: RawRequest) -> Result<RawResponse, ApiError> {
        match self.client.raw(req.clone()).await {
            Err(ApiError::NotAuthenticated) | Err(ApiError::SessionExpired) => {
                self.ensure_login().await?;
                self.client.raw(req).await
            }
            other => other,
        }
    }

    /// Executa um upload multipart autenticado, com login sob demanda.
    pub async fn call_upload(&self, upload: RawUpload) -> Result<RawResponse, ApiError> {
        match self.client.raw_upload(upload.clone()).await {
            Err(ApiError::NotAuthenticated) | Err(ApiError::SessionExpired) => {
                self.ensure_login().await?;
                self.client.raw_upload(upload).await
            }
            other => other,
        }
    }

    /// Faz login se necessario, serializando chamadas concorrentes.
    pub async fn ensure_login(&self) -> Result<(), ApiError> {
        let _guard = self.login_lock.lock().await;
        self.client.ensure_login().await
    }

    pub fn uptime_secs(&self) -> u64 {
        self.boot.elapsed().as_secs()
    }

    pub fn token_required(&self) -> bool {
        self.api_token.is_some()
    }
}
