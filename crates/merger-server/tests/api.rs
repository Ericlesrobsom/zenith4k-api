//! Testes de integracao do servidor, sem painel real.
//!
//! Nenhum teste aqui toca a rede: os endpoints exercitados sao publicos
//! (`/health`, `/openapi.json`), locais (`/v1/meta`) ou falham antes de
//! qualquer I/O (allowlist e trava de confirmacao).

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::{to_bytes, Body};
use axum::extract::ConnectInfo;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::Router;
use merger_core::{ApiError, ClientConfig, MergerClient};
use merger_server::app;
use merger_server::error::AppError;
use merger_server::state::AppState;
use serde_json::Value;
use tower::ServiceExt;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn temp_dir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("zenith4k-api-teste-{nanos}-{id}"))
}

fn state(token: Option<&str>, audit_dir: &Path) -> AppState {
    let cfg = ClientConfig::new("http://127.0.0.1:9/", "teste", "teste").expect("config de teste");
    let client = MergerClient::new(cfg).expect("cliente de teste");
    AppState::new(client, token.map(str::to_string), audit_dir)
}

fn get(uri: &str, token: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder().uri(uri).method("GET");
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    builder.body(Body::empty()).expect("request de teste")
}

fn post_json(uri: &str, token: Option<&str>, body: &str) -> Request<Body> {
    let mut builder = Request::builder()
        .uri(uri)
        .method("POST")
        .header("content-type", "application/json");
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    builder
        .body(Body::from(body.to_string()))
        .expect("request de teste")
}

fn with_peer(mut req: Request<Body>, ip: [u8; 4]) -> Request<Body> {
    req.extensions_mut()
        .insert(ConnectInfo(SocketAddr::from((ip, 40_000))));
    req
}

async fn send(router: Router, req: Request<Body>) -> (StatusCode, Value) {
    let response = router.oneshot(req).await.expect("resposta do servidor");
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 2 * 1024 * 1024)
        .await
        .expect("corpo da resposta");
    let body = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()))
    };
    (status, body)
}

#[tokio::test]
async fn mapeamento_de_erros_do_painel_para_http() {
    let casos = [
        (ApiError::NotAuthenticated, 401, "nao_autenticado"),
        (ApiError::SessionExpired, 401, "sessao_expirada"),
        (
            ApiError::TwoFactorRequired {
                email_mascarado: None,
            },
            428,
            "dois_fatores_necessario",
        ),
        (ApiError::InvalidTwoFactor, 401, "codigo_2fa_invalido"),
        (ApiError::Forbidden("x".into()), 403, "sem_permissao"),
        (ApiError::NotFound, 404, "nao_encontrado"),
        (
            ApiError::ConfirmationRequired("x".into()),
            409,
            "confirmacao_necessaria",
        ),
        (
            ApiError::RateLimited {
                retry_after_secs: Some(30),
            },
            429,
            "rate_limit",
        ),
        (
            ApiError::PathNotAllowed("x".into()),
            400,
            "caminho_nao_permitido",
        ),
        (
            ApiError::Config("corpo invalido".into()),
            400,
            "parametros_invalidos",
        ),
        (
            ApiError::Panel {
                status: 500,
                body: "x".into(),
            },
            502,
            "erro_painel",
        ),
    ];

    for (erro, status, code) in casos {
        let resposta = AppError::from(erro).into_response();
        assert_eq!(resposta.status(), StatusCode::from_u16(status).unwrap());
        let bytes = to_bytes(resposta.into_body(), 4096)
            .await
            .expect("corpo do erro");
        let body: Value = serde_json::from_slice(&bytes).expect("json do erro");
        assert_eq!(body["error"]["code"], code);
        assert_eq!(body["ok"], false);
    }
}

