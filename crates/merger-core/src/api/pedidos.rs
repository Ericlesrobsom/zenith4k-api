//! Pedidos de conteudo (filmes e series).
//!
//! Rotas (fonte: `routes_revendas.go`, `merger_api_pedidos.go`):
//! - `GET  /merger/api/pedidos/listar`
//! - `POST /merger/api/pedidos/criar`
//! - `POST /merger/api/pedidos/status` (admin)
//! - `POST /merger/api/pedidos/creditos/transferir`
//!
//! ## Contrato real do painel
//!
//! O handler `mergerAPIPedidosCriar` aceita apenas `tipo` `movie` ou `series`.
//! `tmdb_id` e `titulo` sao obrigatorios; `audio` aceita `dublado`,
//! `legendado` ou `ambos`; `episodios_selecionados` e uma lista de strings.
//! Campos como `nome`, `canal`, `episodios` ou `prioridade` **nao existem**
//! nessa rota (o painel ignora campos desconhecidos). Pedido de correcao de
//! canal/conteudo e feito por [`crate::api::reportes`].
//!
//! O painel cobra 1 credito de pedido por criacao (independente da quantidade
//! de episodios) e recusa um segundo pedido pendente para o mesmo `tmdb_id`.
//!
//! Respostas de sucesso: `{"ok":true,"creditos_pedidos":N}`. Erros de negocio
//! costumam vir com HTTP 200 e `{"ok":false,"erro":"..."}`.
//!
//! As funcoes `normalizar_*` sao puras e validam/ajustam o corpo antes do
//! request, replicando as regras do Go (defaults e enums). Erros de validacao
//! local usam [`ApiError::Config`].

use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

/// `GET /merger/api/pedidos/listar`
pub const ROTA_PEDIDOS_LISTAR: &str = "/merger/api/pedidos/listar";
/// `POST /merger/api/pedidos/criar`
pub const ROTA_PEDIDOS_CRIAR: &str = "/merger/api/pedidos/criar";
/// `POST /merger/api/pedidos/status` (admin)
pub const ROTA_PEDIDOS_STATUS: &str = "/merger/api/pedidos/status";
/// `POST /merger/api/pedidos/creditos/transferir`
pub const ROTA_PEDIDOS_CREDITOS_TRANSFERIR: &str = "/merger/api/pedidos/creditos/transferir";

/// Tipos aceitos por `pedidos/criar`.
pub const PEDIDOS_TIPOS: [&str; 2] = ["movie", "series"];
/// Faixas de audio aceitas por `pedidos/criar`.
pub const PEDIDOS_AUDIOS: [&str; 3] = ["dublado", "legendado", "ambos"];
/// Status aceitos por `pedidos/status`.
pub const PEDIDOS_STATUS: [&str; 3] = ["adicionado", "rejeitado", "pendente"];

/// Valida e normaliza o corpo de `POST /merger/api/pedidos/criar`.
///
/// Regras (espelho de `merger_api_pedidos.go`):
/// - `tmdb_id`: inteiro maior que zero (obrigatorio);
/// - `titulo`: string nao vazia (obrigatorio);
/// - `tipo`: `movie` ou `series`; qualquer outro valor vira `movie`;
/// - `audio`: `dublado`, `legendado` ou `ambos`; qualquer outro vira `dublado`;
/// - `poster_url` e `link_m3u`: strings opcionais (default vazio);
/// - `episodios_selecionados`: lista de strings (default `[]`).
///
/// Devolve um objeto apenas com os campos conhecidos pela rota.
pub fn normalizar_pedido_criacao(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "pedidos_criar")?;
    let tmdb_id = inteiro_positivo(obj, "tmdb_id")?;
    let titulo = texto_obrigatorio(obj, "titulo")?;
    let tipo = valor_enum_permissivo(obj, "tipo", &PEDIDOS_TIPOS, "movie")?;
    let audio = valor_enum_permissivo(obj, "audio", &PEDIDOS_AUDIOS, "dublado")?;
    let poster_url = texto_opcional(obj, "poster_url")?;
    let link_m3u = texto_opcional(obj, "link_m3u")?;
    let episodios = lista_de_textos(obj, "episodios_selecionados")?;

    Ok(json!({
        "tmdb_id": tmdb_id,
        "tipo": tipo,
        "titulo": titulo,
        "poster_url": poster_url,
        "audio": audio,
        "link_m3u": link_m3u,
        "episodios_selecionados": episodios,
    }))
}

