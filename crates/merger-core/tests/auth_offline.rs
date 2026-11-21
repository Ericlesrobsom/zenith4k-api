//! Testes offline da autenticacao (sem rede, sem painel vivo).
//!
//! Cobrem a interpretacao pura das respostas de login documentadas no Go
//! (`cliente_painel.go` e `merger_login.go`) e a montagem dos payloads.

use merger_core::auth::{build_login_body, interpret_login_body, LoginOutcome};
use merger_core::{ApiError, PanelRole};
use serde_json::{json, Value};

fn interpretar(status: u16, body: Value) -> Result<LoginOutcome, ApiError> {
    interpret_login_body(status, &body)
}

#[test]
fn sucesso_devolve_ok_sem_whoami_ainda() {
    let out = interpretar(200, json!({ "ok": true })).expect("resposta de sucesso");
    assert_eq!(
        out,
        LoginOutcome::Ok {
            whoami: Value::Null
        }
    );
}

#[test]
fn senha_errada_e_rejeicao_sem_vencimento() {
    let out = interpretar(
        200,
        json!({ "ok": false, "erro": "Credenciais incorretas" }),
    )
    .expect("rejeicao e um estado, nao erro de transporte");

    assert_eq!(
        out,
        LoginOutcome::Rejected {
            erro: "Credenciais incorretas".to_string(),
            vencido: false,
        }
    );
}

#[test]
fn usuario_fora_da_franquia_preserva_mensagem() {
    let out = interpretar(
        200,
        json!({ "ok": false, "erro": "Usuário não pertence a esta franquia" }),
    )
    .expect("rejeicao");

    match out {
        LoginOutcome::Rejected { erro, vencido } => {
            assert_eq!(erro, "Usuário não pertence a esta franquia");
            assert!(!vencido);
        }
        outro => panic!("esperava Rejected, veio {outro:?}"),
    }
}

#[test]
fn dois_fatores_pede_codigo_com_email_mascarado() {
    let out = interpretar(
        200,
        json!({
            "ok": false,
            "precisa_2fa": true,
            "email_mascarado": "d***@exemplo.com"
        }),
    )
    .expect("desafio 2FA e um estado normal");

    assert_eq!(
        out,
        LoginOutcome::TwoFactorRequired {
            email_mascarado: Some("d***@exemplo.com".to_string()),
        }
    );
}

#[test]
fn dois_fatores_sem_email_mascarado() {
    let out = interpretar(200, json!({ "ok": false, "precisa_2fa": true })).expect("desafio 2FA");

    assert_eq!(
        out,
        LoginOutcome::TwoFactorRequired {
            email_mascarado: None
        }
    );
}

#[test]
fn codigo_invalido_vira_erro_dedicado() {
    let err = interpretar(
        200,
        json!({
            "ok": false,
            "precisa_2fa": true,
            "erro": "codigo_invalido",
            "email_mascarado": "d***@exemplo.com"
        }),
    )
    .expect_err("codigo invalido deve falhar");

    assert!(
        matches!(err, ApiError::InvalidTwoFactor),
        "esperava InvalidTwoFactor, veio {err:?}"
    );
}

#[test]
fn falha_de_envio_do_2fa_vira_erro_de_painel() {
    let err = interpretar(
        200,
        json!({
            "ok": false,
            "precisa_2fa": true,
            "erro": "falha_envio",
            "email_mascarado": "d***@exemplo.com"
        }),
    )
    .expect_err("falha de envio nao pode pedir codigo");

    match err {
        ApiError::Panel { status, body } => {
            assert_eq!(status, 200);
            assert!(body.contains("codigo 2FA"), "body: {body}");
        }
        outro => panic!("esperava Panel, veio {outro:?}"),
    }
}

#[test]
fn vencido_sem_erro_usa_mensagem_local() {
    let out = interpretar(
        200,
        json!({
            "ok": false,
            "vencido": true,
            "pagamento": { "valor": 5000, "pix": "00020126..." }
        }),
    )
    .expect("vencimento e um estado");

    match out {
        LoginOutcome::Rejected { erro, vencido } => {
            assert!(vencido);
            assert!(erro.contains("vencimento"), "erro: {erro}");
        }
        outro => panic!("esperava Rejected vencido, veio {outro:?}"),
    }
}

