//! Apps Ibo (msproapp.top) — ativacao por MAC, contas e atribuicao.
//!
//! Rotas (fonte: `routes_msproapp.go`, `msproapp_clientes.go`,
//! `msproapp_contas.go`, `msproapp_posse_api.go`):
//! - `GET    /merger/api/cliente/msproapp_contas`
//! - `GET    /merger/api/cliente/msproapp/aplicativos?conta=`
//! - `GET    /merger/api/cliente/msproapp/clientes?conta=`
//! - `POST   /merger/api/cliente/msproapp/clientes?conta=&geracao=`
//! - `DELETE /merger/api/cliente/msproapp/cliente?id=&conta=&geracao=`
//! - `POST   /merger/api/cliente/msproapp/clientes/atribuir?conta=&geracao=`
//!
//! `geracao` e o token de concorrencia da conta: este cliente le a geracao em
//! `msproapp_contas`, manda em toda mutacao e, se o painel responder
//! `geracao_alterada` (HTTP 409), refaz a leitura e o request **uma vez**.
//! O chamador nunca precisa gerenciar isso.
//!
//! Anti-leak: a listagem de contas sai sanitizada (sem `cofre_uuid`, e-mail ou
//! credenciais) e a listagem de clientes remove `cofre_uuid`/`geracao`. O
//! `m3u8_list` (contem usuario/senha do cliente final) so viaja no corpo do
//! POST de ativacao; nunca e devolvido nem logado.

use std::collections::HashSet;

use chrono::NaiveDate;
use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};
use crate::raw::RawResponse;

/// Contas visiveis + apps + clientes.
pub const ROTA_CONTAS: &str = "/merger/api/cliente/msproapp_contas";
pub const ROTA_APPS: &str = "/merger/api/cliente/msproapp/aplicativos";
pub const ROTA_CLIENTES: &str = "/merger/api/cliente/msproapp/clientes";
pub const ROTA_CLIENTE: &str = "/merger/api/cliente/msproapp/cliente";
pub const ROTA_ATRIBUIR: &str = "/merger/api/cliente/msproapp/clientes/atribuir";

/// Dados internos da conta (geracao/pode_atribuir nao vao para a API publica).
#[derive(Debug, Clone)]
pub struct ContaIbo {
    pub id: i64,
    pub apelido: String,
    pub painel: String,
    pub conectada: bool,
    pub geracao: i64,
    pub pode_atribuir: bool,
}

impl MergerClient {
    /// Lista as contas Apps Ibo visiveis, sanitizadas.
    ///
    /// Cada item tem apenas `id`, `apelido`, `conectada` e `painel`.
    /// `cofre_uuid`, e-mail e credenciais nunca saem daqui.
    pub async fn apps_ibo_contas(&self) -> Result<Value> {
        let raw = self.apps_ibo_contas_raw().await?;
        let mut vistos: HashSet<i64> = HashSet::new();
        let mut contas: Vec<Value> = Vec::new();
        for lista in ["minhas", "disponiveis"] {
            let Some(itens) = raw.get(lista).and_then(Value::as_array) else {
                continue;
            };
            for conta in itens {
                let id = conta.get("id").and_then(Value::as_i64).unwrap_or(0);
                if id <= 0 || !vistos.insert(id) {
                    continue;
                }
                contas.push(json!({
                    "id": id,
                    "apelido": conta.get("apelido").and_then(Value::as_str).unwrap_or(""),
                    "conectada": conta.get("conectada").and_then(Value::as_bool).unwrap_or(false),
                    "painel": conta.get("painel").and_then(Value::as_str).unwrap_or(""),
                }));
            }
        }
        Ok(json!({ "ok": true, "contas": contas }))
    }

    /// Aplicativos da conta (lista viva do painel externo).
    pub async fn apps_ibo_apps(&self, conta: i64) -> Result<Value> {
        let conta = validar_conta(conta)?;
        let query = vec![("conta".to_string(), conta.to_string())];
        let resp = self.send("GET", ROTA_APPS, &query, None, false).await?;
        Ok(resp.body)
    }

