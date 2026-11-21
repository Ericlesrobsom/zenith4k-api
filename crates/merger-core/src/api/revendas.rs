//! Revendas / sub-revendas (arvore da franquia).
//!
//! Rotas (fonte: `routes_revendas.go`, `merger_api_revendas.go`,
//! `cliente_transfer_revenda.go`):
//! - `GET  /merger/api/cliente/revendas`
//! - `GET  /merger/api/cliente/revendas/arvore`
//! - `POST /merger/api/cliente/revendas/renovar`
//! - `GET  /merger/api/cliente/revendas/nota?id=` (leitura)
//! - `POST /merger/api/cliente/revendas/nota`
//! - `POST /merger/api/cliente/revenda_unlimited`
//! - `POST /merger/api/cliente/transfer_revenda`
//! - `GET  /merger/api/cliente/transfer_revenda/destinos?origem=`
//!
//! Observacao: `/revendas/renovar` decodifica o corpo com
//! `DisallowUnknownFields`, entao o helper `build_revenda_renovar_body` monta
//! exatamente `{id, vencimento_atual}`.

use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

impl MergerClient {
    /// Lista as revendas visiveis para a franquia autenticada.
    ///
    /// O painel ignora filtros nessa rota; `query` existe para simetria e
    /// repasse de parametros futuros.
    pub async fn revendas_list(&self, query: &[(String, String)]) -> Result<Value> {
        self.send("GET", "/merger/api/cliente/revendas", query, None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Arvore hierarquica de revendas (com contagens agregadas por nivel).
    pub async fn revendas_tree(&self) -> Result<Value> {
        self.send(
            "GET",
            "/merger/api/cliente/revendas/arvore",
            &[],
            None,
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Renova manualmente uma sub-revenda por mais 30 dias (sem cobranca).
    ///
    /// `body` aceita `{id, vencimento_atual}`. `vencimento_atual` deve ser a
    /// data exibida na lista (`YYYY-MM-DD`) ou `""` para pular a conferencia.
    pub async fn revendas_renovar(&self, body: Value) -> Result<Value> {
        let body = build_revenda_renovar_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/revendas/renovar",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Grava nota/contato interno na revenda.
    ///
    /// `body` aceita `id` ou `username` como alvo, mais os campos opcionais
    /// `contato_nome`, `contato_email`, `contato_telegram`, `contato_whatsapp`,
    /// `contato_observacoes` e `vencimento`.
    pub async fn revendas_nota(&self, body: Value) -> Result<Value> {
        let body = build_revenda_nota_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/revendas/nota",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Le a nota/contato atual da revenda (GET informativo).
    pub async fn revendas_nota_ler(&self, id: i64) -> Result<Value> {
        if id <= 0 {
            return Err(ApiError::Config(
                "informe o id da revenda (inteiro maior que zero)".into(),
            ));
        }
        let query = vec![("id".to_string(), id.to_string())];
        self.send(
            "GET",
            "/merger/api/cliente/revendas/nota",
            &query,
            None,
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Liga/desliga o modo ilimitado de uma revenda.
    ///
    /// `body` aceita `{id, novo_estado, saldo_rebaixamento?}`; `novo_estado`
    /// deve ser `1` (promover) ou `0` (rebaixar).
    pub async fn revenda_unlimited(&self, body: Value) -> Result<Value> {
        let body = build_revenda_unlimited_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/revenda_unlimited",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Destinos possiveis para transferir uma revenda.
    ///
    /// O painel exige `origem` (username da revenda a mover) e so aceita
    /// chamadas da franquia raiz.
    pub async fn transfer_revenda_destinos(&self, origem: &str) -> Result<Value> {
        let origem = origem.trim();
        if origem.is_empty() {
            return Err(ApiError::Config(
                "informe a origem (username da revenda a transferir)".into(),
            ));
        }
        let query = vec![("origem".to_string(), origem.to_string())];
        self.send(
            "GET",
            "/merger/api/cliente/transfer_revenda/destinos",
            &query,
            None,
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Move uma revenda para outro pai da arvore.
    ///
    /// Operacao estrutural/destrutiva: mover a raiz entre franquias descarta
    /// recursos de marca/operacao. O painel nao pede confirmacao; a checagem
    /// de permissao (franquia raiz) e feita no servidor.
    pub async fn transfer_revenda(&self, body: Value) -> Result<Value> {
        let body = build_transfer_revenda_body(&body)?;
        self.send(
            "POST",
            "/merger/api/cliente/transfer_revenda",
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }
}

/// Corpo de `POST /revendas/renovar` (o painel rejeita campos extras).
pub fn build_revenda_renovar_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let id = inteiro_positivo(obj, "id")?;
    let vencimento = obj
        .get("vencimento_atual")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            ApiError::Config(
                "campo vencimento_atual obrigatorio (YYYY-MM-DD ou \"\" para ignorar)".into(),
            )
        })?;

    Ok(json!({ "id": id, "vencimento_atual": vencimento }))
}

/// Corpo de `POST /revendas/nota` (alvo por `id` ou `username`).
pub fn build_revenda_nota_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;

    let id = inteiro_opcional(obj, "id")?;
    if let Some(valor) = id {
        if valor <= 0 {
            return Err(ApiError::Config("campo id deve ser maior que zero".into()));
        }
    }
    let username = texto_opcional(obj, "username")?;
    if id.is_none() && username.is_none() {
        return Err(ApiError::Config(
            "informe id ou username da revenda alvo".into(),
        ));
    }

    let mut saida = Map::new();
    if let Some(valor) = id {
        saida.insert("id".to_string(), json!(valor));
    }
    if let Some(valor) = username {
        saida.insert("username".to_string(), json!(valor));
    }
    for campo in [
        "contato_nome",
        "contato_email",
        "contato_telegram",
        "contato_whatsapp",
        "contato_observacoes",
        "vencimento",
    ] {
        if let Some(valor) = obj.get(campo).and_then(Value::as_str) {
            saida.insert(campo.to_string(), json!(valor));
        }
    }

    Ok(Value::Object(saida))
}

/// Corpo de `POST /revenda_unlimited`.
pub fn build_revenda_unlimited_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let id = inteiro_positivo(obj, "id")?;
    let novo_estado = inteiro_opcional(obj, "novo_estado")?.ok_or_else(|| {
        ApiError::Config("campo novo_estado obrigatorio (1 promove, 0 rebaixa)".into())
    })?;
    if novo_estado != 0 && novo_estado != 1 {
        return Err(ApiError::Config(
            "novo_estado deve ser 0 (rebaixar) ou 1 (promover)".into(),
        ));
    }
    let saldo = inteiro_opcional(obj, "saldo_rebaixamento")?.unwrap_or(0);

    Ok(json!({
        "id": id,
        "novo_estado": novo_estado,
        "saldo_rebaixamento": saldo,
    }))
}

/// Corpo de `POST /transfer_revenda`.
pub fn build_transfer_revenda_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let origem = texto_obrigatorio(obj, "origem")?;
    let novo_pai = texto_obrigatorio(obj, "novo_pai")?;
    let confirmar_cross = match obj.get("confirmar_cross") {
        None | Some(Value::Null) => false,
        Some(Value::Bool(valor)) => *valor,
        Some(_) => {
            return Err(ApiError::Config(
                "campo confirmar_cross deve ser booleano".into(),
            ))
        }
    };

    Ok(json!({
        "origem": origem,
        "novo_pai": novo_pai,
        "confirmar_cross": confirmar_cross,
    }))
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