/// Valida e normaliza o corpo de `POST /merger/api/pedidos/status`.
///
/// `id` precisa ser positivo; `status` precisa estar em [`PEDIDOS_STATUS`]
/// (sem default); `nota` e uma string opcional. A rota exige sessao admin.
pub fn normalizar_pedido_status(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "pedidos_status")?;
    let id = inteiro_positivo(obj, "id")?;
    let status = exigir_enum(obj, "status", &PEDIDOS_STATUS)?;
    let nota = texto_opcional(obj, "nota")?;

    Ok(json!({
        "id": id,
        "status": status,
        "nota": nota,
    }))
}

/// Valida e normaliza o corpo de `POST /merger/api/pedidos/creditos/transferir`.
///
/// `revenda_id` e `quantidade` precisam ser inteiros positivos. O painel
/// aceita apenas sub-revenda direta do solicitante como destino.
pub fn normalizar_transferencia_creditos(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "pedidos_creditos_transferir")?;
    let revenda_id = inteiro_positivo(obj, "revenda_id")?;
    let quantidade = inteiro_positivo(obj, "quantidade")?;

    Ok(json!({
        "revenda_id": revenda_id,
        "quantidade": quantidade,
    }))
}

impl MergerClient {
    /// Lista pedidos de conteudo visiveis para a sessao.
    ///
    /// A rota nao tem filtros proprios: admin ve tudo, franquia ve a propria
    /// subarvore. `query` e repassada como veio (o painel ignora parametros
    /// desconhecidos). Resposta:
    /// `{"ok":true,"pedidos":[...],"creditos_pedidos":N}`.
    pub async fn pedidos_listar(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_PEDIDOS_LISTAR, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Cria um pedido (filme ou serie) consumindo 1 credito de pedido.
    ///
    /// O corpo e normalizado por [`normalizar_pedido_criacao`]. **Nao** use
    /// esta rota para canal ou correcao: o painel so aceita `movie`/`series`.
    /// Veja [`crate::api::reportes::MergerClient::reportes_criar`].
    pub async fn pedidos_criar(&self, body: Value) -> Result<Value> {
        let corpo = normalizar_pedido_criacao(body)?;
        let resp = self
            .send("POST", ROTA_PEDIDOS_CRIAR, &[], Some(&corpo), false)
            .await?;
        Ok(resp.body)
    }

    /// Atualiza o status de um pedido (`adicionado`, `rejeitado` ou `pendente`).
    ///
    /// Rota de admin. Ao rejeitar um pedido pendente o painel devolve 1 credito
    /// ao autor. Resposta: `{"ok":true}` ou `{"ok":false,"erro":"..."}`.
    pub async fn pedidos_status(&self, body: Value) -> Result<Value> {
        let corpo = normalizar_pedido_status(body)?;
        let resp = self
            .send("POST", ROTA_PEDIDOS_STATUS, &[], Some(&corpo), false)
            .await?;
        Ok(resp.body)
    }

    /// Transfere creditos de pedido para uma sub-revenda direta.
    ///
    /// Resposta: `{"ok":true,"creditos_pedidos":<saldo do remetente>}`.
    pub async fn pedidos_creditos_transferir(&self, body: Value) -> Result<Value> {
        let corpo = normalizar_transferencia_creditos(body)?;
        let resp = self
            .send(
                "POST",
                ROTA_PEDIDOS_CREDITOS_TRANSFERIR,
                &[],
                Some(&corpo),
                false,
            )
            .await?;
        Ok(resp.body)
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

fn lista_de_textos(obj: &Map<String, Value>, campo: &str) -> Result<Vec<String>> {
    match obj.get(campo) {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::Array(itens)) => {
            let mut out = Vec::with_capacity(itens.len());
            for item in itens {
                let texto = item.as_str().ok_or_else(|| {
                    ApiError::Config(format!("campo {campo} deve conter apenas strings"))
                })?;
                let texto = texto.trim();
                if !texto.is_empty() {
                    out.push(texto.to_string());
                }
            }
            Ok(out)
        }
        Some(_) => Err(ApiError::Config(format!(
            "campo {campo} deve ser uma lista de strings"
        ))),
    }
}

fn valor_enum_permissivo(
    obj: &Map<String, Value>,
    campo: &str,
    permitidos: &[&str],
    padrao: &str,
) -> Result<String> {
    match obj.get(campo) {
        None | Some(Value::Null) => Ok(padrao.to_string()),
        Some(Value::String(s)) => {
            let valor = s.trim().to_ascii_lowercase();
            if permitidos.contains(&valor.as_str()) {
                Ok(valor)
            } else {
                Ok(padrao.to_string())
            }
        }
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
