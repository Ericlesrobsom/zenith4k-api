//! Reportes de conteudo (canal fora do ar, filme/serie/episodio com problema).
//!
//! Rotas (fonte: `routes_revendas.go`, `merger_api_reportes.go`):
//! - `GET  /merger/api/reportes/listar`
//! - `POST /merger/api/reportes/criar`
//! - `POST /merger/api/reportes/status` (admin)
//!
//! ## Contrato real do painel
//!
//! `tipo` aceita `live` (default), `movie` ou `series`. `stream_id_original`
//! e `nome_conteudo` sao obrigatorios, e `motivo` precisa estar na whitelist
//! do tipo (veja [`motivos_validos`]). O painel tambem aplica anti-spam
//! (duplicata pendente do mesmo stream) e um teto diario de reportes.
//!
//! Respostas de sucesso: `{"ok":true}`. Erros de negocio costumam vir com
//! HTTP 200 e `{"ok":false,"erro":"..."}`.

use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

/// `GET /merger/api/reportes/listar`
pub const ROTA_REPORTES_LISTAR: &str = "/merger/api/reportes/listar";
/// `POST /merger/api/reportes/criar`
pub const ROTA_REPORTES_CRIAR: &str = "/merger/api/reportes/criar";
/// `POST /merger/api/reportes/status` (admin)
pub const ROTA_REPORTES_STATUS: &str = "/merger/api/reportes/status";

/// Motivos aceitos para `tipo=live` (strings exatas do painel).
pub const MOTIVOS_LIVE: [&str; 5] = [
    "O canal está travando",
    "O canal está sem áudio",
    "O canal não da imagem",
    "Link off/quebrado",
    "O link está trocado",
];

/// Motivos aceitos para `tipo=movie` (strings exatas do painel).
pub const MOTIVOS_MOVIE: [&str; 5] = [
    "O filme não carrega",
    "O filme está sem áudio",
    "O filme não da imagem",
    "Link off/quebrado",
    "O link está trocado",
];

/// Motivos aceitos para `tipo=series` (strings exatas do painel).
pub const MOTIVOS_SERIES: [&str; 6] = [
    "O episódio não corresponde ao que está no link",
    "O episódio não funciona",
    "O episódio esta sem áudio",
    "O episódio não tem imagens",
    "Série completa off",
    "O link está trocado",
];

/// Status aceitos por `reportes/status`.
pub const REPORTES_STATUS: [&str; 2] = ["corrigido", "pendente"];

/// Whitelist de motivos por tipo. Tipo desconhecido cai em `live`, igual ao Go.
pub fn motivos_validos(tipo: &str) -> &'static [&'static str] {
    match tipo.trim().to_ascii_lowercase().as_str() {
        "movie" => &MOTIVOS_MOVIE,
        "series" => &MOTIVOS_SERIES,
        _ => &MOTIVOS_LIVE,
    }
}

/// Valida e normaliza o corpo de `POST /merger/api/reportes/criar`.
///
/// Regras (espelho de `merger_api_reportes.go`):
/// - `stream_id_original` e `nome_conteudo`: strings obrigatorias;
/// - `tipo`: `live` | `movie` | `series`; qualquer outro vira `live`;
/// - `motivo`: obrigatorio e restrito a [`motivos_validos`] do tipo;
/// - `fonte_id`: inteiro >= 0 (default 0);
/// - `episodios_afetados`: string opcional (default vazio).
pub fn normalizar_reporte_criacao(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "reportes_criar")?;
    let tipo = tipo_de_reporte(obj)?;
    let stream_id_original = texto_obrigatorio(obj, "stream_id_original")?;
    let nome_conteudo = texto_obrigatorio(obj, "nome_conteudo")?;
    let motivo = texto_obrigatorio(obj, "motivo")?;
    let motivos = motivos_validos(&tipo);
    if !motivos.contains(&motivo.as_str()) {
        return Err(ApiError::Config(format!(
            "motivo invalido para tipo {tipo}; use um de: {}",
            motivos.join(" | ")
        )));
    }
    let fonte_id = inteiro_nao_negativo(obj, "fonte_id")?;
    let episodios_afetados = texto_opcional(obj, "episodios_afetados")?;

    Ok(json!({
        "fonte_id": fonte_id,
        "stream_id_original": stream_id_original,
        "tipo": tipo,
        "nome_conteudo": nome_conteudo,
        "motivo": motivo,
        "episodios_afetados": episodios_afetados,
    }))
}

