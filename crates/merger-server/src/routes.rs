//! Handlers da API local.
//!
//! Todos os endpoints de dominio chamam o painel pelo passthrough
//! `MergerClient::raw` (via [`AppState::call`]), com login sob demanda. Assim o
//! servidor nao depende dos atalhos tipados de `merger-core` estarem prontos.

use std::collections::HashSet;
use std::sync::OnceLock;
use std::time::Instant;

use axum::body::Bytes;
use axum::extract::{Multipart, RawQuery, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use merger_core::auth::LoginOutcome;
use merger_core::raw::{RawRequest, RawUpload, UploadFile};
use merger_core::ApiError;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::envelope::success;
use crate::error::AppError;
use crate::extract::ApiJson;
use crate::state::AppState;

const OPENAPI_YAML: &str = include_str!("../../../docs/openapi.yaml");
const OPENAPI_JSON: &str = include_str!("../../../docs/openapi.json");
const SERVICE: &str = "zenith4k-api";

/// Rotas aceitas no passthrough (/v1/raw).
const RAW_METHODS: [&str; 6] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD"];

/// Catalogo de rotas do painel (gerado por `scripts/extract_paths.mjs`).
const PANEL_PATHS_TXT: &str = include_str!("panel_paths.txt");
static PANEL_PATHS: OnceLock<HashSet<&'static str>> = OnceLock::new();

fn painel_tem_rota(caminho: &str) -> bool {
    let rotas = PANEL_PATHS.get_or_init(|| {
        PANEL_PATHS_TXT
            .lines()
            .map(str::trim)
            .filter(|linha| !linha.is_empty())
            .collect()
    });
    rotas.contains(caminho)
}

/// Destino de upload aceito: rota do catalogo ou liberada por
/// `ZENITH_UPLOAD_PATHS` (lista separada por virgula).
fn upload_destino_permitido(caminho: &str) -> bool {
    if painel_tem_rota(caminho) {
        return true;
    }
    match std::env::var("ZENITH_UPLOAD_PATHS") {
        Ok(extra) => extra
            .split(',')
            .map(str::trim)
            .filter(|rota| !rota.is_empty())
            .any(|rota| rota == caminho),
        Err(_) => false,
    }
}

/// Converte recusa de negocio do painel (`ok:false` ou `erro`) em HTTP 409.
///
/// O painel responde `200 {"ok":false,"erro":"..."}` para recusas legitimas
/// (saldo insuficiente, duplicidade, trial, permissao). Sem isso, o envelope
/// da API diria `ok:true` e a automacao acharia que funcionou.
///
/// Nao e aplicado em `/v1/raw`: ali o chamador quer o corpo cru do painel.
fn checar_erro_negocio(data: Value) -> Result<Value, AppError> {
    let ok = data.get("ok").and_then(Value::as_bool);
    let tem_erro = data.get("erro").is_some_and(|valor| !valor.is_null());
    if ok != Some(false) && !(ok.is_none() && tem_erro) {
        return Ok(data);
    }
    let mensagem = data
        .get("erro")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|mensagem| !mensagem.is_empty())
        .or_else(|| data.get("msg").and_then(Value::as_str).map(str::trim))
        .unwrap_or("o painel recusou a operacao")
        .to_string();
    Err(AppError::new(StatusCode::CONFLICT, "erro_negocio_painel", mensagem).with_details(data))
}

// ---------------------------------------------------------------------------
// Infraestrutura
// ---------------------------------------------------------------------------

/// Repete a chamada uma vez depois do login quando o painel responde 401.
macro_rules! login_retry {
    ($st:expr, $expr:expr) => {{
        match $expr.await {
            Err(ApiError::SessionExpired) | Err(ApiError::NotAuthenticated) => {
                $st.ensure_login().await?;
                $expr.await
            }
            outro => outro,
        }
    }};
}

/// `GET /health` — publico, nao toca no painel.
pub async fn health(State(st): State<AppState>) -> Json<Value> {
    let started = Instant::now();
    success(
        started,
        json!({
            "service": SERVICE,
            "version": env!("CARGO_PKG_VERSION"),
            "status": "ok",
            "uptime_secs": st.uptime_secs(),
        }),
    )
}

/// `GET /openapi.json` - JSON real (gerado de `docs/openapi.yaml`).
pub async fn openapi_json() -> Response {
    (
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        OPENAPI_JSON,
    )
        .into_response()
}