#[test]
fn vencido_do_painel_admin_preserva_mensagem() {
    let mensagem = "Seu painel venceu. Fale com o administrador para renovar.";
    let out = interpretar(
        200,
        json!({
            "ok": false,
            "vencido": true,
            "painel_admin": true,
            "erro": mensagem
        }),
    )
    .expect("vencimento");

    assert_eq!(
        out,
        LoginOutcome::Rejected {
            erro: mensagem.to_string(),
            vencido: true,
        }
    );
}

#[test]
fn vencido_de_franquia_descendente_tem_mensagem_propria() {
    let out = interpretar(
        200,
        json!({
            "ok": false,
            "vencido": true,
            "painel_franquia": true
        }),
    )
    .expect("vencimento");

    match out {
        LoginOutcome::Rejected { erro, vencido } => {
            assert!(vencido);
            assert!(erro.contains("fornecedor"), "erro: {erro}");
        }
        outro => panic!("esperava Rejected vencido, veio {outro:?}"),
    }
}

#[test]
fn rate_limit_vira_erro_sem_retry() {
    let err = interpretar(429, json!({ "erro": "rate_limited" })).expect_err("429 deve falhar");

    assert!(
        matches!(
            err,
            ApiError::RateLimited {
                retry_after_secs: None
            }
        ),
        "esperava RateLimited, veio {err:?}"
    );
}

#[test]
fn status_inesperado_vira_erro_de_painel() {
    let err = interpretar(500, json!({ "erro": "boom" })).expect_err("500 deve falhar");

    match err {
        ApiError::Panel { status, body } => {
            assert_eq!(status, 500);
            assert!(body.contains("boom"), "body: {body}");
        }
        outro => panic!("esperava Panel, veio {outro:?}"),
    }
}

#[test]
fn resposta_sem_json_e_rejeitada() {
    let err = interpretar(200, Value::Null).expect_err("HTML/null nao e login valido");

    match err {
        ApiError::Panel { status, body } => {
            assert_eq!(status, 200);
            assert!(body.contains("sem JSON"), "body: {body}");
        }
        outro => panic!("esperava Panel, veio {outro:?}"),
    }
}

#[test]
fn rejeicao_generica_tem_mensagem_de_fallback() {
    let out = interpretar(200, json!({ "ok": false })).expect("estado de rejeicao");

    assert_eq!(
        out,
        LoginOutcome::Rejected {
            erro: "login recusado pelo painel".to_string(),
            vencido: false,
        }
    );
}

#[test]
fn body_de_franquia_tem_apenas_os_campos_do_contrato() {
    let body = build_login_body(
        PanelRole::Franchise,
        "revenda1",
        "segredo",
        Some("matriz"),
        None,
    );

    assert_eq!(
        body,
        json!({
            "username": "revenda1",
            "password": "segredo",
            "franquia": "matriz"
        })
    );
}

#[test]
fn body_de_franquia_omite_franquia_vazia_e_inclui_codigo() {
    let body = build_login_body(
        PanelRole::Franchise,
        "revenda1",
        "segredo",
        Some("   "),
        Some(" 123456 "),
    );

    assert_eq!(
        body,
        json!({
            "username": "revenda1",
            "password": "segredo",
            "codigo": "123456"
        })
    );
}

#[test]
fn body_de_admin_usa_user_e_pass() {
    let body = build_login_body(
        PanelRole::Admin,
        "mestre",
        "segredo",
        Some("ignorado"),
        None,
    );

    assert_eq!(
        body,
        json!({
            "user": "mestre",
            "pass": "segredo"
        })
    );
}

#[test]
fn body_de_admin_com_codigo() {
    let body = build_login_body(PanelRole::Admin, "mestre", "segredo", None, Some("654321"));

    assert_eq!(
        body,
        json!({
            "user": "mestre",
            "pass": "segredo",
            "codigo": "654321"
        })
    );
}

#[test]
fn codigo_vazio_nao_vai_no_corpo() {
    let body = build_login_body(PanelRole::Admin, "mestre", "segredo", None, Some("  "));

    assert_eq!(body.get("codigo"), None);
}
