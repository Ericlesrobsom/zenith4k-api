//! Usuarios / clientes finais (linhas Xtream gerenciadas pela franquia).
//!
//! Rotas (fonte: `routes_sigma.go`, `cliente_sigma_users*.go`):
//! - `GET  /merger/api/cliente/sigma_users`
//! - `GET  /merger/api/cliente/sigma_users/revendas`
//! - `POST /merger/api/cliente/sigma_users/criar`    (parametros na query)
//! - `POST /merger/api/cliente/sigma_users/editar`   (corpo JSON)
//! - `POST /merger/api/cliente/sigma_users/toggle`   (parametros na query)
//! - `POST /merger/api/cliente/sigma_users/renovar`  (corpo JSON)
//! - `POST /merger/api/cliente/sigma_users/estender` (corpo JSON)
//! - `POST /merger/api/cliente/sigma_users/delete`   (parametros na query)
//! - `POST /merger/api/cliente/sigma_users/telas`    (corpo JSON)
//! - `POST /merger/api/cliente/sigma_users/valor`    (corpo JSON)
//! - `POST /merger/api/cliente/sigma_users/adultos`  (parametros na query)
//! - `POST /merger/api/cliente/sigma_users/contato`  (corpo JSON)
//! - `GET  /merger/api/cliente/sigma_users/contato?id=`
//!
//! Particularidade do painel: `criar`, `toggle`, `delete` e `adultos` leem os
//! dados da **query string**, nao do corpo. Os helpers `build_*` fazem essa
//! traducao a partir de um objeto JSON ergonomico e validam os campos.

use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

const BASE: &str = "/merger/api/cliente/sigma_users";

