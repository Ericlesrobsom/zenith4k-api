//! Testes offline dos contratos de revendas, usuarios e planos.
//!
//! Nenhum teste abre rede: exercitamos os helpers puros de montagem de
//! query/corpo e as travas locais que rodam antes do `send`.

use merger_core::api::planos::build_criar_linha_body;
use merger_core::api::revendas::{
    build_revenda_editar_body, build_revenda_nota_body, build_revenda_nova_body,
    build_revenda_renovar_body, build_revenda_unlimited_body, build_transfer_revenda_body,
    vencimento_em_dias,
};
use merger_core::api::usuarios::{
    build_usuario_adultos_query, build_usuario_contato_body, build_usuario_criar_query,
    build_usuario_delete_query, build_usuario_editar_body, build_usuario_estender_body,
    build_usuario_renovar_body, build_usuario_telas_body, build_usuario_toggle_query,
    build_usuario_valor_body,
};
use merger_core::{ApiError, ClientConfig, MergerClient};
use serde_json::json;

fn cliente() -> MergerClient {
    let cfg = ClientConfig::new("https://painel.exemplo.test", "franquia", "segredo")
        .expect("config valida");
    MergerClient::new(cfg).expect("cliente sem rede")
}

fn pares(valores: &[(&str, &str)]) -> Vec<(String, String)> {
    valores
        .iter()
        .map(|(chave, valor)| (chave.to_string(), valor.to_string()))
        .collect()
}

// ---------------------------------------------------------------------------
// Revendas
// ---------------------------------------------------------------------------

#[test]
fn renovar_monta_corpo_exato_sem_campos_extras() {
    let body = json!({
        "id": 42,
        "vencimento_atual": "2026-11-30",
        "username": "ignorado",
    });
    let montado = build_revenda_renovar_body(&body).expect("corpo valido");
    assert_eq!(montado, json!({"id": 42, "vencimento_atual": "2026-11-30"}));
}

#[test]
fn renovar_aceita_vencimento_vazio_para_pular_conferencia() {
    let montado = build_revenda_renovar_body(&json!({"id": 7, "vencimento_atual": ""}))
        .expect("corpo valido");
    assert_eq!(montado, json!({"id": 7, "vencimento_atual": ""}));
}