/// `GET /openapi.yaml` - fonte YAML embutida no binario.
pub async fn openapi_yaml() -> Response {
    (
        [(header::CONTENT_TYPE, "application/yaml; charset=utf-8")],
        OPENAPI_YAML,
    )
        .into_response()
}

/// `GET /v1/meta` — versao, origem do painel e estado da sessao.
pub async fn meta(State(st): State<AppState>) -> Json<Value> {
    let started = Instant::now();
    let session = st.client.session();
    let origin = st
        .client
        .api_origin()
        .map(|url| url.to_string())
        .unwrap_or_default();
    success(
        started,
        json!({
            "service": SERVICE,
            "version": env!("CARGO_PKG_VERSION"),
            "panel_origin": origin,
            "authenticated": session.authenticated,
            "session": {
                "authenticated": session.authenticated,
                "username": session.username,
                "role": session.role,
                "logged_in_at": session.logged_in_at,
            },
            "auth": {
                "token_required": st.token_required(),
                "dev_mode": !st.token_required(),
            },
            "started_at": st.started_at,
        }),
    )
}

/// `POST /v1/login` — corpo `{ "codigo": "..." }` (codigo so quando ha 2FA).
pub async fn login(State(st): State<AppState>, body: Bytes) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let input: LoginIn = if body.is_empty() {
        LoginIn::default()
    } else {
        serde_json::from_slice(&body).map_err(|err| {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "json_invalido",
                format!("corpo invalido: {err}"),
            )
        })?
    };
    let codigo = input
        .codigo
        .as_deref()
        .map(str::trim)
        .filter(|code| !code.is_empty());
    let outcome = match codigo {
        Some(code) => st.client.login_with_code(code).await?,
        None => st.client.login().await?,
    };
    // O core devolve o desafio 2FA como outcome; a API local expoe como 428
    // para o agente saber que precisa repetir o login com `codigo`.
    if let LoginOutcome::TwoFactorRequired { email_mascarado } = &outcome {
        return Err(merger_core::ApiError::TwoFactorRequired {
            email_mascarado: email_mascarado.clone(),
        }
        .into());
    }
    Ok(success(
        started,
        serde_json::to_value(outcome).unwrap_or(Value::Null),
    ))
}

/// `POST /v1/logout` — encerra a sessao no painel.
pub async fn logout(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    st.client.logout().await?;
    Ok(success(started, json!({ "ok": true })))
}

/// `GET /v1/whoami` — identidade da sessao no painel.
pub async fn whoami(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    if !st.client.is_authenticated() {
        st.ensure_login().await?;
    }
    let who = st.client.whoami().await?;
    Ok(success(started, who))
}

// ---------------------------------------------------------------------------
// Passthrough
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct RawIn {
    /// Metodo HTTP; default `GET`.
    #[serde(default = "default_method")]
    pub method: String,
    /// Caminho absoluto dentro de `/merger/api/*` ou `/api/webhook/*`.
    pub path: String,
    /// Query como objeto `{chave: valor}` ou lista de pares `[["k","v"]]`.
    #[serde(default)]
    pub query: Option<Value>,
    #[serde(default)]
    pub body: Option<Value>,
    /// Obrigatorio `true` em operacoes destrutivas.
    #[serde(default)]
    pub confirm: bool,
}

fn default_method() -> String {
    "GET".to_string()
}

/// `POST /v1/raw` — passthrough autenticado com allowlist e `confirm`.
pub async fn raw(
    State(st): State<AppState>,
    ApiJson(input): ApiJson<RawIn>,
) -> Result<Response, AppError> {
    let started = Instant::now();
    let method = input.method.trim().to_ascii_uppercase();
    if !RAW_METHODS.contains(&method.as_str()) {
        return Err(AppError::bad_request(format!(
            "metodo {method:?} nao suportado (use GET, POST, PUT, PATCH, DELETE ou HEAD)"
        )));
    }
    let req = RawRequest {
        method,
        path: input.path,
        query: parse_query_pairs(input.query)?,
        body: input.body,
        confirm: input.confirm,
    };
    let panel_path = req.path.clone();
    let response = st.call(req).await?;
    let mut resposta = success(
        started,
        serde_json::to_value(response).unwrap_or(Value::Null),
    )
    .into_response();
    // Header interno: a auditoria registra o caminho real do painel e remove
    // o header antes de devolver ao cliente.
    if let Ok(valor) = header_value(&panel_path) {
        resposta.headers_mut().insert("x-audit-panel-path", valor);
    }
    Ok(resposta)
}

