//! Testes offline dos contratos de conteudo (pedidos, reportes, catalogo,
//! tickets e avisos).
//!
//! Nenhum teste faz chamada de rede: cobrem apenas as funcoes puras de
//! montagem de query/corpo e as constantes de rota conferidas contra o Go.

use merger_core::api::avisos::{consulta_avisos, normalizar_aviso_lida, AVISOS_LIMITE_MAX};
use merger_core::api::catalogo::{
    consulta_cat_canais, consulta_categorias, consulta_dashboard_financeiro, consulta_pendencia,
    consulta_pendencia_de_valor, consulta_pendencia_opcoes, normalizar_pendencia_corrigir,
    normalizar_pendencia_previa, CATALOGO_TIPOS, PENDENCIA_ACOES, ROTA_CATALOGO_PENDENCIA_CORRIGIR,
    ROTA_CATEGORIAS, ROTA_DASHBOARD_FINANCEIRO,
};
use merger_core::api::pedidos::{
    normalizar_pedido_criacao, normalizar_pedido_status, normalizar_transferencia_creditos,
    PEDIDOS_AUDIOS, PEDIDOS_STATUS, PEDIDOS_TIPOS, ROTA_PEDIDOS_CRIAR, ROTA_PEDIDOS_LISTAR,
    ROTA_PEDIDOS_STATUS,
};
use merger_core::api::reportes::{
    motivos_validos, normalizar_reporte_criacao, normalizar_reporte_status, MOTIVOS_LIVE,
    MOTIVOS_MOVIE, MOTIVOS_SERIES, REPORTES_STATUS, ROTA_REPORTES_CRIAR,
};
use merger_core::api::tickets::{
    consulta_ticket_id, consulta_tickets_admin, normalizar_ticket_novo, normalizar_ticket_resposta,
    normalizar_ticket_status, TICKET_MAX_ASSUNTO, TICKET_STATUS,
};
use merger_core::{ApiError, Result};
use serde_json::{json, Value};

fn espera_config<T: std::fmt::Debug>(resultado: Result<T>) -> String {
    match resultado {
        Ok(valor) => panic!("esperava erro de validacao, veio {valor:?}"),
        Err(ApiError::Config(msg)) => msg,
        Err(outro) => panic!("esperava ApiError::Config, veio {outro:?}"),
    }
}

fn par(chave: &str, valor: &str) -> (String, String) {
    (chave.to_string(), valor.to_string())
}

// ============================================================
// Constantes de rota (contratos conferidos no Go)
// ============================================================

#[test]
fn rotas_batem_com_o_painel_go() {
    assert_eq!(ROTA_PEDIDOS_LISTAR, "/merger/api/pedidos/listar");
    assert_eq!(ROTA_PEDIDOS_CRIAR, "/merger/api/pedidos/criar");
    assert_eq!(ROTA_PEDIDOS_STATUS, "/merger/api/pedidos/status");
    assert_eq!(ROTA_REPORTES_CRIAR, "/merger/api/reportes/criar");
    assert_eq!(ROTA_CATEGORIAS, "/merger/api/categorias");
    assert_eq!(
        ROTA_CATALOGO_PENDENCIA_CORRIGIR,
        "/merger/api/cliente/catalogo/pendencias/corrigir"
    );
    assert_eq!(
        ROTA_DASHBOARD_FINANCEIRO,
        "/merger/api/dashboard_financeiro"
    );
}

#[test]
fn enums_refletem_o_go() {
    assert_eq!(PEDIDOS_TIPOS, ["movie", "series"]);
    assert_eq!(PEDIDOS_AUDIOS, ["dublado", "legendado", "ambos"]);
    assert_eq!(PEDIDOS_STATUS, ["adicionado", "rejeitado", "pendente"]);
    assert_eq!(REPORTES_STATUS, ["corrigido", "pendente"]);
    assert_eq!(CATALOGO_TIPOS, ["live", "movie", "series"]);
    assert_eq!(PENDENCIA_ACOES, ["substituir", "remover"]);
    assert_eq!(TICKET_STATUS, ["aberto", "respondido", "fechado"]);
    assert_eq!(MOTIVOS_LIVE.len(), 5);
    assert_eq!(MOTIVOS_MOVIE.len(), 5);
    assert_eq!(MOTIVOS_SERIES.len(), 6);
}

// ============================================================
// Pedidos de conteudo
// ============================================================