#[tokio::test]
async fn health_e_publico_e_usa_envelope() {
    let dir = temp_dir();
    let (status, body) = send(app(state(Some("segredo"), &dir)), get("/health", None)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert_eq!(body["data"]["status"], "ok");
    assert!(body["data"]["version"].is_string());
    assert!(body["meta"]["elapsed_ms"].is_number());
}

#[tokio::test]
async fn meta_exige_token_quando_configurado() {
    let dir = temp_dir();
    let router = app(state(Some("segredo"), &dir));

    let (status, body) = send(router.clone(), get("/v1/meta", None)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["ok"], false);
    assert_eq!(body["error"]["code"], "nao_autorizado");

    let (status, body) = send(router, get("/v1/meta", Some("segredo"))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert_eq!(body["data"]["authenticated"], false);
    assert_eq!(body["data"]["auth"]["token_required"], true);
    assert_eq!(body["data"]["auth"]["dev_mode"], false);
    assert!(body["data"]["panel_origin"]
        .as_str()
        .unwrap_or_default()
        .starts_with("http://127.0.0.1:9"));
}

#[tokio::test]
async fn modo_dev_aceita_somente_loopback() {
    let dir = temp_dir();
    let router = app(state(None, &dir));

    let (status, body) = send(router.clone(), get("/v1/meta", None)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "token_obrigatorio");

    let (status, body) = send(
        router.clone(),
        with_peer(get("/v1/meta", None), [127, 0, 0, 1]),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["auth"]["dev_mode"], true);

    let (status, body) = send(router, with_peer(get("/v1/meta", None), [10, 0, 0, 7])).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "acesso_negado");
}

#[tokio::test]
async fn rota_protegida_sem_token_devolve_401() {
    let dir = temp_dir();
    let (status, body) = send(app(state(Some("segredo"), &dir)), get("/v1/revendas", None)).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "nao_autorizado");
}

#[tokio::test]
async fn openapi_json_e_json_de_verdade() {
    let dir = temp_dir();
    let response = app(state(Some("segredo"), &dir))
        .oneshot(get("/openapi.json", None))
        .await
        .expect("resposta do servidor");

    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    assert!(
        content_type.contains("application/json"),
        "content-type: {content_type}"
    );

    let bytes = to_bytes(response.into_body(), 4 * 1024 * 1024)
        .await
        .expect("corpo do openapi");
    let doc: Value = serde_json::from_slice(&bytes).expect("JSON valido");
    assert_eq!(doc["openapi"], "3.1.0");
    assert!(doc["paths"]["/v1/raw"].is_object());
}

#[tokio::test]
async fn openapi_yaml_continua_disponivel() {
    let dir = temp_dir();
    let response = app(state(Some("segredo"), &dir))
        .oneshot(get("/openapi.yaml", None))
        .await
        .expect("resposta do servidor");

    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    assert!(
        content_type.contains("yaml"),
        "content-type: {content_type}"
    );

    let bytes = to_bytes(response.into_body(), 4 * 1024 * 1024)
        .await
        .expect("corpo do openapi");
    let text = String::from_utf8(bytes.to_vec()).expect("yaml utf-8");
    assert!(text.contains("openapi: 3.1.0"));
}

#[tokio::test]
async fn fallback_404_usa_envelope() {
    let dir = temp_dir();
    let (status, body) = send(app(state(Some("segredo"), &dir)), get("/nao-existe", None)).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["ok"], false);
    assert_eq!(body["error"]["code"], "rota_nao_encontrada");
}

#[tokio::test]
async fn metodo_nao_permitido_usa_envelope() {
    let dir = temp_dir();
    let (status, body) = send(
        app(state(Some("segredo"), &dir)),
        post_json("/health", None, "{}"),
    )
    .await;

    assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(body["error"]["code"], "metodo_nao_permitido");
}

#[tokio::test]
async fn raw_bloqueia_caminho_fora_da_allowlist() {
    let dir = temp_dir();
    let (status, body) = send(
        app(state(Some("segredo"), &dir)),
        post_json(
            "/v1/raw",
            Some("segredo"),
            r#"{"method":"GET","path":"https://evil.example/x"}"#,
        ),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "caminho_nao_permitido");
}

#[tokio::test]
async fn raw_exige_confirm_em_operacao_destrutiva() {
    let dir = temp_dir();
    let (status, body) = send(
        app(state(Some("segredo"), &dir)),
        post_json(
            "/v1/raw",
            Some("segredo"),
            r#"{"method":"POST","path":"/merger/api/cliente/sigma_users/delete","body":{"username":"x"}}"#,
        ),
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], "confirmacao_necessaria");
}

