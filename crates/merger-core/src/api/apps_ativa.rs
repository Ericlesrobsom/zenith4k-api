//! Apps Ativa (Ative App) — ativacao **paga** por MAC.
//!
//! Nao confundir com Apps Ibo (`msproapp`), que e ativacao sem custo. Aqui a
//! conta tem saldo em R$ (`apps_ativa_saldo_cents`), o painel debita antes de
//! chamar o provedor e estorna (refund) se o provedor recusar.
//!
//! Rotas (fonte: `routes_apps_ativa.go`, `apps_ativa_ativar.go`,
//! `apps_ativa.go`, `apps_ativa_listar.go`):
//! - `GET  /merger/api/cliente/apps_ativa/saldo`
//! - `GET  /merger/api/apps_ativa/aplicativos`
//! - `GET  /merger/api/cliente/apps_ativa/historico` (movimentacoes)
//! - `GET  /merger/api/cliente/apps_ativa/historico_ativacoes` (status)
//! - `POST /merger/api/cliente/apps_ativa/ativar`
//!
//! ## MAC antes do debito
//!
//! O painel debita o saldo **antes** de chamar o provedor e so descobre o
//! formato errado la. Para nao pagar uma ativacao recusada, este cliente
//! normaliza o MAC para `aa:bb:cc:dd:ee:ff` (hex minusculo) **antes** de
//! enviar; MAC invalido vira `ApiError::InvalidMac` (400 `mac_invalido`) sem
//! tocar no saldo.
//!
//! ## Status
//!
//! A ativacao nasce `Solicitado` e so muda quando o Ative App chama o webhook
//! `POST /api/webhook/apps_ativa/{token}` (configurado pelo admin em
//! `GET /merger/api/admin/apps_ativa/webhook_info`). Nao existe rota de
//! refresh/polling no escopo do cliente.

use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

pub const ROTA_SALDO: &str = "/merger/api/cliente/apps_ativa/saldo";
pub const ROTA_ATIVAR: &str = "/merger/api/cliente/apps_ativa/ativar";
pub const ROTA_HISTORICO: &str = "/merger/api/cliente/apps_ativa/historico";
pub const ROTA_ATIVACOES: &str = "/merger/api/cliente/apps_ativa/historico_ativacoes";
pub const ROTA_APPS: &str = "/merger/api/apps_ativa/aplicativos";