#[test]
fn pedido_filme_minimo_aplica_defaults() {
    let saida = normalizar_pedido_criacao(json!({
        "tmdb_id": 550,
        "titulo": "Clube da Luta"
    }))
    .unwrap();

    assert_eq!(saida["tmdb_id"], 550);
    assert_eq!(saida["titulo"], "Clube da Luta");
    assert_eq!(saida["tipo"], "movie");
    assert_eq!(saida["audio"], "dublado");
    assert_eq!(saida["poster_url"], "");
    assert_eq!(saida["link_m3u"], "");
    assert_eq!(saida["episodios_selecionados"], json!([]));
}

#[test]
fn pedido_serie_preserva_episodios_e_audio() {
    let saida = normalizar_pedido_criacao(json!({
        "tmdb_id": 1399,
        "titulo": "Game of Thrones",
        "tipo": "series",
        "audio": "ambos",
        "poster_url": "https://image.tmdb.org/t/p/w500/poster.jpg",
        "link_m3u": "https://exemplo/list.m3u",
        "episodios_selecionados": ["S01E01", " S01E02 ", ""]
    }))
    .unwrap();

    assert_eq!(saida["tipo"], "series");
    assert_eq!(saida["audio"], "ambos");
    assert_eq!(saida["episodios_selecionados"], json!(["S01E01", "S01E02"]));
}

#[test]
fn pedido_tipo_e_audio_invalidos_caem_no_default_do_go() {
    let saida = normalizar_pedido_criacao(json!({
        "tmdb_id": 1,
        "titulo": "Teste",
        "tipo": "canal",
        "audio": "original"
    }))
    .unwrap();

    assert_eq!(saida["tipo"], "movie");
    assert_eq!(saida["audio"], "dublado");
}

#[test]
fn pedido_sem_titulo_falha_antes_do_request() {
    let msg = espera_config(normalizar_pedido_criacao(json!({ "tmdb_id": 1 })));
    assert!(msg.contains("titulo"), "mensagem inesperada: {msg}");
}

#[test]
fn pedido_com_tmdb_id_zero_falha() {
    let msg = espera_config(normalizar_pedido_criacao(json!({
        "tmdb_id": 0,
        "titulo": "Teste"
    })));
    assert!(msg.contains("tmdb_id"), "mensagem inesperada: {msg}");
}

#[test]
fn pedido_com_episodios_nao_textuais_falha() {
    let msg = espera_config(normalizar_pedido_criacao(json!({
        "tmdb_id": 1,
        "titulo": "Teste",
        "tipo": "series",
        "episodios_selecionados": [1, 2]
    })));
    assert!(
        msg.contains("episodios_selecionados"),
        "mensagem inesperada: {msg}"
    );
}

#[test]
fn pedido_status_valida_enum_e_nota() {
    let saida = normalizar_pedido_status(json!({
        "id": 42,
        "status": "ADICIONADO"
    }))
    .unwrap();
    assert_eq!(saida["id"], 42);
    assert_eq!(saida["status"], "adicionado");
    assert_eq!(saida["nota"], "");

    let msg = espera_config(normalizar_pedido_status(json!({
        "id": 42,
        "status": "entregue"
    })));
    assert!(msg.contains("status"), "mensagem inesperada: {msg}");
}

#[test]
fn transferencia_de_creditos_exige_quantidade_positiva() {
    let saida = normalizar_transferencia_creditos(json!({
        "revenda_id": 7,
        "quantidade": 3
    }))
    .unwrap();
    assert_eq!(saida["revenda_id"], 7);
    assert_eq!(saida["quantidade"], 3);

    let msg = espera_config(normalizar_transferencia_creditos(json!({
        "revenda_id": 7,
        "quantidade": 0
    })));
    assert!(msg.contains("quantidade"), "mensagem inesperada: {msg}");
}

// ============================================================
// Reportes de conteudo
// ============================================================

#[test]
fn reporte_de_canal_usa_whitelist_de_live() {
    let saida = normalizar_reporte_criacao(json!({
        "fonte_id": 3,
        "stream_id_original": "1024",
        "nome_conteudo": "ESPN",
        "motivo": "O canal está travando"
    }))
    .unwrap();

    assert_eq!(saida["tipo"], "live");
    assert_eq!(saida["fonte_id"], 3);
    assert_eq!(saida["motivo"], "O canal está travando");
    assert_eq!(saida["episodios_afetados"], "");
}

#[test]
fn reporte_de_filme_com_motivo_de_canal_falha() {
    let msg = espera_config(normalizar_reporte_criacao(json!({
        "fonte_id": 3,
        "stream_id_original": "900",
        "tipo": "movie",
        "nome_conteudo": "Matrix",
        "motivo": "O canal está travando"
    })));
    assert!(
        msg.contains("motivo invalido"),
        "mensagem inesperada: {msg}"
    );
}