fn header_value(
    valor: &str,
) -> Result<axum::http::HeaderValue, axum::http::header::InvalidHeaderValue> {
    axum::http::HeaderValue::from_str(valor)
}

#[derive(Debug, Default, Deserialize)]
pub struct LoginIn {
    #[serde(default)]
    pub codigo: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers de chamada
// ---------------------------------------------------------------------------

async fn get_panel(st: &AppState, path: &str, query: Option<String>) -> Result<Value, AppError> {
    let mut req = RawRequest::get(path);
    req.query = parse_raw_query(query)?;
    let response = st.call(req).await?;
    Ok(response.json_or_text())
}

/// Rotas cujos campos viram query exigem um objeto plano de escalares.
fn validar_campos_escalares(body: &Value) -> Result<(), AppError> {
    if let Some(map) = body.as_object() {
        for (chave, valor) in map {
            if matches!(valor, Value::Array(_) | Value::Object(_)) {
                return Err(AppError::bad_request(format!(
                    "campo {chave} precisa ser escalar"
                )));
            }
        }
    }
    Ok(())
}

fn parse_raw_query(raw: Option<String>) -> Result<Vec<(String, String)>, AppError> {
    let Some(raw) = raw else {
        return Ok(Vec::new());
    };
    let raw = raw.trim_start_matches('?');
    Ok(url::form_urlencoded::parse(raw.as_bytes())
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect())
}

/// Aceita `{"chave": "valor"}` ou `[["chave", "valor"]]` (e
/// `{"chave":"valor"}` dentro de listas). Valores escalares viram string.
fn parse_query_pairs(value: Option<Value>) -> Result<Vec<(String, String)>, AppError> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    match value {
        Value::Null => Ok(Vec::new()),
        Value::Object(map) => {
            let mut pairs = Vec::with_capacity(map.len());
            for (key, value) in map {
                push_pair(&mut pairs, key, value)?;
            }
            Ok(pairs)
        }
        Value::Array(items) => {
            let mut pairs = Vec::with_capacity(items.len());
            for item in items {
                match item {
                    Value::Array(par) if par.len() == 2 => {
                        let key = scalar(&par[0]).ok_or_else(|| {
                            AppError::bad_request("query: chave do par precisa ser escalar")
                        })?;
                        let value = scalar(&par[1]).ok_or_else(|| {
                            AppError::bad_request("query: valor do par precisa ser escalar")
                        })?;
                        pairs.push((key, value));
                    }
                    Value::Object(obj) => {
                        let key = obj
                            .get("chave")
                            .or_else(|| obj.get("key"))
                            .and_then(scalar)
                            .ok_or_else(|| {
                                AppError::bad_request(
                                    "query: objeto precisa de \"chave\" (ou \"key\")",
                                )
                            })?;
                        let value = obj
                            .get("valor")
                            .or_else(|| obj.get("value"))
                            .and_then(scalar)
                            .ok_or_else(|| {
                                AppError::bad_request(
                                    "query: objeto precisa de \"valor\" (ou \"value\")",
                                )
                            })?;
                        pairs.push((key, value));
                    }
                    _ => {
                        return Err(AppError::bad_request(
                            "query: use objeto {chave: valor} ou lista de pares [[\"chave\",\"valor\"]]",
                        ))
                    }
                }
            }
            Ok(pairs)
        }
        _ => Err(AppError::bad_request(
            "query deve ser objeto {chave: valor} ou lista de pares",
        )),
    }
}

fn push_pair(pairs: &mut Vec<(String, String)>, key: String, value: Value) -> Result<(), AppError> {
    match value {
        Value::Array(items) => {
            for item in items {
                let item = scalar(&item).ok_or_else(|| {
                    AppError::bad_request(format!("query.{key} precisa conter valores escalares"))
                })?;
                pairs.push((key.clone(), item));
            }
            Ok(())
        }
        outro => {
            let value = scalar(&outro).ok_or_else(|| {
                AppError::bad_request(format!("query.{key} precisa ser um valor escalar"))
            })?;
            pairs.push((key, value));
            Ok(())
        }
    }
}

