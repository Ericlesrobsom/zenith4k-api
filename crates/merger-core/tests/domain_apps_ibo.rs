//! Testes offline do modulo Apps Ibo (msproapp).

use merger_core::api::apps_ibo::{build_apps_ibo_ativar_body, normalizar_mac, normalizar_preco};
use merger_core::{ApiError, ClientConfig, MergerClient};
use serde_json::json;

fn cliente() -> MergerClient {
    let cfg = ClientConfig::new("https://painel.exemplo.test", "franquia", "segredo")
        .expect("config valida");
    MergerClient::new(cfg).expect("cliente sem rede")
}

fn base_m3u8() -> serde_json::Value {
    json!({
        "conta": 36,
        "app_id": "10",
        "mac": "00:1a:79:00:00:00",
        "modo": "1",
        "server_name": "Servidor QA",
        "price": "0",
        "expire_date": "2026-12-31",
        "m3u8_list": "http://exemplo.test/get.php?username=x&password=y",
    })
}

// ---------------------------------------------------------------------------
// Normalizadores
// ---------------------------------------------------------------------------

#[test]
fn mac_aceita_formatos_comuns() {
    assert_eq!(
        normalizar_mac("00:1a:79:00:00:00").expect("mac"),
        "00:1A:79:00:00:00"
    );
    assert_eq!(
        normalizar_mac("001A79000000").expect("mac"),
        "00:1A:79:00:00:00"
    );
    assert_eq!(
        normalizar_mac("00-1A-79-00-00-00").expect("mac"),
        "00:1A:79:00:00:00"
    );
    assert_eq!(
        normalizar_mac("00 1A 79 00 00 00").expect("mac"),
        "00:1A:79:00:00:00"
    );
}

#[test]
fn mac_rejeita_tamanho_errado() {
    let erro = normalizar_mac("00:1A:79:00:00").expect_err("curto");
    assert!(matches!(erro, ApiError::InvalidMac(_)));
}

#[test]
fn mac_aceita_letras_fora_de_hex_como_o_painel() {
    // O campo do msproapp.top nao exige hexadecimal; exigir aqui recusaria
    // aparelho que o painel aceita.
    assert_eq!(
        normalizar_mac("zz:11:22:33:44:55").expect("mac"),
        "ZZ:11:22:33:44:55"
    );
}

#[test]
fn preco_normaliza_virgula_e_milhar() {
    assert_eq!(normalizar_preco("30").expect("preco"), "30");
    assert_eq!(normalizar_preco("30,5").expect("preco"), "30.5");
    assert_eq!(normalizar_preco("30,50").expect("preco"), "30.50");
    assert_eq!(normalizar_preco("1.234,56").expect("preco"), "1234.56");
    assert_eq!(normalizar_preco("1.000").expect("preco"), "1000");
    assert_eq!(normalizar_preco("0").expect("preco"), "0");
}

#[test]
fn preco_rejeita_formatos_estranhos() {
    for bruto in ["", "1e5", "abc", "30,555", "1.2.3"] {
        let erro = normalizar_preco(bruto).expect_err("preco invalido");
        assert!(matches!(erro, ApiError::Config(_)), "aceitou {bruto:?}");
    }
}

// ---------------------------------------------------------------------------
// Payload de ativacao
// ---------------------------------------------------------------------------

#[test]
fn ativar_m3u8_monta_payload_completo() {
    let payload = build_apps_ibo_ativar_body(&base_m3u8()).expect("payload");
    assert_eq!(payload["modo_selecao"], "1");
    assert_eq!(payload["mac_device"], "00:1A:79:00:00:00");
    assert_eq!(payload["server_name"], "Servidor QA");
    assert_eq!(payload["ranking_app_id"], "10");
    assert_eq!(payload["price"], "0");
    assert_eq!(payload["expire_date"], "2026-12-31");
    assert_eq!(
        payload["m3u8_list"],
        "http://exemplo.test/get.php?username=x&password=y"
    );
    assert_eq!(payload["xteam_username"], "");
    assert_eq!(payload["xteam_password"], "");
    assert_eq!(payload["dns"], "");
    assert_eq!(payload["username_login"], serde_json::Value::Null);
    assert_eq!(payload["password_login"], serde_json::Value::Null);
    assert_eq!(payload["is_trial"], false);
    assert_eq!(payload["general_list_title"], "");
    assert_eq!(payload["playlists"], serde_json::Value::Null);
    assert!(payload.get("conta").is_none());
}

