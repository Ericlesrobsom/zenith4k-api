//! Planos, creditos e criacao de linha.
//!
//! Rotas (fonte: `routes_revendas.go`, `merger_api_cliente_painel.go`,
//! `planos_credito_rede.go`):
//! - `GET    /merger/api/cliente/planos` (`?incluir_ocultos=1` para a gestao)
//! - `POST   /merger/api/cliente/planos/creditos`  (`{plano_id, creditos}`)
//! - `DELETE /merger/api/cliente/planos/creditos?plano_id=` (volta ao padrao)
//! - `GET    /merger/api/cliente/creditos`
//! - `POST   /merger/api/cliente/criar_linha`
//!
//! Atencao: o brief citava `GET /planos/creditos`, mas o Go so aceita POST e
//! DELETE nessa rota (GET devolve 405). Implementamos os dois metodos reais.

use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

const PATH_CREDITOS_PLANOS: &str = "/merger/api/cliente/planos/creditos";

impl MergerClient {
    /// Planos visiveis para a franquia (ocultos pela raiz ficam de fora).
    pub async fn planos_list(&self) -> Result<Value> {
        self.send("GET", "/merger/api/cliente/planos", &[], None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Planos visiveis incluindo os ocultados pela franquia raiz.
    ///
    /// Equivale a `GET /planos?incluir_ocultos=1` (tela de gestao de planos).
    pub async fn planos_list_incluindo_ocultos(&self) -> Result<Value> {
        let query = vec![("incluir_ocultos".to_string(), "1".to_string())];
        self.send("GET", "/merger/api/cliente/planos", &query, None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Ajusta o credito de rede de um plano para a arvore da franquia raiz.
    ///
    /// Substitui a leitura inexistente (`GET`) do brief: o painel so aceita
    /// `POST {plano_id, creditos}` e `DELETE ?plano_id=`.
    pub async fn planos_creditos(&self, plano_id: i64, creditos: i64) -> Result<Value> {
        if plano_id <= 0 {
            return Err(ApiError::Config("plano_id deve ser maior que zero".into()));
        }
        if !(0..=100).contains(&creditos) {
            return Err(ApiError::Config(
                "creditos deve ficar entre 0 e 100 (planoCreditosRedeMax)".into(),
            ));
        }
        let body = json!({ "plano_id": plano_id, "creditos": creditos });
        self.send("POST", PATH_CREDITOS_PLANOS, &[], Some(&body), false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Remove o credito de rede do plano (volta ao valor padrao do plano).
    ///
    /// A rota usa `DELETE`; o cliente exige `confirm = true` para qualquer
    /// DELETE, mesmo sendo um "reset" reversivel.
    pub async fn planos_creditos_remover(&self, plano_id: i64, confirm: bool) -> Result<Value> {
        if plano_id <= 0 {
            return Err(ApiError::Config("plano_id deve ser maior que zero".into()));
        }
        if !confirm {
            return Err(ApiError::ConfirmationRequired(format!(
                "{PATH_CREDITOS_PLANOS}?plano_id={plano_id}"
            )));
        }
        let query = vec![("plano_id".to_string(), plano_id.to_string())];
        self.send("DELETE", PATH_CREDITOS_PLANOS, &query, None, true)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Saldo consolidado da franquia (`{ok, credits, unlimited}`).
    pub async fn creditos(&self) -> Result<Value> {
        self.send("GET", "/merger/api/cliente/creditos", &[], None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Cria uma linha Xtream (cliente final) debitando o plano.
    ///
    /// `body` segue os nomes do painel: `username`, `password`, `plano_id`
    /// (obrigatorios) e os opcionais `valor_personalizado`, `contato_nome`,
    /// `contato_whatsapp`, `contato_telegram`, `contato_email`, `observacoes`,
    /// `servidor_id`, `renovacao_qr`.
    pub async fn criar_linha(&self, body: Value) -> Result<Value> {
        let body = build_criar_linha_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/criar_linha",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }
}

/// Corpo de `POST /merger/api/cliente/criar_linha`.
pub fn build_criar_linha_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let username = texto_obrigatorio(obj, "username")?;
    let password = texto_obrigatorio(obj, "password")?;
    let plano_id = inteiro_positivo(obj, "plano_id")?;

    let mut saida = Map::new();
    saida.insert("username".to_string(), json!(username));
    saida.insert("password".to_string(), json!(password));
    saida.insert("plano_id".to_string(), json!(plano_id));

    if let Some(valor) = numero_opcional(obj, "valor_personalizado")? {
        saida.insert("valor_personalizado".to_string(), json!(valor));
    }
    for campo in [
        "contato_nome",
        "contato_whatsapp",
        "contato_telegram",
        "contato_email",
        "observacoes",
    ] {
        if let Some(valor) = obj.get(campo).and_then(Value::as_str) {
            saida.insert(campo.to_string(), json!(valor));
        }
    }
    if let Some(servidor) = inteiro_opcional(obj, "servidor_id")? {
        saida.insert("servidor_id".to_string(), json!(servidor));
    }
    if let Some(renovacao_qr) = obj.get("renovacao_qr") {
        let valor = renovacao_qr
            .as_bool()
            .ok_or_else(|| ApiError::Config("campo renovacao_qr deve ser booleano".into()))?;
        saida.insert("renovacao_qr".to_string(), json!(valor));
    }

    Ok(Value::Object(saida))
}

fn objeto(body: &Value) -> Result<&Map<String, Value>> {
    body.as_object()
        .ok_or_else(|| ApiError::Config("corpo da requisicao precisa ser um objeto JSON".into()))
}

fn texto_obrigatorio(obj: &Map<String, Value>, campo: &str) -> Result<String> {
    match obj.get(campo).and_then(Value::as_str).map(str::trim) {
        Some(valor) if !valor.is_empty() => Ok(valor.to_string()),
        _ => Err(ApiError::Config(format!(
            "campo {campo} obrigatorio (string nao vazia)"
        ))),
    }
}

fn inteiro_opcional(obj: &Map<String, Value>, campo: &str) -> Result<Option<i64>> {
    match obj.get(campo) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(numero)) => numero
            .as_i64()
            .map(Some)
            .ok_or_else(|| ApiError::Config(format!("campo {campo} deve ser inteiro"))),
        Some(Value::String(texto)) if texto.trim().is_empty() => Ok(None),
        Some(Value::String(texto)) => texto
            .trim()
            .parse::<i64>()
            .map(Some)
            .map_err(|_| ApiError::Config(format!("campo {campo} deve ser inteiro"))),
        Some(_) => Err(ApiError::Config(format!("campo {campo} deve ser inteiro"))),
    }
}

fn inteiro_positivo(obj: &Map<String, Value>, campo: &str) -> Result<i64> {
    match inteiro_opcional(obj, campo)? {
        Some(valor) if valor > 0 => Ok(valor),
        Some(_) => Err(ApiError::Config(format!(
            "campo {campo} deve ser maior que zero"
        ))),
        None => Err(ApiError::Config(format!(
            "campo {campo} obrigatorio (inteiro positivo)"
        ))),
    }
}

fn numero_opcional(obj: &Map<String, Value>, campo: &str) -> Result<Option<f64>> {
    match obj.get(campo) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(numero)) => numero
            .as_f64()
            .map(Some)
            .ok_or_else(|| ApiError::Config(format!("campo {campo} deve ser numero"))),
        Some(Value::String(texto)) => texto
            .trim()
            .replace(',', ".")
            .parse::<f64>()
            .map(Some)
            .map_err(|_| ApiError::Config(format!("campo {campo} deve ser numero"))),
        Some(_) => Err(ApiError::Config(format!("campo {campo} deve ser numero"))),
    }
}
