//! Envelope de sucesso `{"ok":true,"data":...,"meta":{"elapsed_ms":N}}`.

use std::time::Instant;

use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};

/// Envelope de sucesso com o tempo gasto pelo handler.
pub fn success(started: Instant, data: impl Serialize) -> Json<Value> {
    let data = serde_json::to_value(data).unwrap_or(Value::Null);
    Json(json!({
        "ok": true,
        "data": data,
        "meta": { "elapsed_ms": elapsed_ms(started) },
    }))
}

pub fn elapsed_ms(started: Instant) -> u64 {
    started.elapsed().as_millis() as u64
}