#[tokio::test]
async fn raw_recusa_metodo_invalido() {
    let dir = temp_dir();
    let (status, body) = send(
        app(state(Some("segredo"), &dir)),
        post_json(
            "/v1/raw",
            Some("segredo"),
            r#"{"method":"BREW","path":"/merger/api/stats"}"#,
        ),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "requisicao_invalida");
}

#[tokio::test]
async fn login_com_json_invalido_devolve_400() {
    let dir = temp_dir();
    let (status, body) = send(
        app(state(Some("segredo"), &dir)),
        post_json("/v1/login", Some("segredo"), "{nope"),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "json_invalido");
}

#[tokio::test]
async fn delete_tipado_sem_confirm_devolve_409() {
    let dir = temp_dir();
    let (status, body) = send(
        app(state(Some("segredo"), &dir)),
        post_json("/v1/usuarios/delete", Some("segredo"), r#"{"id":1}"#),
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], "confirmacao_necessaria");
}

#[tokio::test]
async fn criar_usuario_recusa_campo_nao_escalar() {
    let dir = temp_dir();
    let (status, body) = send(
        app(state(Some("segredo"), &dir)),
        post_json(
            "/v1/usuarios/criar",
            Some("segredo"),
            r#"{"username":"x","password":"y","extra":{"a":1}}"#,
        ),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "requisicao_invalida");
}

#[tokio::test]
async fn auditoria_grava_jsonl_sem_corpo() {
    let dir = temp_dir();
    let (status, _) = send(app(state(Some("segredo"), &dir)), get("/health", None)).await;
    assert_eq!(status, StatusCode::OK);

    let entry = std::fs::read_dir(&dir)
        .expect("diretorio de auditoria")
        .filter_map(|item| item.ok())
        .map(|item| item.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("api-") && name.ends_with(".jsonl"))
        })
        .expect("arquivo de auditoria");
    let text = std::fs::read_to_string(&entry).expect("conteudo da auditoria");
    let line = text.lines().next().expect("linha de auditoria");
    let parsed: Value = serde_json::from_str(line).expect("jsonl valido");

    assert_eq!(parsed["method"], "GET");
    assert_eq!(parsed["path"], "/health");
    assert_eq!(parsed["status"], 200);
    assert!(parsed["elapsed_ms"].is_number());
    assert!(parsed.get("body").is_none());
    assert!(!line.contains("Bearer"));
    assert!(!line.contains("senha"));

    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Multipart: upload generico e tickets
// ---------------------------------------------------------------------------

fn multipart(uri: &str, token: &str, parts: &[(&str, &str)]) -> Request<Body> {
    let boundary = "zenith-boundary";
    let mut body = String::new();
    for (name, value) in parts {
        body.push_str(&format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n"
        ));
    }
    body.push_str(&format!("--{boundary}--\r\n"));
    Request::builder()
        .uri(uri)
        .method("POST")
        .header("authorization", format!("Bearer {token}"))
        .header(
            "content-type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .expect("request multipart")
}

