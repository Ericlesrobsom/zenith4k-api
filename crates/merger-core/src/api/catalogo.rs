//! Catalogo: categorias, pendencias de correcao, jogos do dia, stats.
//!
//! Rotas (fonte: `routes_catalogo.go`, `merger_api_categorias.go`,
//! `cliente_canal_cat.go`, `catalogo_pendencias_api.go`,
//! `routes_merger_core.go`, `merger_api_sync.go`, `merger_api_dashboard.go`):
//! - `GET  /merger/api/categorias` (`tipo`, `servidor`)
//! - `GET  /merger/api/cliente/cat/canais` (`vid`, `tipo`)
//! - `GET  /merger/api/cliente/catalogo/pendencias`
//! - `GET  /merger/api/cliente/catalogo/pendencias/revisar` (`id`)
//! - `GET  /merger/api/cliente/catalogo/pendencias/opcoes` (`id`, `q`)
//! - `POST /merger/api/cliente/catalogo/pendencias/previa`
//! - `POST /merger/api/cliente/catalogo/pendencias/corrigir`
//! - `GET  /merger/api/cliente/agenda/jogos`
//! - `GET  /merger/api/stats`
//! - `GET  /merger/api/dashboard_financeiro` (`offset`)
//!
//! As rotas de pendencias exigem sessao cliente e, na pratica, a franquia
//! RAIZ (titular); sub-revenda recebe 403 `apenas_franquia`. `previa` devolve
//! um token de 10 minutos que precisa ser enviado em `corrigir`.
//!
//! As funcoes `consulta_*` e `normalizar_*` sao puras (testaveis offline) e
//! reproduzem defaults/clamps do Go. Erros de validacao local usam
//! [`ApiError::Config`].

use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

/// `GET /merger/api/categorias`
pub const ROTA_CATEGORIAS: &str = "/merger/api/categorias";
/// `GET /merger/api/cliente/cat/canais`
pub const ROTA_CAT_CANAIS: &str = "/merger/api/cliente/cat/canais";
/// `GET /merger/api/cliente/catalogo/pendencias`
pub const ROTA_CATALOGO_PENDENCIAS: &str = "/merger/api/cliente/catalogo/pendencias";
/// `GET /merger/api/cliente/catalogo/pendencias/revisar`
pub const ROTA_CATALOGO_PENDENCIA_REVISAR: &str = "/merger/api/cliente/catalogo/pendencias/revisar";
/// `GET /merger/api/cliente/catalogo/pendencias/opcoes`
pub const ROTA_CATALOGO_PENDENCIA_OPCOES: &str = "/merger/api/cliente/catalogo/pendencias/opcoes";
/// `POST /merger/api/cliente/catalogo/pendencias/previa`
pub const ROTA_CATALOGO_PENDENCIA_PREVIA: &str = "/merger/api/cliente/catalogo/pendencias/previa";
/// `POST /merger/api/cliente/catalogo/pendencias/corrigir`
pub const ROTA_CATALOGO_PENDENCIA_CORRIGIR: &str =
    "/merger/api/cliente/catalogo/pendencias/corrigir";
/// `GET /merger/api/cliente/agenda/jogos`
pub const ROTA_AGENDA_JOGOS: &str = "/merger/api/cliente/agenda/jogos";
/// `GET /merger/api/stats`
pub const ROTA_STATS: &str = "/merger/api/stats";
/// `GET /merger/api/dashboard_financeiro`
pub const ROTA_DASHBOARD_FINANCEIRO: &str = "/merger/api/dashboard_financeiro";

/// Tipos de midia aceitos pelo catalogo.
pub const CATALOGO_TIPOS: [&str; 3] = ["live", "movie", "series"];

/// Acoes aceitas na previa/correcao de pendencia.
pub const PENDENCIA_ACOES: [&str; 2] = ["substituir", "remover"];

/// Monta a query de `GET /merger/api/categorias`.
///
/// `tipo` aceita `live` (default do painel), `movie` ou `series`; `servidor`
/// e opcional (0/ausente = Principal). Valores invalidos falham localmente em
/// vez de gerar uma listagem vazia silenciosa.
pub fn consulta_categorias(
    tipo: Option<&str>,
    servidor: Option<u32>,
) -> Result<Vec<(String, String)>> {
    let mut query = Vec::new();
    if let Some(tipo) = tipo {
        query.push(("tipo".to_string(), tipo_catalogo(tipo)?));
    }
    if let Some(servidor) = servidor {
        if servidor > 0 {
            query.push(("servidor".to_string(), servidor.to_string()));
        }
    }
    Ok(query)
}