    /// Lista os clientes da conta (resumo sem `m3u8_list`/credenciais).
    ///
    /// Remove `cofre_uuid` e `geracao` da resposta. `donos` (revendas da
    /// arvore) permanece para permitir atribuicao.
    pub async fn apps_ibo_clientes(&self, conta: i64) -> Result<Value> {
        let mut body = self.apps_ibo_clientes_raw(conta).await?;
        if let Some(obj) = body.as_object_mut() {
            obj.remove("cofre_uuid");
            obj.remove("geracao");
        }
        Ok(body)
    }

    /// Ativa um MAC na conta.
    ///
    /// `body` aceita `{conta, app_id, mac, modo, server_name, price,
    /// expire_date, m3u8_list?, xteam_username?, xteam_password?, dns?,
    /// username_login?, password_login?, whatsapp?, url_epg?, plan_id?,
    /// account_username?, account_password?, is_trial?}`.
    pub async fn apps_ibo_ativar(&self, body: Value) -> Result<Value> {
        let conta = conta_do_corpo(&body)?;
        let payload = build_apps_ibo_ativar_body(&body)?;
        self.apps_ibo_mutar(conta, "POST", ROTA_CLIENTES, &[], false, Some(&payload))
            .await
    }

    /// Remove um cliente da conta. Exige `confirm = true`.
    pub async fn apps_ibo_excluir(&self, id: &str, conta: i64, confirm: bool) -> Result<Value> {
        let id = id.trim();
        if id.is_empty() {
            return Err(ApiError::Config("id do cliente obrigatorio".into()));
        }
        if !confirm {
            return Err(ApiError::ConfirmationRequired(format!(
                "excluir cliente Apps Ibo {id}"
            )));
        }
        let conta = validar_conta(conta)?;
        let extra = vec![("id".to_string(), id.to_string())];
        self.apps_ibo_mutar(conta, "DELETE", ROTA_CLIENTE, &extra, true, None)
            .await
    }

    /// Atribui um cliente a uma revenda da arvore (`dono` = username).
    ///
    /// O painel so aceita a franquia dona; a revenda recebe 403.
    pub async fn apps_ibo_atribuir(&self, conta: i64, id: &str, dono: &str) -> Result<Value> {
        let conta = validar_conta(conta)?;
        let id = id.trim();
        let dono = dono.trim();
        if id.is_empty() {
            return Err(ApiError::Config("id do cliente obrigatorio".into()));
        }
        if dono.is_empty() {
            return Err(ApiError::Config(
                "dono obrigatorio (username da revenda)".into(),
            ));
        }

        let listagem = self.apps_ibo_clientes_raw(conta).await?;
        let dono_id = listagem
            .get("donos")
            .and_then(Value::as_array)
            .and_then(|donos| {
                donos.iter().find(|item| {
                    ["username", "usuario_acesso"]
                        .iter()
                        .any(|campo| item.get(*campo).and_then(Value::as_str) == Some(dono))
                })
            })
            .and_then(|item| item.get("id").and_then(Value::as_i64))
            .filter(|id| *id > 0)
            .ok_or_else(|| {
                ApiError::Config(format!(
                    "revenda {dono:?} nao encontrada na arvore desta conta"
                ))
            })?;

        let body = json!({
            "ids": [id],
            "dono_id": dono_id,
            "todos_sem_dono": false,
        });
        self.apps_ibo_mutar(conta, "POST", ROTA_ATRIBUIR, &[], false, Some(&body))
            .await
    }

    /// Resposta crua de `msproapp_contas` (uso interno).
    async fn apps_ibo_contas_raw(&self) -> Result<Value> {
        let resp = self.send("GET", ROTA_CONTAS, &[], None, false).await?;
        Ok(resp.body)
    }

    /// Resposta crua de `msproapp/clientes` (uso interno; tem `cofre_uuid`).
    async fn apps_ibo_clientes_raw(&self, conta: i64) -> Result<Value> {
        let conta = validar_conta(conta)?;
        let query = vec![("conta".to_string(), conta.to_string())];
        let resp = self.send("GET", ROTA_CLIENTES, &query, None, false).await?;
        Ok(resp.body)
    }

