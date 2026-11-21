//! Revendas / sub-revendas (arvore da franquia).
//!
//! Rotas (fonte: `routes_revendas.go`, `merger_api_revendas.go`,
//! `cliente_transfer_revenda.go`):
//! - `GET    /merger/api/cliente/revendas`            (listar)
//! - `POST   /merger/api/cliente/revendas`            (CRIAR)  ← mesma rota, metodo POST
//! - `PUT    /merger/api/cliente/revendas`            (editar acesso/senha/permissoes/creditos)
//! - `PATCH  /merger/api/cliente/revendas`            (ligar/desligar em cascata)
//! - `DELETE /merger/api/cliente/revendas?id=N`       (remover em cascata)
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

use chrono::{Duration as ChronoDuration, Local};
use serde_json::{json, Map, Value};

use crate::client::MergerClient;
use crate::error::{ApiError, Result};

/// Rota da colecao de revendas: lista (GET), cria (POST), edita (PUT),
/// liga/desliga (PATCH) e remove (DELETE).
pub const ROTA_REVENDAS: &str = "/merger/api/cliente/revendas";
/// Link de auto-cadastro da revenda (GET gera lazy; POST regenera).
pub const ROTA_CONVITE: &str = "/merger/api/cliente/convite";
pub const ROTA_CONVITE_REGENERAR: &str = "/merger/api/cliente/convite/regenerar";
/// Adesoes manuais de mensalistas feitas pelo convite.
pub const ROTA_CONVITE_ADESAO_PENDENTES: &str = "/merger/api/cliente/convite/adesao/pendentes";
pub const ROTA_CONVITE_ADESAO_CONFIRMAR: &str = "/merger/api/cliente/convite/adesao/confirmar";
/// Oferta de mensalista no convite (GET/POST).
pub const ROTA_CONVITE_MENSALISTA_CONFIG: &str = "/merger/api/cliente/convite/mensalista-config";

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

    /// Cria uma sub-revenda (revenda filha da franquia autenticada).
    ///
    /// `body` aceita:
    /// - `username` (obrigatorio)
    /// - `password` (obrigatorio, minimo 4 caracteres)
    /// - `credits` (inteiro; **minimo 10** quando `unlimited = 0`)
    /// - `unlimited` (`0`/`1`; ilimitada ignora `credits`)
    /// - `pode_criar_revenda` (`0`/`1`, default `1`)
    /// - `pode_criar_ilimitado` (`0`/`1`, default `0`; so vale se ilimitada)
    /// - `pode_conectar_whatsapp` (`0`/`1`, default `0`)
    ///
    /// O painel debita os creditos iniciais do criador quando ele nao e
    /// ilimitado e grava o evento `revenda.criada` na auditoria.
    pub async fn revendas_criar(&self, body: Value) -> Result<Value> {
        let body = build_revenda_nova_body(&body)?;
        self.send("POST", ROTA_REVENDAS, &[], Some(&body), false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Cria a sub-revenda e aplica o vencimento inicial do painel.
    ///
    /// O `POST /revendas` do painel **nao** aceita vencimento; ele e gravado
    /// pela rota `revendas/nota` logo depois. `dias_painel = 0` pula a etapa
    /// (mesmo comportamento da UI). Se a criacao funcionar e o vencimento
    /// falhar, a resposta volta com `vencimento_aplicado: false` e `aviso`
    /// (a revenda **existe**).
    pub async fn revendas_criar_com_vencimento(
        &self,
        body: Value,
        dias_painel: u32,
    ) -> Result<Value> {
        let username = body
            .get("username")
            .and_then(Value::as_str)
            .map(str::trim)
            .unwrap_or_default()
            .to_string();

        let criada = self.revendas_criar(body).await?;
        let ok = criada.get("ok").and_then(Value::as_bool).unwrap_or(false);
        if !ok {
            return Ok(criada);
        }
        let username = criada
            .get("username")
            .and_then(Value::as_str)
            .map(str::to_string)
            .or_else(|| {
                criada
                    .get("usuario_acesso")
                    .and_then(Value::as_str)
                    .map(str::to_string)
            })
            .unwrap_or(username);
        if dias_painel == 0 {
            return Ok(criada);
        }

        let vencimento = vencimento_em_dias(dias_painel);
        let nota = self
            .revendas_nota(json!({ "username": username, "vencimento": vencimento }))
            .await;

        match nota {
            Ok(resp) if resp.get("ok").and_then(Value::as_bool).unwrap_or(false) => {
                let mut saida = criada;
                if let Some(obj) = saida.as_object_mut() {
                    obj.insert("vencimento_painel".to_string(), json!(vencimento));
                    obj.insert("vencimento_aplicado".to_string(), json!(true));
                }
                Ok(saida)
            }
            Ok(resp) => Ok(com_aviso(
                criada,
                vencimento,
                format!("revenda criada, mas o vencimento nao foi aplicado: {resp}"),
            )),
            Err(err) => Ok(com_aviso(
                criada,
                vencimento,
                format!("revenda criada, mas o vencimento nao foi aplicado: {err}"),
            )),
        }
    }

    /// Edita uma sub-revenda existente (`PUT /merger/api/cliente/revendas`).
    ///
    /// Aceita um grupo por chamada, igual ao painel:
    /// - acesso: `{id, usuario_acesso, password?}`
    /// - senha: `{id, password}`
    /// - permissoes: `{id, set_pode_criar_revenda?, set_pode_criar_ilimitado?, set_pode_conectar_whatsapp?}`
    /// - creditos: `{id, transfer}` (minimo 10; debita do pai se ele nao for ilimitado)
    pub async fn revendas_editar(&self, body: Value) -> Result<Value> {
        let body = build_revenda_editar_body(&body)?;
        self.send("PUT", ROTA_REVENDAS, &[], Some(&body), false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Liga/desliga a revenda e toda a subarvore (`PATCH .../revendas`).
    pub async fn revendas_toggle(&self, id: i64, enabled: bool) -> Result<Value> {
        if id <= 0 {
            return Err(ApiError::Config(
                "informe o id da revenda (inteiro maior que zero)".into(),
            ));
        }
        let body = json!({ "id": id, "enabled": if enabled { 1 } else { 0 } });
        self.send("PATCH", ROTA_REVENDAS, &[], Some(&body), false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Remove a revenda e toda a subarvore (`DELETE .../revendas?id=N`).
    ///
    /// **Destrutivo**: apaga as linhas Xtream da subarvore. Exige
    /// `confirm = true`; a trava de exclusao do painel ainda pode recusar.
    pub async fn revendas_remover(&self, id: i64, confirm: bool) -> Result<Value> {
        if id <= 0 {
            return Err(ApiError::Config(
                "informe o id da revenda (inteiro maior que zero)".into(),
            ));
        }
        let query = vec![("id".to_string(), id.to_string())];
        self.send("DELETE", ROTA_REVENDAS, &query, None, confirm)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Le o link de auto-cadastro (gera o codigo na primeira chamada).
    pub async fn convite(&self) -> Result<Value> {
        self.send("GET", ROTA_CONVITE, &[], None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Regenera o codigo do convite; o link antigo morre na hora.
    pub async fn convite_regenerar(&self) -> Result<Value> {
        self.send("POST", ROTA_CONVITE_REGENERAR, &[], None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Lista adesoes manuais pendentes de mensalistas vindas do convite.
    pub async fn convite_adesao_pendentes(&self) -> Result<Value> {
        self.send("GET", ROTA_CONVITE_ADESAO_PENDENTES, &[], None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Confirma o recebimento de uma adesao manual (`{pedido_id}`).
    pub async fn convite_adesao_confirmar(&self, pedido_id: &str) -> Result<Value> {
        let pedido_id = pedido_id.trim();
        if pedido_id.is_empty() {
            return Err(ApiError::Config(
                "informe o pedido_id da adesao a confirmar".into(),
            ));
        }
        let body = json!({ "pedido_id": pedido_id });
        self.send(
            "POST",
            ROTA_CONVITE_ADESAO_CONFIRMAR,
            &[],
            Some(&body),
            false,
        )
        .await
        .map(|resp| resp.json_or_text())
    }

    /// Le a configuracao da oferta de mensalista no convite.
    pub async fn convite_mensalista_config(&self) -> Result<Value> {
        self.send("GET", ROTA_CONVITE_MENSALISTA_CONFIG, &[], None, false)
            .await
            .map(|resp| resp.json_or_text())
    }

    /// Salva a configuracao da oferta de mensalista no convite.
    ///
    /// Corpo: `{"habilitado": bool, "curva": {"tarifas_centavos": [..]}}`.
    pub async fn convite_mensalista_config_salvar(&self, body: Value) -> Result<Value> {
        if !body.is_object() {
            return Err(ApiError::Config(
                "corpo da configuracao precisa ser um objeto JSON".into(),
            ));
        }
        self.send(
            "POST",
            ROTA_CONVITE_MENSALISTA_CONFIG,
            &[],
            Some(&body),
            false,
        )
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
    ///
    /// **Rebaixamento sem `saldo_rebaixamento`:** o painel aplicaria `0` e
    /// zeraria os creditos da revenda silenciosamente. Aqui o saldo atual da
    /// revenda e lido e reaplicado, a menos que o chamador informe um valor
    /// explicito (inclusive `0`, que passa a ser perda intencional).
    pub async fn revenda_unlimited(&self, body: Value) -> Result<Value> {
        let mut body = build_revenda_unlimited_body(&body)?;
        let rebaixando = body.get("novo_estado").and_then(Value::as_i64) == Some(0);
        if rebaixando && body.get("saldo_rebaixamento").is_none() {
            let id = body.get("id").and_then(Value::as_i64).unwrap_or(0);
            let saldo = match self.saldo_atual_revenda(id).await {
                Ok(saldo) => saldo,
                // Sem sessao: deixa o chamador (servidor/CLI) fazer o login.
                Err(err @ (ApiError::SessionExpired | ApiError::NotAuthenticated)) => {
                    return Err(err)
                }
                Err(_) => {
                    return Err(ApiError::Config(
                        "informe saldo_rebaixamento: nao consegui ler o saldo atual da revenda"
                            .into(),
                    ))
                }
            };
            body["saldo_rebaixamento"] = json!(saldo);
        }
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

    /// Le o saldo atual (`credits`) de uma revenda na listagem da franquia.
    async fn saldo_atual_revenda(&self, id: i64) -> Result<i64> {
        let lista = self.revendas_list(&[]).await?;
        if let Some(itens) = lista.as_array() {
            for item in itens {
                if item.get("id").and_then(Value::as_i64) == Some(id) {
                    return item
                        .get("credits")
                        .and_then(Value::as_i64)
                        .ok_or_else(|| ApiError::Config("revenda sem saldo legivel".into()));
                }
            }
        }
        Err(ApiError::Config(
            "revenda nao encontrada na listagem".into(),
        ))
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

/// Corpo de `POST /merger/api/cliente/revendas` (criacao).
///
/// Espelha as validacoes do handler Go: senha com no minimo 4 caracteres,
/// minimo de 10 creditos quando a revenda nao e ilimitada e
/// `pode_criar_ilimitado` forcado a 0 em revenda finita.
pub fn build_revenda_nova_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let username = texto_obrigatorio(obj, "username")?;
    let password = obj
        .get("password")
        .or_else(|| obj.get("senha"))
        .and_then(Value::as_str)
        .map(str::trim)
        .unwrap_or_default();
    if password.chars().count() < 4 {
        return Err(ApiError::Config(
            "campo password obrigatorio (minimo 4 caracteres)".into(),
        ));
    }

    let unlimited = flag_cero_um(obj, "unlimited")?.unwrap_or(0);
    let credits = inteiro_opcional(obj, "credits")?.unwrap_or(0);
    if unlimited == 0 && credits < 10 {
        return Err(ApiError::Config(
            "credits: minimo de 10 creditos para criar revenda limitada".into(),
        ));
    }
    let pode_revenda = flag_cero_um(obj, "pode_criar_revenda")?.unwrap_or(1);
    let mut pode_ilimitado = flag_cero_um(obj, "pode_criar_ilimitado")?.unwrap_or(0);
    if unlimited != 1 {
        pode_ilimitado = 0;
    }
    let pode_whatsapp = flag_cero_um(obj, "pode_conectar_whatsapp")?.unwrap_or(0);
    let creditos_iniciais = if unlimited == 1 { 0 } else { credits };

    Ok(json!({
        "username": username,
        "password": password,
        "credits": creditos_iniciais,
        "unlimited": unlimited,
        "pode_criar_revenda": pode_revenda,
        "pode_criar_ilimitado": pode_ilimitado,
        "pode_conectar_whatsapp": pode_whatsapp,
    }))
}

/// Corpo de `PUT /merger/api/cliente/revendas` (um grupo por chamada).
pub fn build_revenda_editar_body(body: &Value) -> Result<Value> {
    let obj = objeto(body)?;
    let id = inteiro_positivo(obj, "id")?;

    let acesso = texto_opcional(obj, "usuario_acesso")?;
    let senha = texto_opcional(obj, "password")?;
    let transfer = if obj.contains_key("transfer") {
        inteiro_opcional(obj, "transfer")?
    } else {
        None
    };
    let mut flags = Map::new();
    for campo in [
        "set_pode_criar_revenda",
        "set_pode_criar_ilimitado",
        "set_pode_conectar_whatsapp",
    ] {
        if let Some(valor) = flag_cero_um(obj, campo)? {
            flags.insert(campo.to_string(), json!(valor));
        }
    }

    let grupos = [
        acesso.is_some() || senha.is_some(),
        transfer.is_some(),
        !flags.is_empty(),
    ]
    .iter()
    .filter(|presente| **presente)
    .count();
    if grupos == 0 {
        return Err(ApiError::Config(
            "informe um grupo: usuario_acesso (+password), password, transfer ou set_pode_*".into(),
        ));
    }
    if grupos > 1 {
        return Err(ApiError::Config(
            "edite um grupo por chamada (acesso, senha, creditos ou permissoes)".into(),
        ));
    }
    if acesso.is_some() && senha.is_none() && obj.contains_key("password") {
        return Err(ApiError::Config("campo password deve ser string".into()));
    }
    if let Some(senha) = senha.as_deref() {
        if senha.chars().count() < 4 {
            return Err(ApiError::Config(
                "campo password deve ter no minimo 4 caracteres".into(),
            ));
        }
    }
    if let Some(valor) = transfer {
        if valor < 10 {
            return Err(ApiError::Config(
                "transfer: transferencia minima de 10 creditos".into(),
            ));
        }
    }

    let mut saida = Map::new();
    saida.insert("id".to_string(), json!(id));
    if let Some(valor) = acesso {
        saida.insert("usuario_acesso".to_string(), json!(valor));
    }
    if let Some(valor) = senha {
        saida.insert("password".to_string(), json!(valor));
    }
    if let Some(valor) = transfer {
        saida.insert("transfer".to_string(), json!(valor));
    }
    for (campo, valor) in flags {
        saida.insert(campo, valor);
    }

    Ok(Value::Object(saida))
}

/// Data `YYYY-MM-DD` daqui a `dias` (fuso local do processo).
pub fn vencimento_em_dias(dias: u32) -> String {
    (Local::now() + ChronoDuration::days(i64::from(dias)))
        .format("%Y-%m-%d")
        .to_string()
}

fn com_aviso(mut criada: Value, vencimento: String, aviso: String) -> Value {
    if let Some(obj) = criada.as_object_mut() {
        obj.insert("vencimento_painel".to_string(), json!(vencimento));
        obj.insert("vencimento_aplicado".to_string(), json!(false));
        obj.insert("aviso".to_string(), json!(aviso));
    }
    criada
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
    let saldo = inteiro_opcional(obj, "saldo_rebaixamento")?;
    if let Some(valor) = saldo {
        if valor < 0 {
            return Err(ApiError::Config(
                "saldo_rebaixamento nao pode ser negativo".into(),
            ));
        }
    }

    let mut saida = json!({ "id": id, "novo_estado": novo_estado });
    match saldo {
        // Rebaixar sem saldo explicito: o metodo busca o saldo atual antes de
        // enviar (evita zerar a revenda por omissao).
        None if novo_estado == 1 => {
            saida["saldo_rebaixamento"] = json!(0);
        }
        None => {}
        Some(valor) => {
            saida["saldo_rebaixamento"] = json!(valor);
        }
    }
    Ok(saida)
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

/// Aceita `0`/`1` como numero, booleano ou string (`true`/`false`).
fn flag_cero_um(obj: &Map<String, Value>, campo: &str) -> Result<Option<i64>> {
    match obj.get(campo) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Bool(valor)) => Ok(Some(if *valor { 1 } else { 0 })),
        Some(Value::Number(numero)) => match numero.as_i64() {
            Some(0) => Ok(Some(0)),
            Some(1) => Ok(Some(1)),
            _ => Err(ApiError::Config(format!("campo {campo} deve ser 0 ou 1"))),
        },
        Some(Value::String(texto)) => match texto.trim().to_ascii_lowercase().as_str() {
            "" => Ok(None),
            "0" | "false" => Ok(Some(0)),
            "1" | "true" => Ok(Some(1)),
            _ => Err(ApiError::Config(format!("campo {campo} deve ser 0 ou 1"))),
        },
        Some(_) => Err(ApiError::Config(format!("campo {campo} deve ser 0 ou 1"))),
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
