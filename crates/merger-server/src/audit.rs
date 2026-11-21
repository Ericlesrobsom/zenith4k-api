//! Auditoria local em JSONL.
//!
//! Cada request grava `{ts,method,path,status,elapsed_ms}` em
//! `./audit/api-AAAA-MM-DD.jsonl`. Corpo, query e cabecalhos **nunca** sao
//! registrados (podem conter senha, token ou dados de cliente).

use std::path::Path;
use std::time::Instant;

use axum::extract::Request;
use axum::extract::State;
use axum::middleware::Next;
use axum::response::Response;
use chrono::Utc;
use serde::Serialize;
use tokio::io::AsyncWriteExt;

use crate::state::AppState;

#[derive(Debug, Serialize)]
struct AuditEntry<'a> {
    ts: String,
    method: &'a str,
    path: &'a str,
    status: u16,
    elapsed_ms: u64,
    /// Caminho real chamado pelo `/v1/raw` (sem query/corpo).
    #[serde(skip_serializing_if = "Option::is_none")]
    panel_path: Option<&'a str>,
}

/// Middleware global: mede a request e grava a linha de auditoria.
///
/// Falha de auditoria nunca derruba a request (somente emite um `warn`).
pub async fn middleware(State(st): State<AppState>, req: Request, next: Next) -> Response {
    let started = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let mut response = next.run(req).await;
    let panel_path = response
        .headers()
        .get("x-audit-panel-path")
        .and_then(|valor| valor.to_str().ok())
        .map(str::to_string);
    if panel_path.is_some() {
        response.headers_mut().remove("x-audit-panel-path");
    }
    let status = response.status().as_u16();
    let elapsed_ms = started.elapsed().as_millis() as u64;
    record(
        &st.audit_dir,
        &method,
        &path,
        status,
        elapsed_ms,
        panel_path.as_deref(),
    )
    .await;
    response
}

async fn record(
    dir: &Path,
    method: &str,
    path: &str,
    status: u16,
    elapsed_ms: u64,
    panel_path: Option<&str>,
) {
    let entry = AuditEntry {
        ts: Utc::now().to_rfc3339(),
        method,
        path,
        status,
        elapsed_ms,
        panel_path,
    };
    let Ok(line) = serde_json::to_string(&entry) else {
        return;
    };
    let file = dir.join(format!(
        "api-{}.jsonl",
        chrono::Local::now().format("%Y-%m-%d")
    ));
    if let Err(err) = append_line(&file, &line).await {
        tracing::warn!(arquivo = %file.display(), %err, "falha ao gravar auditoria");
    }
}

async fn append_line(file: &Path, line: &str) -> std::io::Result<()> {
    if let Some(parent) = file.parent() {
        if !parent.as_os_str().is_empty() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }
    let mut handle = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file)
        .await?;
    handle.write_all(line.as_bytes()).await?;
    handle.write_all(b"\n").await?;
    handle.flush().await
}
