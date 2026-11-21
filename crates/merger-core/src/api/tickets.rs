//! Tickets de suporte (cliente x admin).
//!
//! Rotas (fonte: `routes_tickets.go`, `tickets_suporte.go`,
//! `tickets_admin.go`):
//! - `GET  /merger/api/cliente/tickets/listar` (cliente)
//! - `GET  /merger/api/cliente/tickets/ver?id=N` (cliente)
//! - `POST /merger/api/cliente/tickets/criar` (multipart)
//! - `POST /merger/api/cliente/tickets/responder` (multipart)
//! - `POST /merger/api/cliente/tickets/fechar?id=N`
//! - `GET  /merger/api/admin/tickets/listar?status=` (admin)
//! - `GET  /merger/api/admin/tickets/ver?id=N` (admin)
//! - `POST /merger/api/admin/tickets/responder` (multipart)
//! - `POST /merger/api/admin/tickets/status` (JSON)
//! - `GET  /merger/api/tickets/anexo?msg=N` (JPEG autenticado)
//!
//! ## Multipart
//!
//! As rotas de criacao e resposta sao `multipart/form-data` (campos `assunto`,
//! `texto` e, opcionalmente, o arquivo `imagem`). O cliente usa
//! [`MergerClient::send_multipart`] e aceita a imagem como
//! [`UploadFile`]. O download do JPEG de `tickets/anexo` continua sendo um GET
//! autenticado — use [`MergerClient::raw`] e trate a resposta binaria.
//!
//! Listagem, leitura, fechamento (cliente) e mudanca de status (admin) usam
//! JSON/query.

use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};
use crate::raw::UploadFile;

/// `GET /merger/api/cliente/tickets/listar`
pub const ROTA_TICKETS_CLIENTE_LISTAR: &str = "/merger/api/cliente/tickets/listar";
/// `GET /merger/api/cliente/tickets/ver`
pub const ROTA_TICKETS_CLIENTE_VER: &str = "/merger/api/cliente/tickets/ver";
/// `POST /merger/api/cliente/tickets/criar` (multipart)
pub const ROTA_TICKETS_CLIENTE_CRIAR: &str = "/merger/api/cliente/tickets/criar";
/// `POST /merger/api/cliente/tickets/responder` (multipart)
pub const ROTA_TICKETS_CLIENTE_RESPONDER: &str = "/merger/api/cliente/tickets/responder";
/// `POST /merger/api/cliente/tickets/fechar`
pub const ROTA_TICKETS_CLIENTE_FECHAR: &str = "/merger/api/cliente/tickets/fechar";
/// `GET /merger/api/admin/tickets/listar`
pub const ROTA_TICKETS_ADMIN_LISTAR: &str = "/merger/api/admin/tickets/listar";
/// `GET /merger/api/admin/tickets/ver`
pub const ROTA_TICKETS_ADMIN_VER: &str = "/merger/api/admin/tickets/ver";
/// `POST /merger/api/admin/tickets/responder` (multipart)
pub const ROTA_TICKETS_ADMIN_RESPONDER: &str = "/merger/api/admin/tickets/responder";
/// `POST /merger/api/admin/tickets/status`
pub const ROTA_TICKETS_ADMIN_STATUS: &str = "/merger/api/admin/tickets/status";
/// `GET /merger/api/tickets/anexo`
pub const ROTA_TICKETS_ANEXO: &str = "/merger/api/tickets/anexo";

/// Status aceitos por `tickets/status` (admin).
pub const TICKET_STATUS: [&str; 3] = ["aberto", "respondido", "fechado"];

/// Limite de chamados nao fechados por autor (anti-spam do painel).
pub const TICKET_MAX_ABERTOS: usize = 5;
/// Teto de caracteres do assunto (o painel corta no mesmo valor).
pub const TICKET_MAX_ASSUNTO: usize = 120;
/// Teto de caracteres do texto de uma mensagem.
pub const TICKET_MAX_TEXTO: usize = 4000;

/// Monta a query `id=<id>` de `tickets/ver` e `tickets/fechar`.
pub fn consulta_ticket_id(id: i64) -> Result<Vec<(String, String)>> {
    if id <= 0 {
        return Err(ApiError::Config(
            "tickets: id precisa ser maior que zero".into(),
        ));
    }
    Ok(vec![("id".to_string(), id.to_string())])
}