/// Valida e normaliza o corpo de `POST /merger/api/reportes/status`.
///
/// `id` precisa ser positivo; `status` aceita apenas `corrigido` ou
/// `pendente` (sem default); `nota` e string opcional. Rota de admin.
pub fn normalizar_reporte_status(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "reportes_status")?;
    let id = inteiro_positivo(obj, "id")?;
    let status = exigir_enum(obj, "status", &REPORTES_STATUS)?;
    let nota = texto_opcional(obj, "nota")?;

    Ok(json!({
        "id": id,
        "status": status,
        "nota": nota,
    }))
}

impl MergerClient {
    /// Lista reportes visiveis para a sessao.
    ///
    /// Admin ve tudo; franquia ve a propria subarvore. Resposta:
    /// `{"ok":true,"reportes":[...]}`.
    pub async fn reportes_listar(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_REPORTES_LISTAR, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Reporta um canal/filme/serie com problema.
    ///
    /// O corpo e normalizado por [`normalizar_reporte_criacao`]. Resposta:
    /// `{"ok":true}` ou `{"ok":false,"erro":"..."}`.
    pub async fn reportes_criar(&self, body: Value) -> Result<Value> {
        let corpo = normalizar_reporte_criacao(body)?;
        let resp = self
            .send("POST", ROTA_REPORTES_CRIAR, &[], Some(&corpo), false)
            .await?;
        Ok(resp.body)
    }

    /// Atualiza o status de um reporte (`corrigido` ou `pendente`).
    ///
    /// Rota de admin. Ao marcar `corrigido`, o painel grava `resolved_at` e
    /// dispara push para o autor. Resposta: `{"ok":true}`.
    pub async fn reportes_status(&self, body: Value) -> Result<Value> {
        let corpo = normalizar_reporte_status(body)?;
        let resp = self
            .send("POST", ROTA_REPORTES_STATUS, &[], Some(&corpo), false)
            .await?;
        Ok(resp.body)
    }
}

fn tipo_de_reporte(obj: &Map<String, Value>) -> Result<String> {
    match obj.get("tipo") {
        None | Some(Value::Null) => Ok("live".to_string()),
        Some(Value::String(s)) => {
            let tipo = s.trim().to_ascii_lowercase();
            Ok(match tipo.as_str() {
                "movie" | "series" => tipo,
                _ => "live".to_string(),
            })
        }
        Some(_) => Err(ApiError::Config("campo tipo deve ser string".into())),
    }
}

fn exigir_objeto<'a>(body: &'a Value, contexto: &str) -> Result<&'a Map<String, Value>> {
    body.as_object().ok_or_else(|| {
        ApiError::Config(format!(
            "{contexto}: o corpo precisa ser um objeto JSON com os campos da rota"
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

fn inteiro_nao_negativo(obj: &Map<String, Value>, campo: &str) -> Result<i64> {
    match obj.get(campo) {
        None | Some(Value::Null) => Ok(0),
        Some(valor) => {
            let n = valor
                .as_i64()
                .ok_or_else(|| ApiError::Config(format!("campo {campo} deve ser um inteiro")))?;
            if n < 0 {
                return Err(ApiError::Config(format!(
                    "campo {campo} nao pode ser negativo"
                )));
            }
            Ok(n)
        }
    }
}

fn texto_obrigatorio(obj: &Map<String, Value>, campo: &str) -> Result<String> {
    let valor = obj
        .get(campo)
        .and_then(Value::as_str)
        .ok_or_else(|| ApiError::Config(format!("campo {campo} e obrigatorio (string)")))?;
    let texto = valor.trim();
    if texto.is_empty() {
        return Err(ApiError::Config(format!(
            "campo {campo} nao pode ficar vazio"
        )));
    }
    Ok(texto.to_string())
}

fn texto_opcional(obj: &Map<String, Value>, campo: &str) -> Result<String> {
    match obj.get(campo) {
        None | Some(Value::Null) => Ok(String::new()),
        Some(Value::String(s)) => Ok(s.clone()),
        Some(_) => Err(ApiError::Config(format!("campo {campo} deve ser string"))),
    }
}

fn exigir_enum(obj: &Map<String, Value>, campo: &str, permitidos: &[&str]) -> Result<String> {
    let valor = obj
        .get(campo)
        .and_then(Value::as_str)
        .ok_or_else(|| ApiError::Config(format!("campo {campo} e obrigatorio (string)")))?;
    let valor = valor.trim().to_ascii_lowercase();
    if permitidos.contains(&valor.as_str()) {
        Ok(valor)
    } else {
        Err(ApiError::Config(format!(
            "campo {campo}: valor {valor:?} invalido (use {})",
            permitidos.join(", ")
        )))
    }
}
