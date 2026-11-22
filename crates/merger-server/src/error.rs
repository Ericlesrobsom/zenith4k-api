//! Mapeamento de `ApiError` para o envelope HTTP de erro.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use merger_core::ApiError;
use serde_json::json;

/// Erro de handler que ja sabe virar `{"ok":false,"error":{...}}`.
#[derive(Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
    /// Corpo bruto do painel (usado em erro de negocio).
    pub details: Option<serde_json::Value>,
}

impl AppError {
    pub fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            details: None,
        }
    }

    /// Anexa o corpo original do painel ao erro (campo `details.panel`).
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    /// Requisicao malformada (JSON invalido, query invalida...).
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "requisicao_invalida", message)
    }
}

impl From<ApiError> for AppError {
    fn from(err: ApiError) -> Self {
        // Mapeamento pedido no brief; variantes nao listadas ganham o status
        // mais fiel ao significado (401 para login recusado, 501 para stub).
        let (status, code) = match &err {
            ApiError::NotAuthenticated => (StatusCode::UNAUTHORIZED, "nao_autenticado"),
            ApiError::SessionExpired => (StatusCode::UNAUTHORIZED, "sessao_expirada"),
            ApiError::StaleGeneration => (StatusCode::CONFLICT, "conta_desatualizada"),
            ApiError::TwoFactorRequired { .. } => {
                (StatusCode::PRECONDITION_REQUIRED, "dois_fatores_necessario")
            }
            ApiError::InvalidTwoFactor => (StatusCode::UNAUTHORIZED, "codigo_2fa_invalido"),
            ApiError::LoginRejected(_) => (StatusCode::UNAUTHORIZED, "login_recusado"),
            ApiError::Forbidden(_) => (StatusCode::FORBIDDEN, "sem_permissao"),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "nao_encontrado"),
            ApiError::ConfirmationRequired(_) => (StatusCode::CONFLICT, "confirmacao_necessaria"),
            ApiError::RateLimited { .. } => (StatusCode::TOO_MANY_REQUESTS, "rate_limit"),
            ApiError::PathNotAllowed(_) => (StatusCode::BAD_REQUEST, "caminho_nao_permitido"),
            ApiError::Panel { .. } => (StatusCode::BAD_GATEWAY, "erro_painel"),
            ApiError::Http(_) => (StatusCode::BAD_GATEWAY, "erro_rede"),
            ApiError::NotImplemented(_) => (StatusCode::NOT_IMPLEMENTED, "nao_implementado"),
            // Nos handlers, `Config` vem dos builders de validacao (corpo do
            // chamador), nao de erro interno: 400 e o status fiel.
            ApiError::Config(_) => (StatusCode::BAD_REQUEST, "parametros_invalidos"),
            ApiError::Json(_) => (StatusCode::INTERNAL_SERVER_ERROR, "json_invalido"),
            ApiError::Url(_) => (StatusCode::INTERNAL_SERVER_ERROR, "url_invalida"),
        };
        Self::new(status, code, err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let mut error = json!({
            "code": self.code,
            "message": self.message,
        });
        if let Some(details) = self.details {
            if let Some(obj) = error.as_object_mut() {
                obj.insert("details".to_string(), json!({ "panel": details }));
            }
        }
        let body = json!({ "ok": false, "error": error });
        (self.status, Json(body)).into_response()
    }
}
