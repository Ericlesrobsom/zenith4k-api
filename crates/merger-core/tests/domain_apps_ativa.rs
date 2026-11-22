//! Testes offline do Apps Ativa (Ative App) — foco no MAC e na key_app.

use merger_core::api::apps_ativa::build_apps_ativa_ativar_body;
use merger_core::mac::{normalizar_mac_ibo, normalizar_mac_provedor};
use merger_core::{ApiError, ClientConfig, MergerClient};
use serde_json::json;

fn cliente() -> MergerClient {
    let cfg = ClientConfig::new("https://painel.exemplo.test", "franquia", "segredo")
        .expect("config valida");
    MergerClient::new(cfg).expect("cliente sem rede")
}

// ---------------------------------------------------------------------------
// MAC do provedor (apps_ativa)
// ---------------------------------------------------------------------------

#[test]
fn mac_provedor_aceita_formatos_do_pedido() {
    let esperado = "ab:51:6e:66:79:60";
    for entrada in [
        "ab516e667960",
        "ab:51:6e:66:79:60",
        "AB-51-6E-66-79-60",
        "ab.51.6e.66.79.60",
        "ab 51 6e 66 79 60",
        "  AB:51:6E:66:79:60  ",
    ] {
        assert_eq!(
            normalizar_mac_provedor(entrada).expect(entrada),
            esperado,
            "entrada {entrada:?}"
        );
    }
}

#[test]
fn mac_provedor_rejeita_curto_e_nao_hex() {
    for entrada in [
        "abc",
        "gg:51:6e:66:79:60",
        "ab:51:6e:66:79",
        "ab:51:6e:66:79:60:01",
    ] {
        let erro = normalizar_mac_provedor(entrada).expect_err(entrada);
        assert!(
            matches!(erro, ApiError::InvalidMac(_)),
            "entrada {entrada:?} virou {erro:?}"
        );
    }
}

#[test]
fn mac_ibo_continua_aceitando_letras_fora_de_hex() {
    assert_eq!(
        normalizar_mac_ibo("zz:11:22:33:44:55").expect("mac ibo"),
        "ZZ:11:22:33:44:55"
    );
    // O formato do provedor e outro: IBO e maiusculo e aceita nao-hex.
    assert_ne!(
        normalizar_mac_ibo("ab:51:6e:66:79:60").expect("mac ibo"),
        normalizar_mac_provedor("ab:51:6e:66:79:60").expect("mac provedor")
    );
}

// ---------------------------------------------------------------------------
// Corpo de ativacao
// ---------------------------------------------------------------------------

#[test]
fn ativar_normaliza_mac_e_limpa_campos() {
    let corpo = build_apps_ativa_ativar_body(&json!({
        "app_uuid": "1c2e4a70-6869-4d79-9e2c-85f972743707",
        "mac_app": "AB-51-6E-66-79-60",
        "key_app": "  123456  ",
        "descricao": "  cliente Joao  ",
    }))
    .expect("corpo valido");

    assert_eq!(
        corpo,
        json!({
            "app_uuid": "1c2e4a70-6869-4d79-9e2c-85f972743707",
            "mac_app": "ab:51:6e:66:79:60",
            "key_app": "123456",
            "descricao": "cliente Joao",
        })
    );
}

#[test]
fn ativar_sem_key_e_sem_descricao_manda_strings_vazias() {
    let corpo = build_apps_ativa_ativar_body(&json!({
        "app_uuid": "x",
        "mac_app": "ab516e667960",
    }))
    .expect("corpo valido");
    assert_eq!(corpo["key_app"], "");
    assert_eq!(corpo["descricao"], "");
}

#[test]
fn ativar_exige_app_uuid() {
    let erro = build_apps_ativa_ativar_body(&json!({"mac_app": "ab516e667960"}))
        .expect_err("sem app_uuid");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn ativar_rejeita_key_com_espaco_ou_controle() {
    for key in ["12 34", "abc\tdef", "chave\u{7}"] {
        let erro = build_apps_ativa_ativar_body(&json!({
            "app_uuid": "x",
            "mac_app": "ab516e667960",
            "key_app": key,
        }))
        .expect_err(key);
        assert!(
            matches!(erro, ApiError::InvalidKeyApp(_)),
            "key {key:?} virou {erro:?}"
        );
    }
}

#[test]
fn ativar_rejeita_key_gigante() {
    let erro = build_apps_ativa_ativar_body(&json!({
        "app_uuid": "x",
        "mac_app": "ab516e667960",
        "key_app": "a".repeat(129),
    }))
    .expect_err("key longa");
    assert!(matches!(erro, ApiError::InvalidKeyApp(_)));
}

#[test]
fn ativar_corta_descricao_em_200() {
    let corpo = build_apps_ativa_ativar_body(&json!({
        "app_uuid": "x",
        "mac_app": "ab516e667960",
        "descricao": "d".repeat(250),
    }))
    .expect("corpo valido");
    assert_eq!(corpo["descricao"].as_str().unwrap().chars().count(), 200);
}

#[tokio::test]
async fn ativar_mac_invalido_falha_antes_de_qualquer_rede() {
    let erro = cliente()
        .apps_ativa_ativar(json!({
            "app_uuid": "x",
            "mac_app": "gg:51:6e:66:79:60",
        }))
        .await
        .expect_err("mac invalido");
    assert!(matches!(erro, ApiError::InvalidMac(_)));
}