/// Monta a query de `GET /merger/api/admin/tickets/listar`.
///
/// O painel ignora status fora de [`TICKET_STATUS`] (lista todos); fazemos o
/// mesmo, omitindo o parametro invalido.
pub fn consulta_tickets_admin(status: Option<&str>) -> Vec<(String, String)> {
    match status {
        Some(status) => {
            let status = status.trim().to_ascii_lowercase();
            if TICKET_STATUS.contains(&status.as_str()) {
                vec![("status".to_string(), status)]
            } else {
                Vec::new()
            }
        }
        None => Vec::new(),
    }
}

/// Valida os campos de abertura de chamado (`assunto`, `texto`).
///
/// O painel trunca assunto em 120 e texto em 4000 caracteres; fazemos o mesmo
/// corte local. Anexo de imagem nao e suportado por este transporte.
pub fn normalizar_ticket_novo(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "tickets_criar")?;
    let assunto = truncar(&texto_obrigatorio(obj, "assunto")?, TICKET_MAX_ASSUNTO);
    let texto = truncar(&texto_obrigatorio(obj, "texto")?, TICKET_MAX_TEXTO);
    Ok(json!({
        "assunto": assunto,
        "texto": texto,
    }))
}

/// Valida os campos de resposta de chamado (`id`, `texto`).
pub fn normalizar_ticket_resposta(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "tickets_responder")?;
    let id = inteiro_positivo(obj, "id")?;
    let texto = truncar(&texto_obrigatorio(obj, "texto")?, TICKET_MAX_TEXTO);
    Ok(json!({
        "id": id,
        "texto": texto,
    }))
}

/// Valida e normaliza o corpo de `POST /merger/api/admin/tickets/status`.
pub fn normalizar_ticket_status(body: Value) -> Result<Value> {
    let obj = exigir_objeto(&body, "tickets_status")?;
    let id = inteiro_positivo(obj, "id")?;
    let status = exigir_enum(obj, "status", &TICKET_STATUS)?;
    Ok(json!({
        "id": id,
        "status": status,
    }))
}