    /// Le a conta na listagem crua e devolve a `geracao` atual.
    async fn apps_ibo_conta(&self, conta: i64) -> Result<ContaIbo> {
        let conta = validar_conta(conta)?;
        let raw = self.apps_ibo_contas_raw().await?;
        for lista in ["minhas", "disponiveis"] {
            let Some(itens) = raw.get(lista).and_then(Value::as_array) else {
                continue;
            };
            for item in itens {
                if item.get("id").and_then(Value::as_i64) != Some(conta) {
                    continue;
                }
                let geracao = item.get("geracao").and_then(Value::as_i64).unwrap_or(0);
                if geracao <= 0 {
                    return Err(ApiError::Config(format!(
                        "conta {conta} sem geracao valida; abra a conta no painel e tente de novo"
                    )));
                }
                return Ok(ContaIbo {
                    id: conta,
                    apelido: item
                        .get("apelido")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string(),
                    painel: item
                        .get("painel")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string(),
                    conectada: item
                        .get("conectada")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                    geracao,
                    pode_atribuir: item
                        .get("pode_atribuir")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                });
            }
        }
        Err(ApiError::NotFound)
    }

    /// Executa uma mutacao com `conta`+`geracao`, repetindo uma vez se a
    /// geracao mudar entre a leitura e o request.
    async fn apps_ibo_mutar(
        &self,
        conta: i64,
        metodo: &str,
        rota: &str,
        extra: &[(String, String)],
        confirm: bool,
        body: Option<&Value>,
    ) -> Result<Value> {
        let conta = validar_conta(conta)?;
        let info = self.apps_ibo_conta(conta).await?;
        let mut resp = self
            .send(
                metodo,
                rota,
                &query_geracao(conta, info.geracao, extra),
                body,
                confirm,
            )
            .await?;

        if resp.status == 409 && resposta_geracao_alterada(&resp) {
            let info = self.apps_ibo_conta(conta).await?;
            resp = self
                .send(
                    metodo,
                    rota,
                    &query_geracao(conta, info.geracao, extra),
                    body,
                    confirm,
                )
                .await?;
        }

        finalizar_mutacao(resp)
    }
}