impl MergerClient {
    /// Saldo em R$ (centavos) + se a integracao tem API key configurada.
    pub async fn apps_ativa_saldo(&self) -> Result<Value> {
        self.send("GET", ROTA_SALDO, &[], None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Catalogo de aplicativos (preco em centavos para o cliente).
    pub async fn apps_ativa_apps(&self) -> Result<Value> {
        self.send("GET", ROTA_APPS, &[], None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Movimentacoes de credito (50 ultimas).
    pub async fn apps_ativa_historico(&self) -> Result<Value> {
        self.send("GET", ROTA_HISTORICO, &[], None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Ativacoes com status (`Solicitado`, `Ativado`, `Reprovado`, ...).
    ///
    /// `status` opcional filtra exatamente pelo valor informado.
    pub async fn apps_ativa_ativacoes(&self, status: Option<&str>) -> Result<Value> {
        let mut query = Vec::new();
        if let Some(status) = status.map(str::trim).filter(|status| !status.is_empty()) {
            query.push(("status".to_string(), status.to_string()));
        }
        self.send("GET", ROTA_ATIVACOES, &query, None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Solicita uma ativacao paga.
    ///
    /// `body` aceita `{app_uuid, mac_app, key_app?, descricao?}`. O MAC e
    /// normalizado antes do request; invalido vira [`ApiError::InvalidMac`]
    /// sem debitar saldo.
    pub async fn apps_ativa_ativar(&self, body: Value) -> Result<Value> {
        let body = build_apps_ativa_ativar_body(&body)?;
        let resp = self
            .send("POST", ROTA_ATIVAR, &[], Some(&body), false)
            .await?;
        Ok(anotar_sincronizacao(resp.json_or_text()))
    }
}

/// Marca se a ativacao nasceu com `id_cobranca` (sem ele, o webhook do Ative
/// App nao consegue casar a ativacao e o status fica `Solicitado` para sempre).
fn anotar_sincronizacao(mut data: Value) -> Value {
    if data.get("ok").and_then(Value::as_bool) != Some(true) {
        return data;
    }
    let id_cobranca = data
        .get("id_cobranca")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();
    if let Some(obj) = data.as_object_mut() {
        obj.insert(
            "webhook_sincronizavel".to_string(),
            json!(!id_cobranca.is_empty()),
        );
        if id_cobranca.is_empty() {
            obj.insert(
                "aviso".to_string(),
                json!(
                    "ativacao criada sem id_cobranca; o webhook do Ative App nao \
                     conseguira atualizar o status desta ativacao"
                ),
            );
        }
    }
    data
}

/// Corpo de `POST /cliente/apps_ativa/ativar` com MAC normalizado.
pub fn build_apps_ativa_ativar_body(body: &Value) -> Result<Value> {
    let obj = body
        .as_object()
        .ok_or_else(|| ApiError::Config("corpo da requisicao precisa ser um objeto JSON".into()))?;

    let app_uuid = texto_obrigatorio(obj, "app_uuid")?;
    if app_uuid.chars().count() > 64 || app_uuid.chars().any(char::is_whitespace) {
        return Err(ApiError::Config(
            "app_uuid invalido: use o UUID do catalogo (sem espacos)".into(),
        ));
    }

    let mac_bruto = texto_obrigatorio(obj, "mac_app")?;
    let mac = crate::mac::normalizar_mac_provedor(&mac_bruto)?;

    let key_app = texto_opcional(obj, "key_app")?;
    if let Some(key) = key_app.as_deref() {
        validar_key_app(key)?;
    }

    let descricao = texto_opcional(obj, "descricao")?
        .map(|descricao| descricao.chars().take(200).collect::<String>())
        .unwrap_or_default();

    Ok(json!({
        "app_uuid": app_uuid,
        "mac_app": mac,
        "key_app": key_app.unwrap_or_default(),
        "descricao": descricao,
    }))
}

/// Valida a `key_app` sem adivinhar o formato do provedor.
///
/// O campo e opcional e "depende do app" (a UI nao conhece a regra por app).
/// Recusamos apenas o que nunca e chave valida: vazio ja virou `None`, entao
/// aqui sobra espaco/controle, nao-ASCII e tamanho absurdo.
fn validar_key_app(key: &str) -> Result<()> {
    if key.chars().count() > 128 {
        return Err(ApiError::InvalidKeyApp(
            "key_app invalida: maximo de 128 caracteres".into(),
        ));
    }
    if key.chars().any(|caractere| caractere.is_whitespace()) {
        return Err(ApiError::InvalidKeyApp(
            "key_app invalida: nao pode conter espacos".into(),
        ));
    }
    if !key.chars().all(|caractere| caractere.is_ascii_graphic()) {
        return Err(ApiError::InvalidKeyApp(
            "key_app invalida: use apenas caracteres ASCII imprimiveis".into(),
        ));
    }
    Ok(())
}

fn texto_obrigatorio(obj: &Map<String, Value>, campo: &str) -> Result<String> {
    texto_opcional(obj, campo)?
        .ok_or_else(|| ApiError::Config(format!("campo {campo} obrigatorio (string nao vazia)")))
}

fn texto_opcional(obj: &Map<String, Value>, campo: &str) -> Result<Option<String>> {
    match obj.get(campo) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(valor)) => {
            let valor = valor.trim();
            if valor.is_empty() {
                Ok(None)
            } else {
                Ok(Some(valor.to_string()))
            }
        }
        Some(_) => Err(ApiError::Config(format!("campo {campo} deve ser string"))),
    }
}

#[cfg(test)]
mod testes {
    use super::anotar_sincronizacao;
    use serde_json::json;

    #[test]
    fn ativacao_sem_id_cobranca_ganha_aviso() {
        let saida = anotar_sincronizacao(json!({
            "ok": true,
            "ativacao_id": 3,
            "id_cobranca": "",
        }));
        assert_eq!(saida["webhook_sincronizavel"], false);
        assert!(saida["aviso"].as_str().unwrap().contains("id_cobranca"));
    }

    #[test]
    fn ativacao_com_id_cobranca_e_sincronizavel() {
        let saida = anotar_sincronizacao(json!({
            "ok": true,
            "ativacao_id": 4,
            "id_cobranca": "abc-123",
        }));
        assert_eq!(saida["webhook_sincronizavel"], true);
        assert!(saida.get("aviso").is_none());
    }

    #[test]
    fn erro_do_painel_nao_ganha_campos() {
        let saida = anotar_sincronizacao(json!({"ok": false, "erro": "Saldo insuficiente"}));
        assert!(saida.get("webhook_sincronizavel").is_none());
    }
}
