//! Extratores que devolvem o envelope de erro da API local.

use axum::extract::{rejection::JsonRejection, FromRequest, Request};
use axum::Json;
use serde::de::DeserializeOwned;

use crate::error::AppError;

/// Igual a `axum::Json`, mas rejeita com o envelope `{"ok":false,...}`.
pub struct ApiJson<T>(pub T);

impl<S, T> FromRequest<S> for ApiJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) =
            Json::<T>::from_request(req, state)
                .await
                .map_err(|rejection: JsonRejection| {
                    AppError::new(rejection.status(), "json_invalido", rejection.body_text())
                })?;
        Ok(Self(value))
    }
}
