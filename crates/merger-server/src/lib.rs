//! API REST local do painel goiptv merger (franquia ZENITH4K).
//!
//! O binario `zenith4k-serve` monta o roteador em [`app`]; a biblioteca fica
//! separada para permitir testes de integracao com `tower::ServiceExt`.

pub mod audit;
pub mod auth;
pub mod envelope;
pub mod error;
pub mod extract;
pub mod routes;
pub mod state;

use axum::extract::DefaultBodyLimit;
use axum::middleware;
use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub use state::AppState;

/// Monta o roteador completo.
///
/// Rotas publicas: `/health` e `/openapi.json`. Todas as rotas `/v1/*` passam
/// pelo middleware de token (ou de loopback, em modo dev).
pub fn app(state: AppState) -> Router {
    let protected = Router::new()
        .route("/v1/meta", get(routes::meta))
        .route("/v1/login", post(routes::login))
        .route("/v1/logout", post(routes::logout))
        .route("/v1/whoami", get(routes::whoami))
        .route("/v1/raw", post(routes::raw))
        .route("/v1/revendas", get(routes::revendas))
        .route("/v1/revendas/criar", post(routes::revendas_criar))
        .route("/v1/revendas/editar", put(routes::revendas_editar))
        .route("/v1/revendas/toggle", patch(routes::revendas_toggle))
        .route("/v1/revendas/remover", delete(routes::revendas_remover))
        .route(
            "/v1/revendas/transferir/destinos",
            get(routes::revendas_transferir_destinos),
        )
        .route("/v1/revendas/transferir", post(routes::revendas_transferir))
        .route("/v1/revendas/convite", get(routes::revendas_convite))
        .route(
            "/v1/revendas/convite/regenerar",
            post(routes::revendas_convite_regenerar),
        )
        .route(
            "/v1/revendas/convite/adesao/pendentes",
            get(routes::revendas_convite_adesao_pendentes),
        )
        .route(
            "/v1/revendas/convite/adesao/confirmar",
            post(routes::revendas_convite_adesao_confirmar),
        )
        .route(
            "/v1/revendas/convite/mensalista-config",
            get(routes::revendas_convite_config).post(routes::revendas_convite_config_salvar),
        )
        .route("/v1/revendas/arvore", get(routes::revendas_arvore))
        .route("/v1/revendas/renovar", post(routes::revendas_renovar))
        .route("/v1/revendas/nota", post(routes::revendas_nota))
        .route("/v1/revendas/unlimited", post(routes::revendas_unlimited))
        .route("/v1/usuarios", get(routes::usuarios))
        .route("/v1/usuarios/criar", post(routes::usuarios_criar))
        .route("/v1/usuarios/editar", post(routes::usuarios_editar))
        .route("/v1/usuarios/toggle", post(routes::usuarios_toggle))
        .route("/v1/usuarios/renovar", post(routes::usuarios_renovar))
        .route("/v1/usuarios/estender", post(routes::usuarios_estender))
        .route("/v1/usuarios/delete", post(routes::usuarios_delete))
        .route("/v1/planos", get(routes::planos))
        .route("/v1/creditos", get(routes::creditos))
        .route("/v1/pedidos", get(routes::pedidos))
        .route("/v1/pedidos/criar", post(routes::pedidos_criar))
        .route("/v1/reportes", get(routes::reportes))
        .route("/v1/reportes/criar", post(routes::reportes_criar))
        .route("/v1/catalogo/pendencias", get(routes::catalogo_pendencias))
        .route("/v1/catalogo/jogos", get(routes::catalogo_jogos))
        .route("/v1/tickets", get(routes::tickets))
        .route("/v1/tickets/criar", post(routes::tickets_criar))
        .route("/v1/tickets/responder", post(routes::tickets_responder))
        .route("/v1/tickets/fechar", post(routes::tickets_fechar))
        .route("/v1/upload", post(routes::upload))
        .route("/v1/avisos", get(routes::avisos))
        .route("/v1/apps-ativa/saldo", get(routes::apps_ativa_saldo))
        .route("/v1/apps-ativa/apps", get(routes::apps_ativa_apps))
        .route(
            "/v1/apps-ativa/historico",
            get(routes::apps_ativa_historico),
        )
        .route(
            "/v1/apps-ativa/ativacoes",
            get(routes::apps_ativa_ativacoes),
        )
        .route("/v1/apps-ativa/ativar", post(routes::apps_ativa_ativar))
        .route("/v1/apps-ibo/contas", get(routes::apps_ibo_contas))
        .route("/v1/apps-ibo/apps", get(routes::apps_ibo_apps))
        .route("/v1/apps-ibo/clientes", get(routes::apps_ibo_clientes))
        .route("/v1/apps-ibo/ativar", post(routes::apps_ibo_ativar))
        .route(
            "/v1/apps-ibo/clientes/{id}",
            delete(routes::apps_ibo_excluir),
        )
        .route(
            "/v1/apps-ibo/clientes/{id}/atribuir",
            post(routes::apps_ibo_atribuir),
        )
        .route("/v1/stats", get(routes::stats))
        .route("/v1/dashboard", get(routes::dashboard))
        .layer(DefaultBodyLimit::max(25 * 1024 * 1024))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::require_token,
        ));

    Router::new()
        .route("/health", get(routes::health))
        .route("/openapi.json", get(routes::openapi_json))
        .route("/openapi.yaml", get(routes::openapi_yaml))
        .merge(protected)
        .fallback(routes::not_found)
        .method_not_allowed_fallback(routes::method_not_allowed)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            audit::middleware,
        ))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