#[test]
fn renovar_exige_vencimento_atual() {
    let erro = build_revenda_renovar_body(&json!({"id": 7})).expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn nota_aceita_username_e_preserva_campos_vazios() {
    let montado = build_revenda_nota_body(&json!({
        "username": " sub01 ",
        "contato_nome": "Fulano",
        "contato_email": "",
        "contato_observacoes": "cobrar na segunda",
        "campo_desconhecido": "nunca vai ao painel",
    }))
    .expect("corpo valido");

    assert_eq!(montado.get("username"), Some(&json!("sub01")));
    assert_eq!(montado.get("contato_nome"), Some(&json!("Fulano")));
    // String vazia limpa o campo no painel: precisa ser preservada.
    assert_eq!(montado.get("contato_email"), Some(&json!("")));
    assert_eq!(
        montado.get("contato_observacoes"),
        Some(&json!("cobrar na segunda"))
    );
    assert!(montado.get("campo_desconhecido").is_none());
}

#[test]
fn nota_exige_id_ou_username() {
    let erro =
        build_revenda_nota_body(&json!({"contato_nome": "Fulano"})).expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn unlimited_normaliza_saldo_ausente_para_zero() {
    let montado =
        build_revenda_unlimited_body(&json!({"id": 3, "novo_estado": 1})).expect("corpo valido");
    assert_eq!(
        montado,
        json!({"id": 3, "novo_estado": 1, "saldo_rebaixamento": 0})
    );
}

#[test]
fn unlimited_rejeita_estado_fora_de_zero_um() {
    let erro =
        build_revenda_unlimited_body(&json!({"id": 3, "novo_estado": 2})).expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn transfer_revenda_monta_origem_novo_pai_e_confirmacao() {
    let montado = build_transfer_revenda_body(&json!({
        "origem": "sub01",
        "novo_pai": "sub02",
        "confirmar_cross": true,
    }))
    .expect("corpo valido");
    assert_eq!(
        montado,
        json!({"origem": "sub01", "novo_pai": "sub02", "confirmar_cross": true})
    );

    let padrao = build_transfer_revenda_body(&json!({"origem": "a", "novo_pai": "b"}))
        .expect("corpo valido");
    assert_eq!(padrao.get("confirmar_cross"), Some(&json!(false)));
}

#[test]
fn transfer_revenda_exige_novo_pai() {
    let erro = build_transfer_revenda_body(&json!({"origem": "sub01"})).expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[tokio::test]
async fn destinos_exige_origem_antes_de_qualquer_rede() {
    let erro = cliente()
        .transfer_revenda_destinos("   ")
        .await
        .expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[tokio::test]
async fn nota_ler_exige_id_positivo() {
    let erro = cliente()
        .revendas_nota_ler(0)
        .await
        .expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

// ---------------------------------------------------------------------------
// Usuarios / clientes finais
// ---------------------------------------------------------------------------

#[test]
fn criar_usuario_traduz_corpo_para_query_do_painel() {
    let query = build_usuario_criar_query(&json!({
        "username": "cliente1",
        "password": "senha123",
        "max_connections": 2,
        "dias": 30,
        "is_trial": 0,
        "bouquet": "COMPLETO C/ ADULTOS",
        "servidor_id": 4,
    }))
    .expect("corpo valido");

    assert_eq!(
        query,
        pares(&[
            ("username", "cliente1"),
            ("password", "senha123"),
            ("max_connections", "2"),
            ("dias", "30"),
            ("is_trial", "0"),
            ("servidor_id", "4"),
            ("bouquet", "COMPLETO C/ ADULTOS"),
        ])
    );
}

#[test]
fn criar_usuario_aceita_numero_como_texto() {
    let query = build_usuario_criar_query(&json!({
        "username": "cliente1",
        "password": "senha123",
        "max_connections": "3",
    }))
    .expect("corpo valido");
    assert!(query.contains(&("max_connections".to_string(), "3".to_string())));
}

#[test]
fn criar_usuario_exige_senha() {
    let erro =
        build_usuario_criar_query(&json!({"username": "cliente1"})).expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn toggle_usa_id_e_acao_na_query() {
    let query =
        build_usuario_toggle_query(&json!({"id": 99, "acao": "bloquear"})).expect("corpo valido");
    assert_eq!(query, pares(&[("id", "99"), ("acao", "bloquear")]));
}

#[test]
fn toggle_rejeita_acao_desconhecida_para_nao_desbloquear_por_engano() {
    let erro = build_usuario_toggle_query(&json!({"id": 99, "acao": "bloqueiar"}))
        .expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn delete_usa_query_id() {
    let query = build_usuario_delete_query(&json!({"id": 123})).expect("corpo valido");
    assert_eq!(query, pares(&[("id", "123")]));
}

#[tokio::test]
async fn delete_sem_confirm_devolve_confirmation_required_sem_rede() {
    let erro = cliente()
        .usuarios_delete(json!({"id": 123}), false)
        .await
        .expect_err("deve falhar");
    match erro {
        ApiError::ConfirmationRequired(rota) => {
            assert!(rota.contains("sigma_users/delete"));
            assert!(rota.contains("123"));
        }
        outro => panic!("erro inesperado: {outro:?}"),
    }
}

#[test]
fn editar_monta_corpo_com_campos_opcionais() {
    let montado = build_usuario_editar_body(&json!({
        "user_id": 5,
        "novo_username": "cliente2",
        "nova_senha": "nova123",
        "validade": "2027-01-31",
        "extra": "descartado",
    }))
    .expect("corpo valido");

    assert_eq!(
        montado,
        json!({
            "user_id": 5,
            "novo_username": "cliente2",
            "nova_senha": "nova123",
            "validade": "2027-01-31",
        })
    );
}

#[test]
fn renovar_usuario_exige_user_id_e_plano_id() {
    let montado =
        build_usuario_renovar_body(&json!({"user_id": 5, "plano_id": 2})).expect("corpo valido");
    assert_eq!(montado, json!({"user_id": 5, "plano_id": 2}));

    let erro = build_usuario_renovar_body(&json!({"user_id": 5})).expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn estender_limita_dias_ao_teto_do_painel() {
    let montado =
        build_usuario_estender_body(&json!({"user_id": 5, "dias": 30})).expect("corpo valido");
    assert_eq!(montado, json!({"user_id": 5, "dias": 30}));

    let erro =
        build_usuario_estender_body(&json!({"user_id": 5, "dias": 3651})).expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn telas_e_valor_montam_corpo_tipado() {
    let telas = build_usuario_telas_body(&json!({"user_id": 5, "telas": 3})).expect("corpo valido");
    assert_eq!(telas, json!({"user_id": 5, "telas": 3}));

    let valor =
        build_usuario_valor_body(&json!({"user_id": 5, "valor": 29.9})).expect("corpo valido");
    assert_eq!(valor.get("user_id"), Some(&json!(5)));
    let numero = valor
        .get("valor")
        .and_then(serde_json::Value::as_f64)
        .expect("valor numerico");
    assert!((numero - 29.9).abs() < 1e-9);
}

#[test]
fn valor_aceita_texto_com_virgula() {
    let valor =
        build_usuario_valor_body(&json!({"user_id": 5, "valor": "29,90"})).expect("corpo valido");
    let numero = valor
        .get("valor")
        .and_then(serde_json::Value::as_f64)
        .expect("valor numerico");
    assert!((numero - 29.9).abs() < 1e-9);
}

#[test]
fn adultos_usa_query_e_omite_plano_zero() {
    let sem_plano =
        build_usuario_adultos_query(&json!({"id": 8, "acao": "ligar"})).expect("corpo valido");
    assert_eq!(sem_plano, pares(&[("id", "8"), ("acao", "ligar")]));

    let com_plano = build_usuario_adultos_query(&json!({
        "id": 8,
        "acao": "desligar",
        "plano_destino_id": 12,
    }))
    .expect("corpo valido");
    assert_eq!(
        com_plano,
        pares(&[
            ("id", "8"),
            ("acao", "desligar"),
            ("plano_destino_id", "12")
        ])
    );
}

#[test]
fn adultos_rejeita_acao_invalida() {
    let erro =
        build_usuario_adultos_query(&json!({"id": 8, "acao": "on"})).expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn contato_preserva_string_vazia_para_limpar_campo() {
    let montado = build_usuario_contato_body(&json!({
        "user_id": 5,
        "contato_whatsapp": "",
        "contato_nome": "Fulano",
    }))
    .expect("corpo valido");
    assert_eq!(montado.get("contato_whatsapp"), Some(&json!("")));
    assert_eq!(montado.get("contato_nome"), Some(&json!("Fulano")));
    assert_eq!(montado.get("user_id"), Some(&json!(5)));
}

// ---------------------------------------------------------------------------
// Planos / creditos
// ---------------------------------------------------------------------------

#[test]
fn criar_linha_monta_corpo_com_contato_e_servidor() {
    let montado = build_criar_linha_body(&json!({
        "username": "cliente1",
        "password": "senha123",
        "plano_id": 3,
        "valor_personalizado": 29.9,
        "contato_nome": "Fulano",
        "contato_whatsapp": "5511999999999",
        "servidor_id": 2,
        "renovacao_qr": true,
        "campo_extra": "descartado",
    }))
    .expect("corpo valido");

    assert_eq!(montado.get("username"), Some(&json!("cliente1")));
    assert_eq!(montado.get("plano_id"), Some(&json!(3)));
    assert_eq!(
        montado.get("contato_whatsapp"),
        Some(&json!("5511999999999"))
    );
    assert_eq!(montado.get("servidor_id"), Some(&json!(2)));
    assert_eq!(montado.get("renovacao_qr"), Some(&json!(true)));
    assert!(montado.get("campo_extra").is_none());
}

#[test]
fn criar_linha_exige_plano() {
    let erro = build_criar_linha_body(&json!({
        "username": "cliente1",
        "password": "senha123",
    }))
    .expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[tokio::test]
async fn planos_creditos_valida_faixa_sem_rede() {
    let erro = cliente()
        .planos_creditos(3, -1)
        .await
        .expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));

    let erro = cliente()
        .planos_creditos(3, 101)
        .await
        .expect_err("deve falhar");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[tokio::test]
async fn planos_creditos_remover_exige_confirmacao() {
    let erro = cliente()
        .planos_creditos_remover(3, false)
        .await
        .expect_err("deve falhar");
    assert!(matches!(erro, ApiError::ConfirmationRequired(_)));
}

// ---------------------------------------------------------------------------
// Criacao/edicao/toggle/remocao de revenda (mesma rota da listagem)
// ---------------------------------------------------------------------------

#[test]
fn criar_revenda_exige_minimo_de_dez_creditos() {
    let erro = build_revenda_nova_body(&json!({
        "username": "nova", "password": "1234", "credits": 9
    }))
    .expect_err("creditos abaixo do minimo");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn criar_revenda_ilimitada_zera_creditos() {
    let corpo = build_revenda_nova_body(&json!({
        "username": " nova ",
        "password": "1234",
        "unlimited": 1,
        "credits": 100,
        "pode_criar_ilimitado": 1,
        "pode_criar_revenda": true,
        "pode_conectar_whatsapp": "1"
    }))
    .expect("corpo valido");
    assert_eq!(
        corpo,
        json!({
            "username": "nova",
            "password": "1234",
            "credits": 0,
            "unlimited": 1,
            "pode_criar_revenda": 1,
            "pode_criar_ilimitado": 1,
            "pode_conectar_whatsapp": 1
        })
    );
}

#[test]
fn criar_revenda_limitada_aplica_defaults_do_painel() {
    let corpo = build_revenda_nova_body(&json!({
        "username": "nova", "password": "abcd", "credits": 10, "pode_criar_ilimitado": 1
    }))
    .expect("corpo valido");
    assert_eq!(corpo["unlimited"], 0);
    assert_eq!(corpo["pode_criar_revenda"], 1);
    // Revenda finita nunca nasce com permissao de criar ilimitadas.
    assert_eq!(corpo["pode_criar_ilimitado"], 0);
    assert_eq!(corpo["pode_conectar_whatsapp"], 0);
}

#[test]
fn criar_revenda_aceita_senha_como_alias() {
    let corpo = build_revenda_nova_body(&json!({
        "username": "nova", "senha": "1234", "credits": 10
    }))
    .expect("alias senha");
    assert_eq!(corpo["password"], "1234");
}

#[test]
fn criar_revenda_exige_senha_de_quatro_caracteres() {
    let erro = build_revenda_nova_body(&json!({
        "username": "nova", "password": "123", "credits": 10
    }))
    .expect_err("senha curta");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn editar_revenda_rejeita_grupos_misturados() {
    let erro = build_revenda_editar_body(&json!({
        "id": 7, "transfer": 10, "set_pode_criar_revenda": 0
    }))
    .expect_err("grupos misturados");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn editar_revenda_monta_grupo_de_acesso_com_senha() {
    let corpo = build_revenda_editar_body(&json!({
        "id": 7, "usuario_acesso": "novo.login", "password": "1234"
    }))
    .expect("acesso + senha");
    assert_eq!(
        corpo,
        json!({"id": 7, "usuario_acesso": "novo.login", "password": "1234"})
    );
}

#[test]
fn editar_revenda_exige_transferencia_minima() {
    let erro = build_revenda_editar_body(&json!({"id": 7, "transfer": 9}))
        .expect_err("transferencia curta");
    assert!(matches!(erro, ApiError::Config(_)));
}

#[test]
fn vencimento_em_dias_tem_formato_iso() {
    let data = vencimento_em_dias(30);
    assert_eq!(data.len(), 10);
    assert_eq!(data.as_bytes()[4], b'-');
    assert_eq!(data.as_bytes()[7], b'-');
    assert!(data[..4].parse::<i32>().is_ok());
}

#[tokio::test]
async fn remover_revenda_exige_confirmacao_sem_rede() {
    let erro = cliente()
        .revendas_remover(7, false)
        .await
        .expect_err("sem confirm");
    assert!(matches!(erro, ApiError::ConfirmationRequired(_)));
}

#[tokio::test]
async fn toggle_e_remocao_validam_id_antes_da_rede() {
    let erro = cliente()
        .revendas_toggle(0, true)
        .await
        .expect_err("id invalido");
    assert!(matches!(erro, ApiError::Config(_)));

    let erro = cliente()
        .revendas_remover(-1, true)
        .await
        .expect_err("id invalido");
    assert!(matches!(erro, ApiError::Config(_)));
}
