use std::sync::RwLock;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::ClientConfig;
use crate::error::{ApiError, Result};
use crate::raw::{RawRequest, RawResponse, RawUpload, UploadFile};

/// Estado de autenticacao em memoria.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionState {
    pub authenticated: bool,
    pub username: Option<String>,
    pub role: Option<String>,
    pub logged_in_at: Option<DateTime<Utc>>,
    pub last_whoami: Option<Value>,
}

/// Cliente HTTP do painel.
///
/// Mantem um cookie jar proprio (uma sessao `merger_session` por instancia),
/// respeita timeout e expoe um passthrough autenticado para toda a API.
pub struct MergerClient {
    cfg: ClientConfig,
    http: Client,
    state: RwLock<SessionState>,
    /// Serializa `ensure_login` para nao gastar duas tentativas no rate limit.
    login_lock: tokio::sync::Mutex<()>,
}

const ALLOWED_PREFIXES: [&str; 3] = ["/merger/api/", "/api/webhook/", "/api/mcp"];

// Operacoes que exigem `confirm=true` no RawRequest.
const DESTRUCTIVE_HINTS: [&str; 13] = [
    "/delete",
    "/deletar",
    "/remove",
    "/remover",
    "/excluir",
    "/apagar",
    "/reset",
    "/limpar",
    "/restaurar",
    "/delete_orfa",
    "/bloquear_todos",
    "/massa",
    "/executar",
];

impl MergerClient {
    pub fn new(cfg: ClientConfig) -> Result<Self> {
        let http = Client::builder()
            .cookie_store(true)
            .timeout(Duration::from_secs(cfg.timeout_secs))
            .user_agent(cfg.user_agent.clone())
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()?;

        Ok(Self {
            cfg,
            http,
            state: RwLock::new(SessionState::default()),
            login_lock: tokio::sync::Mutex::new(()),
        })
    }

    pub fn config(&self) -> &ClientConfig {
        &self.cfg
    }

    pub fn api_origin(&self) -> Result<url::Url> {
        self.cfg.api_origin()
    }

    /// Snapshot do estado de sessao.
    pub fn session(&self) -> SessionState {
        self.state.read().expect("session lock envenenado").clone()
    }

    pub fn is_authenticated(&self) -> bool {
        self.state
            .read()
            .expect("session lock envenenado")
            .authenticated
    }

    /// Lock usado por `ensure_login` para evitar logins concorrentes.
    pub(crate) fn login_lock(&self) -> &tokio::sync::Mutex<()> {
        &self.login_lock
    }

    pub(crate) fn mark_authenticated(&self, whoami: Value) {
        let mut st = self.state.write().expect("session lock envenenado");
        st.authenticated = true;
        st.logged_in_at = Some(Utc::now());
        st.username = whoami
            .get("username")
            .and_then(Value::as_str)
            .map(str::to_string);
        st.role = whoami
            .get("role")
            .and_then(Value::as_str)
            .map(str::to_string);
        st.last_whoami = Some(whoami);
    }

    pub(crate) fn mark_logged_out(&self) {
        let mut st = self.state.write().expect("session lock envenenado");
        *st = SessionState::default();
    }

    /// Valida e normaliza o caminho antes de qualquer request.
    pub(crate) fn validate_path(path: &str) -> Result<String> {
        let p = path.trim();
        if p.is_empty() {
            return Err(ApiError::PathNotAllowed("caminho vazio".into()));
        }
        if p.contains("://") || p.contains("..") {
            return Err(ApiError::PathNotAllowed(format!(
                "caminho absoluto ou com traversal nao e permitido: {p}"
            )));
        }
        let normalized = if p.starts_with('/') {
            p.to_string()
        } else {
            format!("/{p}")
        };
        if !ALLOWED_PREFIXES
            .iter()
            .any(|prefix| normalized.starts_with(prefix))
        {
            return Err(ApiError::PathNotAllowed(format!(
                "{normalized} nao comeca com nenhum prefixo permitido ({})",
                ALLOWED_PREFIXES.join(", ")
            )));
        }
        Ok(normalized)
    }

    pub(crate) fn is_destructive(method: &str, path: &str) -> bool {
        let m = method.to_ascii_uppercase();
        if m == "DELETE" {
            return true;
        }
        let lower = path.to_ascii_lowercase();
        DESTRUCTIVE_HINTS.iter().any(|hint| lower.contains(hint))
    }

