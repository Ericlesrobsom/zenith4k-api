//! Binario `zenith4k-serve`: sobe a API REST local.

use std::net::SocketAddr;

use anyhow::Context;
use merger_server::{app, state::AppState};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Carrega .env (ignorado pelo Git) sem sobrescrever o ambiente.
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "merger_server=info,tower_http=info,merger_core=info".into()),
        )
        .init();

    let state = AppState::from_env()
        .context("defina ZENITH_URL, ZENITH_USER e ZENITH_PASS (veja .env.example)")?;

    if state.api_token.is_none() {
        tracing::warn!(
            "ZENITH_API_TOKEN nao definido: modo dev, somente conexoes loopback serao aceitas"
        );
    }

    let bind = std::env::var("ZENITH_BIND").unwrap_or_else(|_| "127.0.0.1:8787".to_string());
    let addr: SocketAddr = bind
        .parse()
        .with_context(|| format!("ZENITH_BIND invalido: {bind:?}"))?;

    if state.api_token.is_none() && !addr.ip().is_loopback() {
        tracing::warn!(
            %addr,
            "bind nao e loopback sem ZENITH_API_TOKEN; requests de outras origens serao recusadas"
        );
    }

    if let Err(err) = tokio::fs::create_dir_all(&state.audit_dir).await {
        tracing::warn!(
            dir = %state.audit_dir.display(),
            %err,
            "nao consegui criar o diretorio de auditoria; falhas serao ignoradas"
        );
    }

    let router = app(state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "zenith4k-serve ouvindo");
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async {
        let _ = tokio::signal::ctrl_c().await;
        tracing::info!("encerrando");
    })
    .await?;
    Ok(())
}
