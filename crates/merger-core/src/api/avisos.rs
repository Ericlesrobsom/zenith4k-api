//! Avisos e notificacoes do painel (Central de Avisos).
//!
//! Rotas (fonte: `routes_loja_pwa.go`, `avisos_handlers.go`):
//! - `GET  /merger/api/avisos?limit=N`
//! - `POST /merger/api/avisos/lida`
//!
//! `GET /merger/api/avisos` nunca e gateado (receber aviso e canal). A
//! resposta tem `{avisos, nao_lidos, visao}`; a visao `admin` (gestao) lista
//! todos os avisos e sempre devolve `nao_lidos=0`.
//!
//! `POST /merger/api/avisos/lida` aceita `{"id":N}` para um aviso ou
//! `{"todos":true}` para zerar o contador. Quando os dois vem juntos, `todos`
//! vence (mesma ordem do handler Go).

use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

/// `GET /merger/api/avisos`
pub const ROTA_AVISOS: &str = "/merger/api/avisos";
/// `POST /merger/api/avisos/lida`
pub const ROTA_AVISO_LIDA: &str = "/merger/api/avisos/lida";

/// Limite default do feed quando `limit` esta ausente/invalido.
pub const AVISOS_LIMITE_PADRAO: u32 = 20;
/// Teto aceito pelo painel para `limit`.
pub const AVISOS_LIMITE_MAX: u32 = 100;

/// Monta a query de `GET /merger/api/avisos`.
///
/// O painel aceita `limit` entre 1 e 100; fora dessa faixa ele cai no default
/// de 20. Aqui omitimos o parametro invalido e deixamos o painel aplicar o
/// default (mesmo resultado, sem request ambiguo).
pub fn consulta_avisos(limit: Option<u32>) -> Vec<(String, String)> {
    match limit {
        Some(n) if (1..=AVISOS_LIMITE_MAX).contains(&n) => {
            vec![("limit".to_string(), n.to_string())]
        }
        _ => Vec::new(),
    }
}

/// Valida e normaliza o corpo de `POST /merger/api/avisos/lida`.
///
/// - `{"todos":true}` marca tudo que a sessao enxerga;
/// - `{"id":N}` marca um aviso (N > 0);
/// - se `todos` for `true`, o `id` e ignorado (mesma precedencia do Go).
pub fn normalizar_aviso_lida(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "aviso_lida")?;
    match obj.get("todos") {
        Some(Value::Bool(true)) => return Ok(json!({ "todos": true })),
        Some(Value::Bool(false)) | None | Some(Value::Null) => {}
        Some(_) => {
            return Err(ApiError::Config(
                "campo todos deve ser booleano".to_string(),
            ))
        }
    }
    let id = inteiro_positivo(obj, "id")?;
    Ok(json!({ "id": id }))
}

impl MergerClient {
    /// Feed de avisos da sessao.
    ///
    /// Use [`consulta_avisos`] para montar a query. Resposta:
    /// `{"avisos":[...],"nao_lidos":N,"visao":"cliente"|"admin"}`.
    pub async fn avisos(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self.send("GET", ROTA_AVISOS, query, None, false).await?;
        Ok(resp.body)
    }

    /// Marca um aviso como lido ou zera o contador (`{"todos":true}`).
    ///
    /// O corpo e normalizado por [`normalizar_aviso_lida`]. Resposta:
    /// `{"ok":true,"nao_lidos":N}` (ou `{"ok":true,"marcadas":N,"nao_lidos":N}`
    /// no modo `todos`).
    pub async fn aviso_lida(&self, body: Value) -> Result<Value> {
        let corpo = normalizar_aviso_lida(body)?;
        let resp = self
            .send("POST", ROTA_AVISO_LIDA, &[], Some(&corpo), false)
            .await?;
        Ok(resp.body)
    }
}

fn exigir_objeto<'a>(body: &'a Value, contexto: &str) -> Result<&'a Map<String, Value>> {
    body.as_object().ok_or_else(|| {
        ApiError::Config(format!(
            "{contexto}: o corpo precisa ser um objeto JSON com {{\"id\":N}} ou {{\"todos\":true}}"
        ))
    })
}

fn inteiro_positivo(obj: &Map<String, Value>, campo: &str) -> Result<i64> {
    let valor = obj
        .get(campo)
        .ok_or_else(|| ApiError::Config(format!("campo {campo} e obrigatorio")))?;
    let n = valor
        .as_i64()
        .ok_or_else(|| ApiError::Config(format!("campo {campo} deve ser um inteiro")))?;
    if n <= 0 {
        return Err(ApiError::Config(format!(
            "campo {campo} deve ser maior que zero"
        )));
    }
    Ok(n)
}