/// Corpo exato de `POST /merger/api/cliente/msproapp/clientes`.
///
/// Espelha `msproappMontarPayload` (Go): valida modo -> MAC -> servidor -> app
/// -> preco -> validade -> campos do modo e envia as 19 chaves, inclusive as
/// que o modo nao usa (string vazia / null), porque o Laravel valida presenca.
pub fn build_apps_ibo_ativar_body(body: &Value) -> Result<Value> {
    let obj = body
        .as_object()
        .ok_or_else(|| ApiError::Config("corpo da requisicao precisa ser um objeto JSON".into()))?;

    let modo_bruto = texto(obj, "modo")
        .or_else(|| texto(obj, "modo_selecao"))
        .unwrap_or_else(|| "1".to_string());
    let modo = match modo_bruto.trim().to_ascii_lowercase().as_str() {
        "0" | "xteam" | "xteam_code" | "xteamcode" => "0",
        "1" | "m3u8" | "m3u" => "1",
        outro => {
            return Err(ApiError::Config(format!(
                "modo invalido {outro:?}: use 1 (M3U8) ou 0 (XTeam)"
            )))
        }
    };

    let mac_bruto = texto(obj, "mac")
        .or_else(|| texto(obj, "mac_device"))
        .ok_or_else(|| ApiError::Config("campo mac obrigatorio".into()))?;
    let mac = normalizar_mac(&mac_bruto)?;

    let server_name = texto(obj, "server_name")
        .ok_or_else(|| ApiError::Config("campo server_name obrigatorio".into()))?;

    let app = texto(obj, "app_id")
        .or_else(|| texto(obj, "ranking_app_id"))
        .ok_or_else(|| ApiError::Config("campo app_id obrigatorio".into()))?;

    let price_bruto = texto(obj, "price").unwrap_or_else(|| "0".to_string());
    let price = normalizar_preco(&price_bruto)?;

    let expire_date = texto(obj, "expire_date")
        .ok_or_else(|| ApiError::Config("campo expire_date obrigatorio".into()))?;
    validar_data(&expire_date)?;

    let mut m3u8_list = String::new();
    let mut xteam_username = String::new();
    let mut xteam_password = String::new();
    let mut dns = String::new();

    if modo == "1" {
        m3u8_list = texto(obj, "m3u8_list").ok_or_else(|| {
            ApiError::Config("m3u8_list obrigatoria no modo M3U8 (modo=1)".into())
        })?;
    } else {
        xteam_username = texto(obj, "xteam_username").ok_or_else(|| {
            ApiError::Config("xteam_username obrigatorio no modo XTeam (modo=0)".into())
        })?;
        xteam_password = texto(obj, "xteam_password").ok_or_else(|| {
            ApiError::Config("xteam_password obrigatorio no modo XTeam (modo=0)".into())
        })?;
        dns = texto(obj, "dns").unwrap_or_default();
    }

    let (username_login, password_login) = if app == "11" {
        (
            json!(texto(obj, "username_login").unwrap_or_default()),
            json!(texto(obj, "password_login").unwrap_or_default()),
        )
    } else {
        (Value::Null, Value::Null)
    };

    // A entrada do painel (`MsproappEntrada.IsTrial`) e **bool**; mandar 0/1
    // faz o decoder do Go recusar o corpo inteiro. A conversao para 0/1
    // acontece dentro do painel, no payload final.
    let is_trial = match obj.get("is_trial") {
        None | Some(Value::Null) => false,
        Some(Value::Bool(flag)) => *flag,
        Some(Value::Number(numero)) => numero.as_i64() == Some(1),
        Some(Value::String(texto)) => texto.trim() == "true" || texto.trim() == "1",
        _ => false,
    };

    Ok(json!({
        "modo_selecao": modo,
        "mac_device": mac,
        "server_name": server_name,
        "account_username": texto(obj, "account_username").unwrap_or_default(),
        "account_password": texto(obj, "account_password").unwrap_or_default(),
        "xteam_username": xteam_username,
        "xteam_password": xteam_password,
        "username_login": username_login,
        "password_login": password_login,
        "ranking_app_id": app,
        "dns": dns,
        "m3u8_list": m3u8_list,
        "url_epg": texto(obj, "url_epg").unwrap_or_default(),
        "price": price,
        "plan_id": texto(obj, "plan_id").unwrap_or_default(),
        "expire_date": expire_date,
        "whatsapp": texto(obj, "whatsapp").unwrap_or_default(),
        "is_trial": is_trial,
        "general_list_title": "",
        "playlists": Value::Null,
    }))
}

/// Normaliza o MAC como o painel: aceita `:`/`-`/`.`/espaco, exige 12
/// caracteres alfanumericos e devolve `AA:BB:CC:DD:EE:FF` em maiuscula.
pub fn normalizar_mac(bruto: &str) -> Result<String> {
    let limpo: String = bruto
        .trim()
        .chars()
        .filter(|caractere| !matches!(caractere, ':' | '-' | '.' | ' '))
        .collect();
    if limpo.chars().count() != 12 {
        return Err(ApiError::Config(format!(
            "MAC invalido {bruto:?}: precisa de 12 caracteres (letras ou numeros)"
        )));
    }
    if !limpo
        .chars()
        .all(|caractere| caractere.is_ascii_alphanumeric())
    {
        return Err(ApiError::Config(format!(
            "MAC invalido {bruto:?}: use apenas letras e numeros"
        )));
    }
    let maiusculo = limpo.to_ascii_uppercase();
    let pares: Vec<String> = maiusculo
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|par| par.iter().collect())
        .collect();
    Ok(pares.join(":"))
}