/// Monta a query de `GET /merger/api/cliente/cat/canais`.
///
/// `vid` e obrigatorio; `tipo` default `live`. A resposta e um array de
/// `{fonte_id, stream_id_original, nome, orfao}`.
pub fn consulta_cat_canais(vid: &str, tipo: Option<&str>) -> Result<Vec<(String, String)>> {
    let vid = vid.trim();
    if vid.is_empty() {
        return Err(ApiError::Config(
            "cat_canais: o parametro vid e obrigatorio".into(),
        ));
    }
    let tipo = match tipo {
        Some(tipo) => tipo_catalogo(tipo)?,
        None => "live".to_string(),
    };
    Ok(vec![
        ("vid".to_string(), vid.to_string()),
        ("tipo".to_string(), tipo),
    ])
}

/// Monta a query `id=<id>` das rotas de pendencia (`revisar`/`opcoes`).
pub fn consulta_pendencia(id: i64) -> Result<Vec<(String, String)>> {
    if id <= 0 {
        return Err(ApiError::Config(
            "catalogo_pendencia: id precisa ser maior que zero".into(),
        ));
    }
    Ok(vec![("id".to_string(), id.to_string())])
}

/// Monta a query de `GET .../pendencias/opcoes` (`id`, `q`).
///
/// `q` e opcional e o painel trunca em 200 caracteres; fazemos o mesmo corte
/// localmente para manter o request previsivel.
pub fn consulta_pendencia_opcoes(id: i64, q: Option<&str>) -> Result<Vec<(String, String)>> {
    let mut query = consulta_pendencia(id)?;
    if let Some(q) = q {
        let q: String = q.trim().chars().take(200).collect();
        if !q.is_empty() {
            query.push(("q".to_string(), q));
        }
    }
    Ok(query)
}

/// Deriva a query de `revisar` a partir do descritor recebido pelo metodo.
///
/// Aceita o proprio numero (`123`) ou um objeto `{"id":123}`, mantendo a
/// assinatura `body: Value` do stub original.
pub fn consulta_pendencia_de_valor(body: &Value) -> Result<Vec<(String, String)>> {
    let id = match body {
        Value::Number(n) => n.as_i64().ok_or_else(|| {
            ApiError::Config("catalogo_pendencia_revisar: id deve ser inteiro".into())
        })?,
        Value::Object(obj) => inteiro_positivo(obj, "id")?,
        _ => {
            return Err(ApiError::Config(
                "catalogo_pendencia_revisar: informe {\"id\":N} ou o proprio numero".into(),
            ))
        }
    };
    consulta_pendencia(id)
}

/// Valida e normaliza o corpo de `POST .../pendencias/previa`.
///
/// Corpo: `{id, revisao, acao, destino_id, fonte_id}`.
/// - `id` positivo; `revisao` string nao vazia (hash devolvido em `revisar`);
/// - `acao`: `substituir` ou `remover`;
/// - `destino_id`: obrigatorio em `substituir`, pode ficar vazio em `remover`;
/// - `fonte_id`: inteiro >= 0 (default 0; usado quando a pendencia e de canal).
///
/// Resposta: `{"ok":true,"token":"...","alteracoes":[...],"conflitos":[...]}`.
pub fn normalizar_pendencia_previa(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "catalogo_pendencia_previa")?;
    let id = inteiro_positivo(obj, "id")?;
    let revisao = texto_obrigatorio(obj, "revisao")?;
    let acao = exigir_enum(obj, "acao", &PENDENCIA_ACOES)?;
    let destino_id = texto_opcional(obj, "destino_id")?;
    if acao == "substituir" && destino_id.trim().is_empty() {
        return Err(ApiError::Config(
            "catalogo_pendencia_previa: destino_id e obrigatorio na acao substituir".into(),
        ));
    }
    let fonte_id = inteiro_nao_negativo(obj, "fonte_id")?;

    Ok(json!({
        "id": id,
        "revisao": revisao,
        "acao": acao,
        "destino_id": destino_id,
        "fonte_id": fonte_id,
    }))
}

/// Valida e normaliza o corpo de `POST .../pendencias/corrigir`.
///
/// Corpo: `{token}` — o token devolvido pela previa (validade de 10 minutos).
/// Resposta: `{"ok":true,"resolvidas":N,"pendentes":M}`.
pub fn normalizar_pendencia_corrigir(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "catalogo_pendencia_corrigir")?;
    let token = texto_obrigatorio(obj, "token")?;
    Ok(json!({ "token": token }))
}

/// Monta a query de `GET /merger/api/dashboard_financeiro`.
///
/// `offset` desloca a janela do grafico em blocos de 30 dias; o painel limita
/// a 24. Ausente/0 nao envia o parametro.
pub fn consulta_dashboard_financeiro(offset: Option<u32>) -> Vec<(String, String)> {
    match offset {
        Some(n) if n > 0 => vec![("offset".to_string(), n.min(24).to_string())],
        _ => Vec::new(),
    }
}

