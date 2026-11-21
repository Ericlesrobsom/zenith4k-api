use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use serde_json::json;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "zenith4k_serve=info,tower_http=info".into()),
        )
        .init();

    let bind = std::env::var("ZENITH_BIND").unwrap_or_else(|_| "127.0.0.1:8787".to_string());
    let addr: SocketAddr = bind.parse()?;

    let app = Router::new()
        .route(
            "/health",
            get(|| async { Json(json!({"ok": true, "service": "zenith4k-api"})) }),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    tracing::info!(%addr, "zenith4k-api ouvindo");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