#[test]
fn reporte_tipo_desconhecido_vira_live() {
    let saida = normalizar_reporte_criacao(json!({
        "fonte_id": 0,
        "stream_id_original": "55",
        "tipo": "canal",
        "nome_conteudo": "SBT",
        "motivo": "Link off/quebrado"
    }))
    .unwrap();

    assert_eq!(saida["tipo"], "live");
    assert_eq!(motivos_validos("live"), &MOTIVOS_LIVE);
    assert_eq!(motivos_validos("movie"), &MOTIVOS_MOVIE);
    assert_eq!(motivos_validos("series"), &MOTIVOS_SERIES);
    assert_eq!(motivos_validos("canal"), &MOTIVOS_LIVE);
}

#[test]
fn reporte_exige_stream_e_nome() {
    let msg = espera_config(normalizar_reporte_criacao(json!({
        "nome_conteudo": "ESPN",
        "motivo": "O canal está travando"
    })));
    assert!(
        msg.contains("stream_id_original"),
        "mensagem inesperada: {msg}"
    );
}

#[test]
fn reporte_status_valida_enum() {
    let saida = normalizar_reporte_status(json!({
        "id": 9,
        "status": "corrigido",
        "nota": "link trocado"
    }))
    .unwrap();
    assert_eq!(saida["status"], "corrigido");
    assert_eq!(saida["nota"], "link trocado");

    let msg = espera_config(normalizar_reporte_status(json!({
        "id": 9,
        "status": "fechado"
    })));
    assert!(msg.contains("status"), "mensagem inesperada: {msg}");
}

// ============================================================
// Catalogo, pendencias e dashboard
// ============================================================

#[test]
fn categorias_sem_filtro_nao_envia_query() {
    assert!(consulta_categorias(None, None).unwrap().is_empty());
    assert!(consulta_categorias(None, Some(0)).unwrap().is_empty());
}

#[test]
fn categorias_com_filtro_normaliza_tipo_e_servidor() {
    let query = consulta_categorias(Some("MOVIE"), Some(2)).unwrap();
    assert_eq!(query, vec![par("tipo", "movie"), par("servidor", "2")]);

    let msg = espera_config(consulta_categorias(Some("documentario"), None));
    assert!(
        msg.contains("tipo de catalogo"),
        "mensagem inesperada: {msg}"
    );
}

#[test]
fn cat_canais_exige_vid_e_default_live() {
    let query = consulta_cat_canais(" 123 ", None).unwrap();
    assert_eq!(query, vec![par("vid", "123"), par("tipo", "live")]);

    let query = consulta_cat_canais("123", Some("series")).unwrap();
    assert_eq!(query, vec![par("vid", "123"), par("tipo", "series")]);

    let msg = espera_config(consulta_cat_canais("   ", None));
    assert!(msg.contains("vid"), "mensagem inesperada: {msg}");
}

#[test]
fn pendencias_montam_query_de_id_e_opcoes() {
    assert_eq!(consulta_pendencia(12).unwrap(), vec![par("id", "12")]);
    assert!(espera_config(consulta_pendencia(0)).contains("id"));

    let query = consulta_pendencia_opcoes(12, Some("  rede  ")).unwrap();
    assert_eq!(query, vec![par("id", "12"), par("q", "rede")]);

    let longa = "x".repeat(250);
    let query = consulta_pendencia_opcoes(12, Some(&longa)).unwrap();
    assert_eq!(query[1].1.chars().count(), 200);
}

#[test]
fn revisar_aceita_numero_ou_objeto() {
    assert_eq!(
        consulta_pendencia_de_valor(&json!(99)).unwrap(),
        vec![par("id", "99")]
    );
    assert_eq!(
        consulta_pendencia_de_valor(&json!({"id": 99})).unwrap(),
        vec![par("id", "99")]
    );
    assert!(espera_config(consulta_pendencia_de_valor(&json!("99"))).contains("id"));
}

#[test]
fn previa_de_pendencia_valida_acao_e_destino() {
    let saida = normalizar_pendencia_previa(json!({
        "id": 5,
        "revisao": "abc123",
        "acao": "substituir",
        "destino_id": "10",
        "fonte_id": 3
    }))
    .unwrap();
    assert_eq!(saida["acao"], "substituir");
    assert_eq!(saida["destino_id"], "10");
    assert_eq!(saida["fonte_id"], 3);

    let remover = normalizar_pendencia_previa(json!({
        "id": 5,
        "revisao": "abc123",
        "acao": "remover",
        "destino_id": ""
    }))
    .unwrap();
    assert_eq!(remover["acao"], "remover");

    let msg = espera_config(normalizar_pendencia_previa(json!({
        "id": 5,
        "revisao": "abc123",
        "acao": "substituir",
        "destino_id": ""
    })));
    assert!(msg.contains("destino_id"), "mensagem inesperada: {msg}");
}

