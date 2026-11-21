//! Handlers da API local.
//!
//! Todos os endpoints de dominio chamam o painel pelo passthrough
//! `MergerClient::raw` (via [`AppState::call`]), com login sob demanda. Assim o
//! servidor nao depende dos atalhos tipados de `merger-core` estarem prontos.

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
const SERVICE: &str = "zenith4k-api";

/// Rotas aceitas no passthrough (/v1/raw).
const RAW_METHODS: [&str; 6] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD"];

// ---------------------------------------------------------------------------
// Infraestrutura
// ---------------------------------------------------------------------------

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

/// `GET /openapi.json` — serve `docs/openapi.yaml` embutido no binario.
pub async fn openapi() -> Response {
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
) -> Result<Json<Value>, AppError> {
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
    let response = st.call(req).await?;
    Ok(success(
        started,
        serde_json::to_value(response).unwrap_or(Value::Null),
    ))
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

async fn post_panel(st: &AppState, path: &str, body: Value) -> Result<Value, AppError> {
    let (body, confirm) = strip_confirm(body);
    let mut req = RawRequest::post(path, body);
    req.confirm = confirm;
    let response = st.call(req).await?;
    Ok(response.json_or_text())
}

/// POST em que o painel le os campos pela **query string** (ex.:
/// `sigma_users/criar`, `toggle`, `delete`), e nao pelo corpo JSON.
async fn post_panel_query(st: &AppState, path: &str, body: Value) -> Result<Value, AppError> {
    let (body, confirm) = strip_confirm(body);
    let mut req = RawRequest::new("POST", path);
    req.query = body_to_query(&body)?;
    req.confirm = confirm;
    let response = st.call(req).await?;
    Ok(response.json_or_text())
}

/// Converte um corpo JSON plano em pares de query (`{"id": 7}` -> `id=7`).
fn body_to_query(body: &Value) -> Result<Vec<(String, String)>, AppError> {
    let Value::Object(map) = body else {
        return Err(AppError::bad_request(
            "corpo precisa ser um objeto JSON com campos escalares",
        ));
    };
    let mut pairs = Vec::with_capacity(map.len());
    for (key, value) in map {
        push_pair(&mut pairs, key.clone(), value.clone())?;
    }
    Ok(pairs)
}

/// Remove `confirm` do corpo antes de enviar ao painel.
fn strip_confirm(body: Value) -> (Value, bool) {
    match body {
        Value::Object(mut map) => {
            let confirm = map.get("confirm").and_then(Value::as_bool).unwrap_or(false);
            map.remove("confirm");
            (Value::Object(map), confirm)
        }
        outro => (outro, false),
    }
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
    Ok(success(started, data))
}

list_route!(avisos, "/merger/api/avisos");

fixed_route!(stats, "/merger/api/stats");
fixed_route!(dashboard, "/merger/api/dashboard_financeiro");

// ---------------------------------------------------------------------------
// Endpoints tipados (POST)
// ---------------------------------------------------------------------------

/// POST com corpo JSON livre. `confirm` e retirado do corpo e usado na trava
/// local de operacoes destrutivas.
macro_rules! post_route {
    ($name:ident, $path:literal, $doc:literal) => {
        #[doc = $doc]
        pub async fn $name(
            State(st): State<AppState>,
            ApiJson(body): ApiJson<Value>,
        ) -> Result<Json<Value>, AppError> {
            let started = Instant::now();
            let data = post_panel(&st, $path, body).await?;
            Ok(success(started, data))
        }
    };
}

/// POST cujos campos o painel espera na query string.
macro_rules! post_query_route {
    ($name:ident, $path:literal, $doc:literal) => {
        #[doc = $doc]
        pub async fn $name(
            State(st): State<AppState>,
            ApiJson(body): ApiJson<Value>,
        ) -> Result<Json<Value>, AppError> {
            let started = Instant::now();
            let data = post_panel_query(&st, $path, body).await?;
            Ok(success(started, data))
        }
    };
}

post_route!(
    revendas_renovar,
    "/merger/api/cliente/revendas/renovar",
    "Renova uma sub-revenda."
);
post_route!(
    revendas_nota,
    "/merger/api/cliente/revendas/nota",
    "Grava uma nota interna na revenda."
);
post_route!(
    revendas_unlimited,
    "/merger/api/cliente/revenda_unlimited",
    "Liga/desliga o modo ilimitado de uma revenda."
);
post_query_route!(
    usuarios_criar,
    "/merger/api/cliente/sigma_users/criar",
    "Cria uma linha/cliente final. Campos na query do painel: username, password, max_connections, dias, is_trial, bouquet, servidor_id."
);
post_route!(
    usuarios_editar,
    "/merger/api/cliente/sigma_users/editar",
    "Edita dados de um cliente."
);
post_query_route!(
    usuarios_toggle,
    "/merger/api/cliente/sigma_users/toggle",
    "Liga/desliga o cliente. Corpo {\"id\": 7, \"acao\": \"bloquear\"|\"desbloquear\"}."
);
post_route!(
    usuarios_renovar,
    "/merger/api/cliente/sigma_users/renovar",
    "Renova o cliente com um plano."
);
post_route!(
    usuarios_estender,
    "/merger/api/cliente/sigma_users/estender",
    "Estende o vencimento sem trocar o plano."
);
post_query_route!(
    usuarios_delete,
    "/merger/api/cliente/sigma_users/delete",
    "Remove um cliente. Corpo {\"id\": 7, \"confirm\": true}."
);
post_route!(
    pedidos_criar,
    "/merger/api/pedidos/criar",
    "Cria um pedido de conteudo."
);
post_route!(
    reportes_criar,
    "/merger/api/reportes/criar",
    "Reporta um canal/filme com problema."
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
    Ok(success(started, data))
}

/// `POST /v1/tickets/responder` — responde um chamado. Corpo `{id, texto}`.
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

    let resposta = st
        .call_upload(RawUpload {
            path,
            fields,
            files,
            confirm,
        })
        .await?;
    Ok(success(started, resposta.json_or_text()))
}

// ---------------------------------------------------------------------------
// Revendas: transferencia e convite/adesao
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
    Ok(success(started, data))
}

/// `GET /v1/revendas/convite` — link de auto-cadastro da franquia.
pub async fn revendas_convite(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.convite()).map_err(AppError::from)?;
    Ok(success(started, data))
}

/// `POST /v1/revendas/convite/regenerar` — invalida o link antigo.
pub async fn revendas_convite_regenerar(
    State(st): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.convite_regenerar()).map_err(AppError::from)?;
    Ok(success(started, data))
}

/// `GET /v1/revendas/convite/adesao/pendentes` — adesoes manuais pendentes.
pub async fn revendas_convite_adesao_pendentes(
    State(st): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.convite_adesao_pendentes()).map_err(AppError::from)?;
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
    Ok(success(started, data))
}

/// `GET /v1/revendas/convite/mensalista-config` — oferta de mensalista.
pub async fn revendas_convite_config(State(st): State<AppState>) -> Result<Json<Value>, AppError> {
    let started = Instant::now();
    let data = login_retry!(st, st.client.convite_mensalista_config()).map_err(AppError::from)?;
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
    Ok(success(started, data))
}

// ---------------------------------------------------------------------------
// Fallbacks
// ---------------------------------------------------------------------------

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