#[test]
fn ativar_m3u8_exige_lista() {
    let mut body = base_m3u8();
    body["m3u8_list"] = json!("");
    let erro = build_apps_ibo_ativar_body(&body).expect_err("sem lista");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn ativar_xteam_exige_credenciais() {
    let mut body = base_m3u8();
    body["modo"] = json!("0");
    let erro = build_apps_ibo_ativar_body(&body).expect_err("sem xteam");
    assert!(matches!(erro, ApiError::Config(_)));

    body["xteam_username"] = json!("user");
    body["xteam_password"] = json!("pass");
    body["dns"] = json!("dns.exemplo.test");
    let payload = build_apps_ibo_ativar_body(&body).expect("payload xteam");
    assert_eq!(payload["modo_selecao"], "0");
    assert_eq!(payload["xteam_username"], "user");
    assert_eq!(payload["xteam_password"], "pass");
    assert_eq!(payload["m3u8_list"], "");
}

#[test]
fn ativar_aceita_modo_por_nome() {
    let mut body = base_m3u8();
    body["modo"] = json!("m3u8");
    let payload = build_apps_ibo_ativar_body(&body).expect("modo nome");
    assert_eq!(payload["modo_selecao"], "1");

    body["modo"] = json!("xteam");
    body["xteam_username"] = json!("user");
    body["xteam_password"] = json!("pass");
    let payload = build_apps_ibo_ativar_body(&body).expect("modo nome");
    assert_eq!(payload["modo_selecao"], "0");
}

#[test]
fn ativar_data_invalida_falha() {
    let mut body = base_m3u8();
    body["expire_date"] = json!("31/12/2026");
    let erro = build_apps_ibo_ativar_body(&body).expect_err("data");
    assert!(matches!(erro, ApiError::Config(_)));

    body["expire_date"] = json!("2026-02-30");
    let erro = build_apps_ibo_ativar_body(&body).expect_err("data");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn ativar_zone_x_preenche_login() {
    let mut body = base_m3u8();
    body["app_id"] = json!("11");
    body["username_login"] = json!("login");
    body["password_login"] = json!("senha");
    let payload = build_apps_ibo_ativar_body(&body).expect("payload");
    assert_eq!(payload["username_login"], "login");
    assert_eq!(payload["password_login"], "senha");
}

#[test]
fn ativar_aceita_aliases_e_trial() {
    let body = json!({
        "conta": 36,
        "ranking_app_id": "10",
        "mac_device": "001A79000000",
        "modo_selecao": "1",
        "server_name": "Servidor QA",
        "price": "30,50",
        "expire_date": "2026-12-31",
        "m3u8_list": "http://exemplo.test/x",
        "is_trial": true,
    });
    let payload = build_apps_ibo_ativar_body(&body).expect("payload");
    assert_eq!(payload["price"], "30.50");
    assert_eq!(payload["is_trial"], true);
}

#[tokio::test]
async fn ativar_exige_conta_sem_rede() {
    let mut body = base_m3u8();
    body["conta"] = json!(0);
    let erro = cliente()
        .apps_ibo_ativar(body)
        .await
        .expect_err("conta invalida");
    assert!(matches!(erro, ApiError::Config(_)));
}

// ---------------------------------------------------------------------------
// Travas locais (sem rede)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn excluir_exige_confirmacao_sem_rede() {
    let erro = cliente()
        .apps_ibo_excluir("4801338", 36, false)
        .await
        .expect_err("sem confirm");
    assert!(matches!(erro, ApiError::ConfirmationRequired(_)));
}

#[tokio::test]
async fn atribuir_exige_dono_sem_rede() {
    let erro = cliente()
        .apps_ibo_atribuir(36, "4801338", "  ")
        .await
        .expect_err("sem dono");
    assert!(matches!(erro, ApiError::Config(_)));
}