impl MergerClient {
    /// Lista os chamados do autor (visao cliente).
    ///
    /// Resposta: `{"ok":true,"tickets":[...]}` com `nao_lidas` por ticket.
    /// Chamado e privado do autor: a franquia nao ve o ticket da sub-revenda.
    pub async fn tickets_listar(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_TICKETS_CLIENTE_LISTAR, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Abre o chamado e devolve a thread (visao cliente).
    ///
    /// Use [`consulta_ticket_id`] para montar a query. Marca o chamado como
    /// lido pelo cliente. Resposta: `{"ok":true,"ticket":{...,"mensagens":[]}}`.
    pub async fn tickets_ver(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_TICKETS_CLIENTE_VER, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Cria um chamado (cliente).
    ///
    /// `body` aceita `{assunto, texto}`; `imagem` e opcional (campo `imagem`).
    /// Resposta: `{"ok":true,"id":N}`.
    pub async fn tickets_criar(&self, body: Value, imagem: Option<UploadFile>) -> Result<Value> {
        let campos = normalizar_ticket_novo(body)?;
        let fields = pares_do_objeto(&campos);
        let files: Vec<UploadFile> = imagem.into_iter().collect();
        let resp = self
            .send_multipart(ROTA_TICKETS_CLIENTE_CRIAR, &fields, &files, false)
            .await?;
        Ok(resp.body)
    }

    /// Responde um chamado (cliente).
    ///
    /// `body` aceita `{id, texto}`; `imagem` e opcional.
    pub async fn tickets_responder(
        &self,
        body: Value,
        imagem: Option<UploadFile>,
    ) -> Result<Value> {
        let campos = normalizar_ticket_resposta(body)?;
        let fields = pares_do_objeto(&campos);
        let files: Vec<UploadFile> = imagem.into_iter().collect();
        let resp = self
            .send_multipart(ROTA_TICKETS_CLIENTE_RESPONDER, &fields, &files, false)
            .await?;
        Ok(resp.body)
    }

    /// Responde um chamado (visao admin).
    pub async fn tickets_responder_admin(
        &self,
        body: Value,
        imagem: Option<UploadFile>,
    ) -> Result<Value> {
        let campos = normalizar_ticket_resposta(body)?;
        let fields = pares_do_objeto(&campos);
        let files: Vec<UploadFile> = imagem.into_iter().collect();
        let resp = self
            .send_multipart(ROTA_TICKETS_ADMIN_RESPONDER, &fields, &files, false)
            .await?;
        Ok(resp.body)
    }

    /// Fecha o proprio chamado (`POST .../fechar?id=N`).
    ///
    /// Use [`consulta_ticket_id`] para montar a query.
    pub async fn tickets_fechar(&self, id: i64) -> Result<Value> {
        let query = consulta_ticket_id(id)?;
        let resp = self
            .send("POST", ROTA_TICKETS_CLIENTE_FECHAR, &query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Lista todos os chamados (visao admin).
    ///
    /// Use [`consulta_tickets_admin`] para montar a query. Resposta:
    /// `{"ok":true,"tickets":[...],"aguardando":N}`.
    pub async fn tickets_listar_admin(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_TICKETS_ADMIN_LISTAR, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Abre o chamado e devolve a thread (visao admin), marcando como lido.
    pub async fn tickets_ver_admin(&self, query: &[(String, String)]) -> Result<Value> {
        let resp = self
            .send("GET", ROTA_TICKETS_ADMIN_VER, query, None, false)
            .await?;
        Ok(resp.body)
    }

    /// Muda o status de um chamado (admin).
    ///
    /// Corpo normalizado por [`normalizar_ticket_status`]. Resposta:
    /// `{"ok":true}`.
    pub async fn tickets_status(&self, body: Value) -> Result<Value> {
        let corpo = normalizar_ticket_status(body)?;
        let resp = self
            .send("POST", ROTA_TICKETS_ADMIN_STATUS, &[], Some(&corpo), false)
            .await?;
        Ok(resp.body)
    }
}

/// Converte um objeto JSON normalizado em pares de formulario.
fn pares_do_objeto(value: &Value) -> Vec<(String, String)> {
    let mut pares = Vec::new();
    if let Some(obj) = value.as_object() {
        for (chave, valor) in obj {
            let texto = match valor {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => String::new(),
                outro => outro.to_string(),
            };
            pares.push((chave.clone(), texto));
        }
    }
    pares
}

fn truncar(texto: &str, max: usize) -> String {
    texto.chars().take(max).collect()
}

fn exigir_objeto<'a>(body: &'a Value, contexto: &str) -> Result<&'a Map<String, Value>> {
    body.as_object().ok_or_else(|| {
        ApiError::Config(format!(
            "{contexto}: o corpo precisa ser um objeto JSON com os campos da rota"
        ))
    })
}

fn inteiro_positivo(obj: &Map<String, Value>, campo: &str) -> Result<i64> {
    let valor = obj
        .get(campo)
        .ok_or_else(|| ApiError::Config(format!("campo {campo} e obrigatorio")))?;
    let n = valor
        .as_i64()
        .ok_or_else(|| ApiError::Config(format!("campo {campo} deve ser um inteiro")))?;
    if n <= 0 {
        return Err(ApiError::Config(format!(
            "campo {campo} deve ser maior que zero"
        )));
    }
    Ok(n)
}

fn texto_obrigatorio(obj: &Map<String, Value>, campo: &str) -> Result<String> {
    let valor = obj
        .get(campo)
        .and_then(Value::as_str)
        .ok_or_else(|| ApiError::Config(format!("campo {campo} e obrigatorio (string)")))?;
    let texto = valor.trim();
    if texto.is_empty() {
        return Err(ApiError::Config(format!(
            "campo {campo} nao pode ficar vazio"
        )));
    }
    Ok(texto.to_string())
}

fn exigir_enum(obj: &Map<String, Value>, campo: &str, permitidos: &[&str]) -> Result<String> {
    let valor = obj
        .get(campo)
        .and_then(Value::as_str)
        .ok_or_else(|| ApiError::Config(format!("campo {campo} e obrigatorio (string)")))?;
    let valor = valor.trim().to_ascii_lowercase();
    if permitidos.contains(&valor.as_str()) {
        Ok(valor)
    } else {
        Err(ApiError::Config(format!(
            "campo {campo}: valor {valor:?} invalido (use {})",
            permitidos.join(", ")
        )))
    }
}