impl MergerClient {
    /// Lista/busca clientes. `query` aceita os filtros reais do painel:
    /// `page`, `per_page` (20/50/100), `status`, `search`, `dono`, `escopo`,
    /// `servidor`, `servidor_cat`, `vence`, `vence_de`, `vence_ate`, `plano`.
    pub async fn usuarios_list(&self, query: &[(String, String)]) -> Result<Value> {
        self.send("GET", BASE, query, None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Lista apenas as revendas da subarvore (para selects/filtros).
    pub async fn usuarios_revendas(&self) -> Result<Value> {
        self.send(
            "GET",
            "/merger/api/cliente/sigma_users/revendas",
            &[],
            None,
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Cria uma linha/cliente.
    ///
    /// O painel le os dados na query string; o `body` aceito aqui segue os
    /// nomes do painel (`username`, `password`, `max_connections`, `dias`,
    /// `is_trial`, `bouquet`, `servidor_id`).
    pub async fn usuarios_criar(&self, body: Value) -> Result<Value> {
        let query = build_usuario_criar_query(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/sigma_users/criar",
            &query,
            None,
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Edita username, senha e/ou validade de um cliente.
    pub async fn usuarios_editar(&self, body: Value) -> Result<Value> {
        let body = build_usuario_editar_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/sigma_users/editar",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Liga/desliga o cliente. `acao` deve ser `bloquear` ou `desbloquear`.
    pub async fn usuarios_toggle(&self, body: Value) -> Result<Value> {
        let query = build_usuario_toggle_query(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/sigma_users/toggle",
            &query,
            None,
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Renova o cliente com um plano (debita creditos do plano).
    pub async fn usuarios_renovar(&self, body: Value) -> Result<Value> {
        let body = build_usuario_renovar_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/sigma_users/renovar",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Estende o vencimento sem trocar o plano (cobra credito proporcional).
    pub async fn usuarios_estender(&self, body: Value) -> Result<Value> {
        let body = build_usuario_estender_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/sigma_users/estender",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Remove um cliente. Operacao destrutiva e irreversivel (vai para a
    /// lixeira do painel): exige `confirm = true`.
    pub async fn usuarios_delete(&self, body: Value, confirm: bool) -> Result<Value> {
        if !confirm {
            return Err(ApiError::ConfirmationRequired(format!(
                "/merger/api/cliente/sigma_users/delete (id {})",
                body.get("id").map(|v| v.to_string()).unwrap_or_default()
            )));
        }
        let query = build_usuario_delete_query(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/sigma_users/delete",
            &query,
            None,
            true,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Ajusta o numero de telas (cobra credito proporcional ao aumentar).
    pub async fn usuarios_telas(&self, body: Value) -> Result<Value> {
        let body = build_usuario_telas_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/sigma_users/telas",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Ajusta o valor de cobranca personalizado da linha (`valor <= 0` limpa).
    pub async fn usuarios_valor(&self, body: Value) -> Result<Value> {
        let body = build_usuario_valor_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/sigma_users/valor",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Liga/desliga conteudo adulto. `acao` deve ser `ligar` ou `desligar`.
    pub async fn usuarios_adultos(&self, body: Value) -> Result<Value> {
        let query = build_usuario_adultos_query(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/sigma_users/adultos",
            &query,
            None,
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Atualiza contato/whatsapp/senha do cliente (POST).
    pub async fn usuarios_contato(&self, body: Value) -> Result<Value> {
        let body = build_usuario_contato_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/sigma_users/contato",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Le os dados de contato e a senha atual do cliente.
    pub async fn usuarios_contato_ler(&self, id: i64) -> Result<Value> {
        if id <= 0 {
            return Err(ApiError::Config(
                "informe o id do cliente (inteiro maior que zero)".into(),
            ));
        }
        let query = vec![("id".to_string(), id.to_string())];
        self.send(
            "GET",
            "/merger/api/cliente/sigma_users/contato",
            &query,
            None,
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }
}

/// Query de `POST .../sigma_users/criar` (o painel nao le corpo nessa rota).
pub fn build_usuario_criar_query(body: &Value) -> Result<Vec<(String, String)>> {
    let obj = objeto(body)?;
    let username = texto_obrigatorio(obj, "username")?;
    let password = texto_obrigatorio(obj, "password")?;

    let mut query = vec![
        ("username".to_string(), username),
        ("password".to_string(), password),
    ];

    for campo in ["max_connections", "dias", "is_trial", "servidor_id"] {
        if let Some(valor) = inteiro_opcional(obj, campo)? {
            query.push((campo.to_string(), valor.to_string()));
        }
    }
    if let Some(bouquet) = texto_opcional(obj, "bouquet")? {
        query.push(("bouquet".to_string(), bouquet));
    }

    Ok(query)
}

/// Corpo de `POST .../sigma_users/editar`.
pub fn build_usuario_editar_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let user_id = inteiro_positivo(obj, "user_id")?;

    let mut saida = Map::new();
    saida.insert("user_id".to_string(), json!(user_id));
    for campo in ["novo_username", "nova_senha", "validade"] {
        if let Some(valor) = obj.get(campo).and_then(Value::as_str) {
            saida.insert(campo.to_string(), json!(valor));
        }
    }

    Ok(Value::Object(saida))
}

/// Query de `POST .../sigma_users/toggle`.
pub fn build_usuario_toggle_query(body: &Value) -> Result<Vec<(String, String)>> {
    let obj = objeto(body)?;
    let id = inteiro_positivo(obj, "id")?;
    let acao = texto_obrigatorio(obj, "acao")?;
    if acao != "bloquear" && acao != "desbloquear" {
        return Err(ApiError::Config(
            "acao deve ser \"bloquear\" ou \"desbloquear\"".into(),
        ));
    }

    Ok(vec![
        ("id".to_string(), id.to_string()),
        ("acao".to_string(), acao),
    ])
}

/// Corpo de `POST .../sigma_users/renovar`.
pub fn build_usuario_renovar_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let user_id = inteiro_positivo(obj, "user_id")?;
    let plano_id = inteiro_positivo(obj, "plano_id")?;
    Ok(json!({ "user_id": user_id, "plano_id": plano_id }))
}

/// Corpo de `POST .../sigma_users/estender` (dias entre 1 e 3650).
pub fn build_usuario_estender_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let user_id = inteiro_positivo(obj, "user_id")?;
    let dias = inteiro_positivo(obj, "dias")?;
    if dias > 3650 {
        return Err(ApiError::Config("dias deve ficar entre 1 e 3650".into()));
    }
    Ok(json!({ "user_id": user_id, "dias": dias }))
}

/// Query de `POST .../sigma_users/delete`.
pub fn build_usuario_delete_query(body: &Value) -> Result<Vec<(String, String)>> {
    let obj = objeto(body)?;
    let id = inteiro_positivo(obj, "id")?;
    Ok(vec![("id".to_string(), id.to_string())])
}

/// Corpo de `POST .../sigma_users/telas`.
pub fn build_usuario_telas_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let user_id = inteiro_positivo(obj, "user_id")?;
    let telas = inteiro_positivo(obj, "telas")?;
    Ok(json!({ "user_id": user_id, "telas": telas }))
}

/// Corpo de `POST .../sigma_users/valor`.
pub fn build_usuario_valor_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let user_id = inteiro_positivo(obj, "user_id")?;
    let valor = numero_opcional(obj, "valor")?.ok_or_else(|| {
        ApiError::Config("campo valor obrigatorio (numero; <= 0 limpa o custom)".into())
    })?;
    Ok(json!({ "user_id": user_id, "valor": valor }))
}

/// Query de `POST .../sigma_users/adultos`.
pub fn build_usuario_adultos_query(body: &Value) -> Result<Vec<(String, String)>> {
    let obj = objeto(body)?;
    let id = inteiro_positivo(obj, "id")?;
    let acao = texto_obrigatorio(obj, "acao")?;
    if acao != "ligar" && acao != "desligar" {
        return Err(ApiError::Config(
            "acao deve ser \"ligar\" ou \"desligar\"".into(),
        ));
    }

    let mut query = vec![
        ("id".to_string(), id.to_string()),
        ("acao".to_string(), acao),
    ];
    if let Some(plano) = inteiro_opcional(obj, "plano_destino_id")? {
        if plano < 0 {
            return Err(ApiError::Config(
                "plano_destino_id nao pode ser negativo".into(),
            ));
        }
        if plano > 0 {
            query.push(("plano_destino_id".to_string(), plano.to_string()));
        }
    }

    Ok(query)
}

/// Corpo de `POST .../sigma_users/contato`.
pub fn build_usuario_contato_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let user_id = inteiro_positivo(obj, "user_id")?;

    let mut saida = Map::new();
    saida.insert("user_id".to_string(), json!(user_id));
    for campo in [
        "novo_username",
        "nova_senha",
        "contato_nome",
        "contato_email",
        "contato_telegram",
        "contato_whatsapp",
        "observacoes",
    ] {
        if let Some(valor) = obj.get(campo).and_then(Value::as_str) {
            saida.insert(campo.to_string(), json!(valor));
        }
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

fn texto_opcional(obj: &Map<String, Value>, campo: &str) -> Result<Option<String>> {
    match obj.get(campo) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(valor)) if valor.trim().is_empty() => Ok(None),
        Some(Value::String(valor)) => Ok(Some(valor.trim().to_string())),
        Some(_) => Err(ApiError::Config(format!("campo {campo} deve ser string"))),
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