/// Normaliza preco no formato do painel (`30`, `30.5`, `1234.56`).
pub fn normalizar_preco(bruto: &str) -> Result<String> {
    let preco = bruto.trim();
    if preco.is_empty() {
        return Err(ApiError::Config("preco obrigatorio".into()));
    }

    let mut normalizado = preco.to_string();
    if normalizado.contains(',') {
        normalizado = normalizado.replace('.', "");
        normalizado = normalizado.replacen(',', ".", 1);
    } else if eh_separador_de_milhar(&normalizado) {
        normalizado = normalizado.replace('.', "");
    }

    if !preco_valido(&normalizado) {
        return Err(ApiError::Config(format!(
            "preco invalido {bruto:?}: use algo como 30 ou 30,50"
        )));
    }
    Ok(normalizado)
}

fn eh_separador_de_milhar(preco: &str) -> bool {
    let partes: Vec<&str> = preco.split('.').collect();
    if partes.len() < 2 {
        return false;
    }
    let inteiro = partes[0];
    (1..=3).contains(&inteiro.len())
        && inteiro.chars().all(|caractere| caractere.is_ascii_digit())
        && partes[1..]
            .iter()
            .all(|parte| parte.len() == 3 && parte.chars().all(|c| c.is_ascii_digit()))
}

fn preco_valido(preco: &str) -> bool {
    let mut partes = preco.split('.');
    let inteiro = partes.next().unwrap_or("");
    if inteiro.is_empty() || !inteiro.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    match partes.next() {
        None => true,
        Some(fracao) => {
            (1..=2).contains(&fracao.len())
                && fracao.chars().all(|c| c.is_ascii_digit())
                && partes.next().is_none()
        }
    }
}

fn validar_data(data: &str) -> Result<()> {
    let bytes = data.as_bytes();
    let formato = bytes.len() == 10 && bytes[4] == b'-' && bytes[7] == b'-';
    if !formato || NaiveDate::parse_from_str(data, "%Y-%m-%d").is_err() {
        return Err(ApiError::Config(format!(
            "expire_date invalida {data:?}: use o formato AAAA-MM-DD"
        )));
    }
    Ok(())
}

fn conta_do_corpo(body: &Value) -> Result<i64> {
    let conta = body
        .get("conta")
        .and_then(Value::as_i64)
        .filter(|conta| *conta > 0)
        .ok_or_else(|| ApiError::Config("campo conta obrigatorio (inteiro positivo)".into()))?;
    Ok(conta)
}

fn validar_conta(conta: i64) -> Result<i64> {
    if conta <= 0 {
        return Err(ApiError::Config(
            "conta obrigatoria (inteiro positivo)".into(),
        ));
    }
    Ok(conta)
}

fn texto(obj: &Map<String, Value>, campo: &str) -> Option<String> {
    obj.get(campo)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|valor| !valor.is_empty())
        .map(str::to_string)
}

fn query_geracao(conta: i64, geracao: i64, extra: &[(String, String)]) -> Vec<(String, String)> {
    let mut query = Vec::with_capacity(2 + extra.len());
    query.push(("conta".to_string(), conta.to_string()));
    query.push(("geracao".to_string(), geracao.to_string()));
    query.extend_from_slice(extra);
    query
}

fn resposta_geracao_alterada(resp: &RawResponse) -> bool {
    resp.body
        .get("erro")
        .and_then(Value::as_str)
        .is_some_and(|codigo| codigo == "geracao_alterada")
}

fn mensagem_painel(resp: &RawResponse) -> String {
    resp.body
        .get("msg")
        .and_then(Value::as_str)
        .or_else(|| resp.body.get("erro").and_then(Value::as_str))
        .map(str::trim)
        .filter(|mensagem| !mensagem.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| {
            let trecho: String = resp.text.chars().take(240).collect();
            format!("painel respondeu {}: {trecho}", resp.status)
        })
}

fn finalizar_mutacao(resp: RawResponse) -> Result<Value> {
    let ok = resp.body.get("ok").and_then(Value::as_bool);
    if resp.ok && ok != Some(false) {
        return Ok(resp.body);
    }
    if resp.status == 409 && resposta_geracao_alterada(&resp) {
        return Err(ApiError::StaleGeneration);
    }
    let mensagem = mensagem_painel(&resp);
    if resp.status == 400 || resp.status == 409 {
        return Err(ApiError::Config(mensagem));
    }
    Err(ApiError::Panel {
        status: resp.status,
        body: mensagem,
    })
}