impl MergerClient {
    /// Categorias do catalogo (todas as categorias do tipo padrao `live`).
    pub async fn categorias(&self) -> Result<Value> {
        self.categorias_filtradas(&[]).await
    }

    /// Categorias com filtros (`tipo`, `servidor`).
    ///
    /// Use [`consulta_categorias`] para montar a query.
    pub async fn categorias_filtradas(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_CATEGORIAS, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Canais de uma categoria da franquia (`vid`, `tipo`).
    ///
    /// Use [`consulta_cat_canais`] para montar a query.
    pub async fn cat_canais(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_CAT_CANAIS, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Pendencias de catalogo (correcoes sugeridas) da franquia.
    ///
    /// Resposta: `{"ok":true,"pendencias":[...],"total":N}`.
    pub async fn catalogo_pendencias(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_CATALOGO_PENDENCIAS, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Opcoes de correcao para uma pendencia.
    ///
    /// Use [`consulta_pendencia_opcoes`] para montar a query.
    /// Resposta: `{"ok":true,"opcoes":[...],"mais":bool}`.
    pub async fn catalogo_pendencia_opcoes(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_CATALOGO_PENDENCIA_OPCOES, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Previa da correcao (nao altera dados; devolve token temporario).
    ///
    /// O corpo e normalizado por [`normalizar_pendencia_previa`].
    pub async fn catalogo_pendencia_previa(&self, body: Value) -> Result<Value> {
        let corpo = normalizar_pendencia_previa(body)?;
        let resp = self
            .send(
                "POST",
                ROTA_CATALOGO_PENDENCIA_PREVIA,
                &[],
                Some(&corpo),
                false,
            )
            .await?;
        Ok(resp.body)
    }

    /// Revisa uma pendencia pelo id (`GET .../revisar?id=N`).
    ///
    /// A assinatura mantem o `body` do stub por compatibilidade: aceita
    /// `{"id":N}` ou o proprio numero. O painel devolve o detalhe em
    /// `{"ok":true,"pendencia":{...}}`.
    pub async fn catalogo_pendencia_revisar(&self, body: Value) -> Result<Value> {
        let query = consulta_pendencia_de_valor(&body)?;
        let resp = self
            .send("GET", ROTA_CATALOGO_PENDENCIA_REVISAR, &query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Aplica a correcao de uma pendencia usando o token da previa.
    ///
    /// O corpo e normalizado por [`normalizar_pendencia_corrigir`].
    pub async fn catalogo_pendencia_corrigir(&self, body: Value) -> Result<Value> {
        let corpo = normalizar_pendencia_corrigir(body)?;
        let resp = self
            .send(
                "POST",
                ROTA_CATALOGO_PENDENCIA_CORRIGIR,
                &[],
                Some(&corpo),
                false,
            )
            .await?;
        Ok(resp.body)
    }

    /// Jogos do dia da agenda esportiva (proxy autenticado do painel).
    ///
    /// A resposta e o array cru da agenda (`time1`, `time2`, `horario`,
    /// `campeonato`/`competicao`, imagens etc.). `query` e repassada como veio.
    pub async fn jogos_do_dia(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_AGENDA_JOGOS, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Estatisticas do catalogo (`canais`, `filmes`, `series`, `bloqueados`,
    /// `fontes`, `servidores`...).
    pub async fn stats(&self) -> Result<Value> {
        let resp = self.send("GET", ROTA_STATS, &[], None, false).await?;
        Ok(resp.body)
    }

    /// Dashboard financeiro (janela atual, offset 0).
    pub async fn dashboard_financeiro(&self) -> Result<Value> {
        self.dashboard_financeiro_com_offset(0).await
    }

    /// Dashboard financeiro com deslocamento da janela do grafico.
    ///
    /// `offset` e clampado em 24 pelo painel; 0 mantem a janela atual.
    pub async fn dashboard_financeiro_com_offset(&self, offset: u32) -> Result<Value> {
        let query = consulta_dashboard_financeiro(if offset == 0 { None } else { Some(offset) });
        let resp = self
            .send("GET", ROTA_DASHBOARD_FINANCEIRO, &query, None, false)
            .await?;
        Ok(resp.body)
    }
}

fn tipo_catalogo(tipo: &str) -> Result<String> {
    let tipo = tipo.trim().to_ascii_lowercase();
    if CATALOGO_TIPOS.contains(&tipo.as_str()) {
        Ok(tipo)
    } else {
        Err(ApiError::Config(format!(
            "tipo de catalogo invalido: {tipo:?} (use {})",
            CATALOGO_TIPOS.join(", ")
        )))
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