    /// Envia um request autenticado. Metodos destrutivos exigem `confirm`.
    pub(crate) async fn send(
        &self,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
        confirm: bool,
    ) -> Result<RawResponse> {
        let normalized = Self::validate_path(path)?;
        let method = Method::from_bytes(method.to_ascii_uppercase().as_bytes())
            .map_err(|_| ApiError::Config(format!("metodo HTTP invalido: {method}")))?;

        if Self::is_destructive(method.as_str(), &normalized) && !confirm {
            return Err(ApiError::ConfirmationRequired(normalized));
        }

        let mut url = self
            .api_origin()?
            .join(normalized.trim_start_matches('/'))?;
        if !query.is_empty() {
            let mut qp = url.query_pairs_mut();
            for (k, v) in query {
                qp.append_pair(k, v);
            }
        }

        let started = Instant::now();
        let mut req = self
            .http
            .request(method.clone(), url)
            .header(reqwest::header::ACCEPT, "application/json, text/plain, */*");
        if let Some(payload) = body {
            req = req.json(payload);
        }

        let resp = req.send().await?;
        self.finish(resp, started).await
    }

    /// Normaliza a resposta (JSON/texto/erros) de qualquer transporte.
    async fn finish(&self, resp: reqwest::Response, started: Instant) -> Result<RawResponse> {
        let status = resp.status();
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);
        let retry_after = resp
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.trim().parse::<u64>().ok())
            .or_else(|| {
                resp.headers()
                    .get("x-ratelimit-reset")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.trim().parse::<u64>().ok())
            });

        let text = resp.text().await?;
        let elapsed_ms = started.elapsed().as_millis() as u64;
        let body_json = serde_json::from_str::<Value>(&text).unwrap_or(Value::Null);

        let out = RawResponse {
            status: status.as_u16(),
            ok: status.is_success(),
            body: body_json,
            text,
            content_type,
            elapsed_ms,
        };

        match status {
            StatusCode::UNAUTHORIZED => {
                self.mark_logged_out();
                Err(ApiError::SessionExpired)
            }
            StatusCode::FORBIDDEN => Err(ApiError::Forbidden(snippet(&out.text))),
            StatusCode::NOT_FOUND => Err(ApiError::NotFound),
            StatusCode::TOO_MANY_REQUESTS => Err(ApiError::RateLimited {
                retry_after_secs: retry_after,
            }),
            s if s.is_server_error() => Err(ApiError::Panel {
                status: s.as_u16(),
                body: snippet(&out.text),
            }),
            _ => Ok(out),
        }
    }

    /// Envia `multipart/form-data` (campos texto + arquivos) para o painel.
    ///
    /// Usado por rotas que nao aceitam JSON: anexo de ticket, banner, logo do
    /// tema, importacao de CSV/M3U etc.
    pub async fn send_multipart(
        &self,
        path: &str,
        fields: &[(String, String)],
        files: &[UploadFile],
        confirm: bool,
    ) -> Result<RawResponse> {
        use reqwest::multipart::{Form, Part};

        let normalized = Self::validate_path(path)?;
        if Self::is_destructive("POST", &normalized) && !confirm {
            return Err(ApiError::ConfirmationRequired(normalized));
        }

        let mut form = Form::new();
        for (key, value) in fields {
            form = form.text(key.clone(), value.clone());
        }
        for file in files {
            let part = Part::bytes(file.bytes.clone())
                .file_name(file.filename.clone())
                .mime_str(&file.mime)?;
            form = form.part(file.field.clone(), part);
        }

        let url = self
            .api_origin()?
            .join(normalized.trim_start_matches('/'))?;
        let started = Instant::now();
        let resp = self
            .http
            .post(url)
            .header(reqwest::header::ACCEPT, "application/json, text/plain, */*")
            .multipart(form)
            .send()
            .await?;
        self.finish(resp, started).await
    }

    /// Passthrough multipart publico.
    pub async fn raw_upload(&self, upload: RawUpload) -> Result<RawResponse> {
        self.send_multipart(&upload.path, &upload.fields, &upload.files, upload.confirm)
            .await
    }

    /// Passthrough publico com allowlist + confirmacao.
    pub async fn raw(&self, req: RawRequest) -> Result<RawResponse> {
        self.send(
            &req.method,
            &req.path,
            &req.query,
            req.body.as_ref(),
            req.confirm,
        )
        .await
    }

    pub async fn raw_get(&self, path: &str, query: &[(String, String)]) -> Result<RawResponse> {
        self.send("GET", path, query, None, false).await
    }

    pub async fn raw_post(&self, path: &str, body: Value) -> Result<RawResponse> {
        self.send("POST", path, &[], Some(&body), false).await
    }
}

fn snippet(text: &str) -> String {
    const MAX: usize = 600;
    if text.chars().count() <= MAX {
        text.to_string()
    } else {
        let head: String = text.chars().take(MAX).collect();
        format!("{head}...")
    }
}