#[tokio::test]
async fn upload_sem_path_devolve_400() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let req = multipart("/v1/upload", "tok", &[("foo", "bar")]);
    let (status, body) = send(router, req).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["ok"], false);
    assert_eq!(body["error"]["code"], "requisicao_invalida");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn upload_com_path_fora_da_allowlist_devolve_400_sem_rede() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let req = multipart("/v1/upload", "tok", &[("path", "/etc/passwd")]);
    let (status, body) = send(router, req).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "destino_upload_invalido");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn tickets_novos_exigem_token() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    for uri in ["/v1/tickets/criar", "/v1/tickets/responder", "/v1/upload"] {
        let req = post_json(uri, None, r#"{"assunto":"a","texto":"b"}"#);
        let (status, body) = send(router.clone(), req).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{uri}");
        assert_eq!(body["ok"], false, "{uri}");
        assert!(body["error"]["code"].is_string(), "{uri}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Revendas: criacao/edicao/toggle/remocao
// ---------------------------------------------------------------------------

fn request(method: &str, uri: &str, token: Option<&str>, body: &str) -> Request<Body> {
    let mut builder = Request::builder().uri(uri).method(method);
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    if !body.is_empty() {
        builder = builder.header("content-type", "application/json");
    }
    builder
        .body(if body.is_empty() {
            Body::empty()
        } else {
            Body::from(body.to_string())
        })
        .expect("request de teste")
}

#[tokio::test]
async fn remover_revenda_sem_confirm_devolve_409_sem_rede() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let req = request("DELETE", "/v1/revendas/remover?id=7", Some("tok"), "");
    let (status, body) = send(router, req).await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], "confirmacao_necessaria");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn criar_revenda_valida_dias_antes_da_rede() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let req = request(
        "POST",
        "/v1/revendas/criar",
        Some("tok"),
        r#"{"username":"nova","password":"1234","credits":10,"dias_painel":"trinta"}"#,
    );
    let (status, body) = send(router, req).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "requisicao_invalida");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn novas_rotas_de_revenda_exigem_token() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let casos = [
        ("POST", "/v1/revendas/criar"),
        ("PUT", "/v1/revendas/editar"),
        ("PATCH", "/v1/revendas/toggle"),
        ("DELETE", "/v1/revendas/remover?id=7"),
    ];
    for (metodo, uri) in casos {
        let req = request(metodo, uri, None, r#"{"id":1}"#);
        let (status, body) = send(router.clone(), req).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{metodo} {uri}");
        assert_eq!(body["ok"], false, "{metodo} {uri}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Transferencia e convite/adesao
// ---------------------------------------------------------------------------

#[tokio::test]
async fn transferir_destinos_exige_origem_sem_rede() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let req = request("GET", "/v1/revendas/transferir/destinos", Some("tok"), "");
    let (status, body) = send(router, req).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "requisicao_invalida");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn transferir_valida_corpo_sem_rede() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let req = request("POST", "/v1/revendas/transferir", Some("tok"), "{}");
    let (status, body) = send(router, req).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "parametros_invalidos");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn convite_confirmar_exige_pedido_id() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let req = request(
        "POST",
        "/v1/revendas/convite/adesao/confirmar",
        Some("tok"),
        "{}",
    );
    let (status, body) = send(router, req).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "requisicao_invalida");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn rotas_de_convite_exigem_token() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let casos = [
        ("GET", "/v1/revendas/convite"),
        ("POST", "/v1/revendas/convite/regenerar"),
        ("GET", "/v1/revendas/convite/adesao/pendentes"),
        ("GET", "/v1/revendas/convite/mensalista-config"),
    ];
    for (metodo, uri) in casos {
        let req = request(metodo, uri, None, "");
        let (status, body) = send(router.clone(), req).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{metodo} {uri}");
        assert_eq!(body["ok"], false, "{metodo} {uri}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn upload_em_rota_inexistente_devolve_400() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let req = multipart(
        "/v1/upload",
        "tok",
        &[("path", "/merger/api/cliente/nao_existe_upload")],
    );
    let (status, body) = send(router, req).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "destino_upload_invalido");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn tickets_fechar_exige_id_na_query() {
    let dir = temp_dir();
    let router = app(state(Some("tok"), &dir));
    let req = request("POST", "/v1/tickets/fechar", Some("tok"), "");
    let (status, body) = send(router, req).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "requisicao_invalida");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn erro_de_negocio_do_painel_vira_409() {
    // Cobre o contrato do envelope sem depender do painel: usa a funcao pura
    // via resposta HTTP simulada nao e possivel aqui, entao o teste unitario
    // em routes.rs cobre `checar_erro_negocio`; aqui garantimos o status padrao.
    let resposta = AppError::new(
        StatusCode::CONFLICT,
        "erro_negocio_painel",
        "Trial nao pode ser estendido",
    )
    .with_details(serde_json::json!({"ok": false, "erro": "Trial nao pode ser estendido"}))
    .into_response();
    assert_eq!(resposta.status(), StatusCode::CONFLICT);
    let bytes = to_bytes(resposta.into_body(), 4096)
        .await
        .expect("corpo do erro");
    let body: Value = serde_json::from_slice(&bytes).expect("json do erro");
    assert_eq!(body["error"]["code"], "erro_negocio_painel");
    assert_eq!(body["error"]["details"]["panel"]["ok"], false);
}