fn scalar(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(flag) => Some(flag.to_string()),
        Value::Null => Some(String::new()),
        Value::Array(_) | Value::Object(_) => None,
    }
}

// ---------------------------------------------------------------------------
// Endpoints tipados (GET)
// ---------------------------------------------------------------------------

/// GET de lista, repassando a query string ao painel.
macro_rules! list_route {
    ($name:ident, $path:literal) => {
        pub async fn $name(
            State(st): State<AppState>,
            RawQuery(query): RawQuery,
        ) -> Result<Json<Value>, AppError> {
            let started = Instant::now();
            let data = get_panel(&st, $path, query).await?;
            let data = checar_erro_negocio(data)?;
            Ok(success(started, data))
        }
    };
}

/// GET sem query.
macro_rules! fixed_route {
    ($name:ident, $path:literal) => {
        pub async fn $name(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
            let started = Instant::now();
            let data = get_panel(&st, $path, None).await?;
            let data = checar_erro_negocio(data)?;
            Ok(success(started, data))
        }
    };
}

list_route!(revendas, "/merger/api/cliente/revendas");
fixed_route!(revendas_arvore, "/merger/api/cliente/revendas/arvore");

/// `POST /v1/revendas/criar` — cria sub-revenda e aplica o vencimento.
///
/// Corpo = contrato do painel (`username`, `password`, `credits`, `unlimited`,
/// `pode_criar_revenda`, `pode_criar_ilimitado`, `pode_conectar_whatsapp`) +
/// `dias_painel` opcional (default 30; `0` nao mexe no vencimento).
pub async fn revendas_criar(
    State(st): State<AppState>,
    ApiJson(mut body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let dias =
        match body.get("dias_painel") {
            None => 30,
            Some(valor) => valor.as_u64().filter(|dias| *dias <= 3650).ok_or_else(|| {
                AppError::bad_request("dias_painel deve ser inteiro entre 0 e 3650")
            })? as u32,
        };
    if let Some(obj) = body.as_object_mut() {
        obj.remove("dias_painel");
    }

    let data = match st
        .client
        .revendas_criar_com_vencimento(body.clone(), dias)
        .await
    {
        Err(ApiError::SessionExpired) | Err(ApiError::NotAuthenticated) => {
            st.ensure_login().await?;
            st.client.revendas_criar_com_vencimento(body, dias).await
        }
        outro => outro,
    }
    .map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `PUT /v1/revendas/editar` — acesso, senha, permissoes ou creditos.
pub async fn revendas_editar(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = match st.client.revendas_editar(body.clone()).await {
        Err(ApiError::SessionExpired) | Err(ApiError::NotAuthenticated) => {
            st.ensure_login().await?;
            st.client.revendas_editar(body).await
        }
        outro => outro,
    }
    .map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `PATCH /v1/revendas/toggle` — liga/desliga a revenda e a subarvore.
pub async fn revendas_toggle(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let id = body
        .get("id")
        .and_then(Value::as_i64)
        .filter(|id| *id > 0)
        .ok_or_else(|| AppError::bad_request("campo id obrigatorio (inteiro positivo)"))?;
    let habilitar = body
        .get("enabled")
        .and_then(|valor| match valor {
            Value::Bool(flag) => Some(*flag),
            Value::Number(numero) => numero.as_i64().map(|n| n == 1),
            _ => None,
        })
        .ok_or_else(|| AppError::bad_request("campo enabled obrigatorio (0/1 ou booleano)"))?;

    let data = match st.client.revendas_toggle(id, habilitar).await {
        Err(ApiError::SessionExpired) | Err(ApiError::NotAuthenticated) => {
            st.ensure_login().await?;
            st.client.revendas_toggle(id, habilitar).await
        }
        outro => outro,
    }
    .map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// Query de `DELETE /v1/revendas/remover`.
#[derive(Debug, Deserialize)]
pub struct RemoverRevendaQuery {
    pub id: i64,
    #[serde(default)]
    pub confirm: bool,
}

/// `DELETE /v1/revendas/remover?id=N&confirm=true` — remove em cascata.
pub async fn revendas_remover(
    State(st): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<RemoverRevendaQuery>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = match st.client.revendas_remover(query.id, query.confirm).await {
        Err(ApiError::SessionExpired) | Err(ApiError::NotAuthenticated) => {
            st.ensure_login().await?;
            st.client.revendas_remover(query.id, query.confirm).await
        }
        outro => outro,
    }
    .map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

list_route!(usuarios, "/merger/api/cliente/sigma_users");
fixed_route!(planos, "/merger/api/cliente/planos");
fixed_route!(creditos, "/merger/api/cliente/creditos");

list_route!(pedidos, "/merger/api/pedidos/listar");
list_route!(reportes, "/merger/api/reportes/listar");

list_route!(
    catalogo_pendencias,
    "/merger/api/cliente/catalogo/pendencias"
);
list_route!(catalogo_jogos, "/merger/api/cliente/agenda/jogos");

/// `GET /v1/tickets` — usa a rota de cliente ou de admin conforme o papel
/// configurado (`ZENITH_ROLE`).
pub async fn tickets(
    State(st): State<AppState>,
    RawQuery(query): RawQuery,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let path = match st.client.config().role {
        merger_core::PanelRole::Admin => "/merger/api/admin/tickets/listar",
        merger_core::PanelRole::Franchise => "/merger/api/cliente/tickets/listar",
    };
    let data = get_panel(&st, path, query).await?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

list_route!(avisos, "/merger/api/avisos");

fixed_route!(stats, "/merger/api/stats");
fixed_route!(dashboard, "/merger/api/dashboard_financeiro");

// ---------------------------------------------------------------------------
// Endpoints tipados (POST)
// ---------------------------------------------------------------------------

/// POST tipado: valida no core, faz login sob demanda e traduz erro de negocio.
///
/// Diferente do passthrough, aqui a logica do `merger-core` (builders de
/// corpo/query e travas locais) **sempre** roda. Era o furo do P3: a macro
/// antiga chamava `raw` e pulava, por exemplo, o fallback de saldo no
/// rebaixamento de revenda ilimitada.
macro_rules! typed_post {
    ($name:ident, $doc:literal, $metodo:ident) => {
        #[doc = $doc]
        pub async fn $name(
            State(st): State<AppState>,
            ApiJson(body): ApiJson<Value>,
        ) -> Result<Json<Value>, AppError> {
            let started = Instant::now();
            let data = login_retry!(st, st.client.$metodo(body.clone())).map_err(AppError::from)?;
            let data = checar_erro_negocio(data)?;
            Ok(success(started, data))
        }
    };
}

typed_post!(
    revendas_renovar,
    "Renova uma sub-revenda. Corpo `{id, vencimento_atual}`.",
    revendas_renovar
);
typed_post!(
    revendas_nota,
    "Grava nota/contato/vencimento na revenda.",
    revendas_nota
);
typed_post!(
    revendas_unlimited,
    "Liga/desliga o modo ilimitado (rebaixar sem saldo reaplica o saldo atual).",
    revenda_unlimited
);
/// Cria uma linha/cliente final. Campos viram **query string** no painel.
pub async fn usuarios_criar(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    validar_campos_escalares(&body)?;
    let data = login_retry!(st, st.client.usuarios_criar(body.clone())).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}
typed_post!(
    usuarios_editar,
    "Edita dados de um cliente.",
    usuarios_editar
);
/// Liga/desliga o cliente. Corpo `{id, acao}`; campos viram query.
pub async fn usuarios_toggle(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    validar_campos_escalares(&body)?;
    let data = login_retry!(st, st.client.usuarios_toggle(body.clone())).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}
typed_post!(
    usuarios_renovar,
    "Renova o cliente com um plano.",
    usuarios_renovar
);
typed_post!(
    usuarios_estender,
    "Estende o vencimento sem trocar o plano.",
    usuarios_estender
);

/// Remove um cliente. Corpo `{id, confirm: true}`.
pub async fn usuarios_delete(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    validar_campos_escalares(&body)?;
    let confirm = body
        .get("confirm")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let data = login_retry!(st, st.client.usuarios_delete(body.clone(), confirm))
        .map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

typed_post!(
    pedidos_criar,
    "Cria um pedido de conteudo (valida tipo/tmdb/audio no core).",
    pedidos_criar
);
typed_post!(
    reportes_criar,
    "Reporta um canal/filme com problema (whitelist de motivos no core).",
    reportes_criar
);

// ---------------------------------------------------------------------------
// Tickets (JSON na API local, multipart no painel) e upload generico
// ---------------------------------------------------------------------------

/// Limite de cada arquivo aceito em `/v1/upload`.
const MAX_UPLOAD_BYTES: usize = 25 * 1024 * 1024;

/// `POST /v1/tickets/criar` — abre um chamado. Corpo `{assunto, texto}`.
pub async fn tickets_criar(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    st.ensure_login().await?;
    let data = match st.client.tickets_criar(body.clone(), None).await {
        Err(ApiError::SessionExpired) | Err(ApiError::NotAuthenticated) => {
            st.ensure_login().await?;
            st.client.tickets_criar(body, None).await
        }
        other => other,
    }
    .map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// Query de `POST /v1/tickets/fechar`.
#[derive(Debug, Deserialize)]
pub struct FecharTicketQuery {
    pub id: Option<i64>,
}

/// `POST /v1/tickets/fechar?id=N` - fecha o proprio chamado.
///
/// O painel exige `id` na **query**; mandar no corpo devolve `sem_acesso`.
pub async fn tickets_fechar(
    State(st): State<AppState>,
    RawQuery(query): RawQuery,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let id = query
        .as_deref()
        .and_then(|raw| {
            url::form_urlencoded::parse(raw.as_bytes())
                .find(|(chave, _)| chave == "id")
                .and_then(|(_, valor)| valor.trim().parse::<i64>().ok())
        })
        .filter(|id| *id > 0)
        .ok_or_else(|| AppError::bad_request("query id obrigatoria (inteiro positivo)"))?;

    let data = login_retry!(st, st.client.tickets_fechar(id)).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `POST /v1/tickets/responder` - responde um chamado. Corpo `{id, texto}`.
pub async fn tickets_responder(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    st.ensure_login().await?;
    let data = match st.client.tickets_responder(body.clone(), None).await {
        Err(ApiError::SessionExpired) | Err(ApiError::NotAuthenticated) => {
            st.ensure_login().await?;
            st.client.tickets_responder(body, None).await
        }
        other => other,
    }
    .map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `POST /v1/upload` — encaminha `multipart/form-data` ao painel.
///
/// Campos texto vao como estao; o campo `path` diz a rota do painel e
/// `confirm` aceita `1`/`true`/`yes`. Partes com `filename` viram arquivos.
pub async fn upload(
    State(st): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let mut path: Option<String> = None;
    let mut confirm = false;
    let mut fields: Vec<(String, String)> = Vec::new();
    let mut files: Vec<UploadFile> = Vec::new();

    while let Some(part) = multipart
        .next_field()
        .await
        .map_err(|err| AppError::bad_request(format!("multipart invalido: {err}")))?
    {
        let name = part.name().unwrap_or_default().to_string();
        let filename = part.file_name().map(str::to_string);
        let content_type = part.content_type().map(str::to_string);

        if name == "path" {
            path = Some(
                part.text()
                    .await
                    .map_err(|err| AppError::bad_request(format!("path invalido: {err}")))?,
            );
        } else if name == "confirm" {
            let valor = part
                .text()
                .await
                .map_err(|err| AppError::bad_request(format!("confirm invalido: {err}")))?;
            confirm = matches!(
                valor.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes"
            );
        } else if let Some(filename) = filename {
            let bytes = part
                .bytes()
                .await
                .map_err(|err| AppError::bad_request(format!("arquivo invalido: {err}")))?;
            if bytes.len() > MAX_UPLOAD_BYTES {
                return Err(AppError::bad_request(format!(
                    "arquivo {filename} passa do limite de {} MB",
                    MAX_UPLOAD_BYTES / (1024 * 1024)
                )));
            }
            files.push(UploadFile {
                field: name,
                filename,
                mime: content_type.unwrap_or_else(|| "application/octet-stream".to_string()),
                bytes: bytes.to_vec(),
            });
        } else {
            let valor = part
                .text()
                .await
                .map_err(|err| AppError::bad_request(format!("campo {name} invalido: {err}")))?;
            fields.push((name, valor));
        }
    }

    let path = path
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .ok_or_else(|| AppError::bad_request("campo multipart path e obrigatorio"))?;

    if !upload_destino_permitido(&path) {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "destino_upload_invalido",
            format!(
                "{path} nao existe no catalogo de rotas do painel; \
                 se for uma rota nova, libere em ZENITH_UPLOAD_PATHS"
            ),
        ));
    }

    let resposta = st
        .call_upload(RawUpload {
            path: path.clone(),
            fields,
            files,
            confirm,
        })
        .await?;

    // O catch-all do painel responde 200 vazio para rota inexistente. Sem esta
    // checagem, o upload "funcionaria" sem fazer nada.
    if resposta.ok && resposta.text.trim().is_empty() {
        return Err(AppError::new(
            StatusCode::BAD_GATEWAY,
            "erro_painel_resposta_vazia",
            format!(
                "o painel respondeu {} com corpo vazio para {path}; \
                 a rota pode nao existir ou nao aceitar upload",
                resposta.status
            ),
        ));
    }

    Ok(success(started, resposta.json_or_text()))
}

// ---------------------------------------------------------------------------
// Revendas: transferencia e convite/adesao
// ---------------------------------------------------------------------------

/// Query de `GET /v1/revendas/transferir/destinos`.
#[derive(Debug, Deserialize)]
pub struct TransferirDestinosQuery {
    pub origem: Option<String>,
}

/// `GET /v1/revendas/transferir/destinos?origem=USER` — pais possiveis.
pub async fn revendas_transferir_destinos(
    State(st): State<AppState>,
    RawQuery(query): RawQuery,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let origem = query
        .as_deref()
        .and_then(|raw| {
            url::form_urlencoded::parse(raw.as_bytes())
                .find(|(chave, _)| chave == "origem")
                .map(|(_, valor)| valor.trim().to_string())
        })
        .filter(|valor| !valor.is_empty())
        .ok_or_else(|| AppError::bad_request("query origem e obrigatoria"))?;

    let data =
        login_retry!(st, st.client.transfer_revenda_destinos(&origem)).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `POST /v1/revendas/transferir` — move a revenda para outro pai.
pub async fn revendas_transferir(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data =
        login_retry!(st, st.client.transfer_revenda(body.clone())).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `GET /v1/revendas/convite` — link de auto-cadastro da franquia.
pub async fn revendas_convite(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.convite()).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `POST /v1/revendas/convite/regenerar` — invalida o link antigo.
pub async fn revendas_convite_regenerar(
    State(st): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.convite_regenerar()).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `GET /v1/revendas/convite/adesao/pendentes` — adesoes manuais pendentes.
pub async fn revendas_convite_adesao_pendentes(
    State(st): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.convite_adesao_pendentes()).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `POST /v1/revendas/convite/adesao/confirmar` — confirma o recebimento.
pub async fn revendas_convite_adesao_confirmar(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let pedido_id = body
        .get("pedido_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|valor| !valor.is_empty())
        .ok_or_else(|| AppError::bad_request("campo pedido_id e obrigatorio"))?
        .to_string();

    let data =
        login_retry!(st, st.client.convite_adesao_confirmar(&pedido_id)).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `GET /v1/revendas/convite/mensalista-config` — oferta de mensalista.
pub async fn revendas_convite_config(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.convite_mensalista_config()).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `POST /v1/revendas/convite/mensalista-config` — salva a oferta.
pub async fn revendas_convite_config_salvar(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.convite_mensalista_config_salvar(body.clone()))
        .map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

// ---------------------------------------------------------------------------
// Fallbacks
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Apps Ativa (Ative App) — ativacao paga, saldo e historico
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct AtivacoesAtivaQuery {
    pub status: Option<String>,
}

/// `GET /v1/apps-ativa/saldo` — saldo em R$ (centavos).
pub async fn apps_ativa_saldo(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.apps_ativa_saldo()).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `GET /v1/apps-ativa/apps` — catalogo de aplicativos.
pub async fn apps_ativa_apps(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.apps_ativa_apps()).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `GET /v1/apps-ativa/historico` — movimentacoes de credito.
pub async fn apps_ativa_historico(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.apps_ativa_historico()).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `GET /v1/apps-ativa/ativacoes?status=` — ativacoes com status.
pub async fn apps_ativa_ativacoes(
    State(st): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<AtivacoesAtivaQuery>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let status = query.status.as_deref();
    let data = login_retry!(st, st.client.apps_ativa_ativacoes(status)).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `POST /v1/apps-ativa/ativar` — ativa um MAC (normaliza antes do debito).
pub async fn apps_ativa_ativar(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data =
        login_retry!(st, st.client.apps_ativa_ativar(body.clone())).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

// ---------------------------------------------------------------------------
// Apps Ibo (msproapp) — contas, apps, clientes, ativacao e atribuicao
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct ContaIboQuery {
    pub conta: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ExcluirIboQuery {
    pub conta: Option<i64>,
    #[serde(default)]
    pub confirm: bool,
}

fn conta_obrigatoria(conta: Option<i64>) -> Result<i64, AppError> {
    conta
        .filter(|conta| *conta > 0)
        .ok_or_else(|| AppError::bad_request("query conta obrigatoria (inteiro positivo)"))
}

/// `GET /v1/apps-ibo/contas` — contas visiveis (sanitizadas).
pub async fn apps_ibo_contas(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.apps_ibo_contas()).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `GET /v1/apps-ibo/apps?conta=N` — aplicativos da conta.
pub async fn apps_ibo_apps(
    State(st): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<ContaIboQuery>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let conta = conta_obrigatoria(query.conta)?;
    let data = login_retry!(st, st.client.apps_ibo_apps(conta)).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `GET /v1/apps-ibo/clientes?conta=N` — clientes da conta.
pub async fn apps_ibo_clientes(
    State(st): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<ContaIboQuery>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let conta = conta_obrigatoria(query.conta)?;
    let data = login_retry!(st, st.client.apps_ibo_clientes(conta)).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `POST /v1/apps-ibo/ativar` — ativa um MAC (modo M3U8 ou XTeam).
pub async fn apps_ibo_ativar(
    State(st): State<AppState>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.apps_ibo_ativar(body.clone())).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `DELETE /v1/apps-ibo/clientes/{id}?conta=N&confirm=true` — remove cliente.
pub async fn apps_ibo_excluir(
    State(st): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Query(query): axum::extract::Query<ExcluirIboQuery>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let conta = conta_obrigatoria(query.conta)?;
    let data = login_retry!(st, st.client.apps_ibo_excluir(&id, conta, query.confirm))
        .map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// `POST /v1/apps-ibo/clientes/{id}/atribuir?conta=N` — `{dono}`.
pub async fn apps_ibo_atribuir(
    State(st): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Query(query): axum::extract::Query<ContaIboQuery>,
    ApiJson(body): ApiJson<Value>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let conta = conta_obrigatoria(query.conta)?;
    let dono = body
        .get("dono")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|dono| !dono.is_empty())
        .ok_or_else(|| AppError::bad_request("campo dono obrigatorio (username da revenda)"))?;
    let data =
        login_retry!(st, st.client.apps_ibo_atribuir(conta, &id, dono)).map_err(AppError::from)?;
    let data = checar_erro_negocio(data)?;
    Ok(success(started, data))
}

/// Fallback de rota inexistente, no envelope de erro.
pub async fn not_found() -> AppError {
    AppError::new(
        StatusCode::NOT_FOUND,
        "rota_nao_encontrada",
        "rota nao existe nesta API",
    )
}

/// Fallback de metodo nao permitido, no envelope de erro.
pub async fn method_not_allowed() -> AppError {
    AppError::new(
        StatusCode::METHOD_NOT_ALLOWED,
        "metodo_nao_permitido",
        "metodo HTTP nao permitido para esta rota",
    )
}

#[cfg(test)]
mod testes {
    use super::checar_erro_negocio;
    use serde_json::json;

    #[test]
    fn recusa_de_negocio_vira_409_com_corpo_do_painel() {
        let erro = checar_erro_negocio(json!({
            "ok": false,
            "erro": "Sem creditos de pedido disponiveis"
        }))
        .expect_err("deve virar erro");
        assert_eq!(erro.status, axum::http::StatusCode::CONFLICT);
        assert_eq!(erro.code, "erro_negocio_painel");
        assert_eq!(erro.message, "Sem creditos de pedido disponiveis");
        assert!(erro.details.is_some());
    }

    #[test]
    fn sucesso_passa_intacto() {
        let corpo = json!({"ok": true, "id": 7});
        let saida = checar_erro_negocio(corpo.clone()).expect("sucesso");
        assert_eq!(saida, corpo);
    }
}