#[test]
fn corrigir_pendencia_exige_token() {
    let saida = normalizar_pendencia_corrigir(json!({ "token": "deadbeef" })).unwrap();
    assert_eq!(saida["token"], "deadbeef");

    let msg = espera_config(normalizar_pendencia_corrigir(json!({ "token": " " })));
    assert!(msg.contains("token"), "mensagem inesperada: {msg}");
}

#[test]
fn dashboard_offset_e_clampado_em_24() {
    assert!(consulta_dashboard_financeiro(None).is_empty());
    assert!(consulta_dashboard_financeiro(Some(0)).is_empty());
    assert_eq!(
        consulta_dashboard_financeiro(Some(3)),
        vec![par("offset", "3")]
    );
    assert_eq!(
        consulta_dashboard_financeiro(Some(99)),
        vec![par("offset", "24")]
    );
}

// ============================================================
// Tickets de suporte
// ============================================================

#[test]
fn tickets_admin_ignora_status_invalido_como_o_go() {
    assert!(consulta_tickets_admin(None).is_empty());
    assert_eq!(
        consulta_tickets_admin(Some("ABERTO")),
        vec![par("status", "aberto")]
    );
    assert!(consulta_tickets_admin(Some("urgente")).is_empty());
}

#[test]
fn ticket_id_exige_positivo() {
    assert_eq!(consulta_ticket_id(8).unwrap(), vec![par("id", "8")]);
    assert!(espera_config(consulta_ticket_id(0)).contains("id"));
}

#[test]
fn ticket_novo_corta_assunto_e_texto() {
    let assunto = "a".repeat(TICKET_MAX_ASSUNTO + 20);
    let saida = normalizar_ticket_novo(json!({
        "assunto": assunto,
        "texto": "  preciso de ajuda  "
    }))
    .unwrap();
    assert_eq!(saida["assunto"].as_str().unwrap().chars().count(), 120);
    assert_eq!(saida["texto"], "preciso de ajuda");

    let msg = espera_config(normalizar_ticket_novo(json!({
        "assunto": "sem texto"
    })));
    assert!(msg.contains("texto"), "mensagem inesperada: {msg}");
}

#[test]
fn ticket_resposta_e_status_validam_campos() {
    let resposta = normalizar_ticket_resposta(json!({
        "id": 4,
        "texto": "voltei a testar"
    }))
    .unwrap();
    assert_eq!(resposta["id"], 4);

    let status = normalizar_ticket_status(json!({
        "id": 4,
        "status": "FECHADO"
    }))
    .unwrap();
    assert_eq!(status["status"], "fechado");

    let msg = espera_config(normalizar_ticket_status(json!({
        "id": 4,
        "status": "pausado"
    })));
    assert!(msg.contains("status"), "mensagem inesperada: {msg}");
}

// ============================================================
// Avisos
// ============================================================

#[test]
fn avisos_limit_so_vai_quando_esta_na_faixa() {
    assert!(consulta_avisos(None).is_empty());
    assert!(consulta_avisos(Some(0)).is_empty());
    assert!(consulta_avisos(Some(AVISOS_LIMITE_MAX + 1)).is_empty());
    assert_eq!(consulta_avisos(Some(50)), vec![par("limit", "50")]);
    assert_eq!(
        consulta_avisos(Some(AVISOS_LIMITE_MAX)),
        vec![par("limit", "100")]
    );
}

#[test]
fn aviso_lida_aceita_id_ou_todos() {
    let um = normalizar_aviso_lida(json!({ "id": 12 })).unwrap();
    assert_eq!(um, json!({ "id": 12 }));

    let todos = normalizar_aviso_lida(json!({ "todos": true })).unwrap();
    assert_eq!(todos, json!({ "todos": true }));

    // Precedencia do Go: `todos` vence quando os dois vem juntos.
    let ambos = normalizar_aviso_lida(json!({ "id": 12, "todos": true })).unwrap();
    assert_eq!(ambos, json!({ "todos": true }));
}

#[test]
fn aviso_lida_sem_id_ou_todos_falha() {
    let msg = espera_config(normalizar_aviso_lida(json!({})));
    assert!(msg.contains("id"), "mensagem inesperada: {msg}");

    let msg = espera_config(normalizar_aviso_lida(json!({ "id": 0 })));
    assert!(msg.contains("id"), "mensagem inesperada: {msg}");
}

#[test]
fn corpo_de_aviso_precisa_ser_objeto() {
    let msg = espera_config(normalizar_aviso_lida(Value::String("12".into())));
    assert!(msg.contains("objeto JSON"), "mensagem inesperada: {msg}");
}
