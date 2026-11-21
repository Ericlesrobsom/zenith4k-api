//! Autenticacao local da API: Bearer token ou loopback em modo dev.

use std::net::SocketAddr;

use axum::extract::ConnectInfo;
use axum::extract::Request;
use axum::extract::State;
use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use crate::state::AppState;

/// Exige `Authorization: Bearer $ZENITH_API_TOKEN`.
///
/// Sem token configurado (modo dev), aceita somente conexoes loopback e
/// recusa tudo que nao tenha `ConnectInfo` (evita rodar aberto por engano).
pub async fn require_token(State(st): State<AppState>, req: Request, next: Next) -> Response {
    if st.api_token.is_some() {
        let authorized = st.api_token.as_deref().is_some_and(|expected| {
            bearer(&req)
                .is_some_and(|provided| constant_time_eq(expected.as_bytes(), provided.as_bytes()))
        });
        if authorized {
            return next.run(req).await;
        }
        return deny(
            StatusCode::UNAUTHORIZED,
            "nao_autorizado",
            "envie Authorization: Bearer <ZENITH_API_TOKEN>",
        );
    }

    match req.extensions().get::<ConnectInfo<SocketAddr>>() {
        Some(ConnectInfo(addr)) if addr.ip().is_loopback() => next.run(req).await,
        Some(ConnectInfo(addr)) => deny(
            StatusCode::FORBIDDEN,
            "acesso_negado",
            format!(
                "sem ZENITH_API_TOKEN somente conexoes loopback sao aceitas (origem {})",
                addr.ip()
            ),
        ),
        None => deny(
            StatusCode::FORBIDDEN,
            "token_obrigatorio",
            "defina ZENITH_API_TOKEN para aceitar conexoes sem ConnectInfo",
        ),
    }
}

fn bearer(req: &Request) -> Option<&str> {
    let value = req.headers().get(AUTHORIZATION)?.to_str().ok()?.trim();
    let (scheme, token) = value.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("bearer") {
        return None;
    }
    let token = token.trim();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

/// Comparacao de bytes sem curto-circuito (mitiga timing attacks simples).
fn constant_time_eq(expected: &[u8], provided: &[u8]) -> bool {
    if expected.len() != provided.len() {
        return false;
    }
    let mut diff = 0u8;
    for (a, b) in expected.iter().zip(provided) {
        diff |= a ^ b;
    }
    diff == 0
}

fn deny(status: StatusCode, code: &'static str, message: impl Into<String>) -> Response {
    (
        status,
        Json(json!({
            "ok": false,
            "error": { "code": code, "message": message.into() }
        })),
    )
        .into_response()
}
