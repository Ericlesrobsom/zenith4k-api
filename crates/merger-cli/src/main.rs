//! CLI `zenith4k`: operacao manual e diagnostico contra o painel.

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use merger_core::raw::{RawRequest, RawUpload, UploadFile};
use merger_core::{ClientConfig, MergerClient};
use serde_json::Value;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(name = "zenith4k", version, about = "CLI do painel goiptv merger")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Clone, Default, Args)]
struct ListArgs {
    /// Par de query no formato chave=valor (repetivel).
    #[arg(short = 'q', long = "query", value_name = "chave=valor")]
    query: Vec<String>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Valida a sessao no painel.
    Whoami,
    /// Faz uma chamada GET crua.
    Get {
        /// Caminho absoluto, ex.: /merger/api/cliente/revendas
        path: String,
        #[command(flatten)]
        list: ListArgs,
    },
    /// Faz uma chamada POST crua.
    Post {
        path: String,
        /// Corpo JSON inline.
        #[arg(long, conflicts_with = "file")]
        json: Option<String>,
        /// Arquivo com o corpo JSON.
        #[arg(long, value_name = "caminho", conflicts_with = "json")]
        file: Option<PathBuf>,
        /// Confirma operacao destrutiva.
        #[arg(long)]
        confirm: bool,
    },
    /// Envia multipart/form-data para o painel (banner, logo, anexo, import).
    Upload {
        /// Rota do painel, ex.: /merger/api/aparencia/imagem
        path: String,
        /// Campo texto no formato chave=valor (repetivel).
        #[arg(short = 'F', long = "field", value_name = "chave=valor")]
        field: Vec<String>,
        /// Arquivo no formato campo=caminho (repetivel).
        #[arg(short = 'f', long = "file", value_name = "campo=caminho")]
        file: Vec<String>,
        /// Confirma operacao destrutiva.
        #[arg(long)]
        confirm: bool,
    },
    /// Passthrough completo: recebe um RawRequest em JSON.
    Call {
        /// RawRequest inline: {"method","path","query","body","confirm"};
        /// query aceita objeto {"chave":"valor"} ou lista de pares.
        #[arg(long, conflicts_with = "file")]
        json: Option<String>,
        /// Arquivo com o RawRequest.
        #[arg(long, value_name = "caminho", conflicts_with = "json")]
        file: Option<PathBuf>,
        /// Forca confirm=true.
        #[arg(long)]
        confirm: bool,
    },
    /// Login no painel (informe --codigo quando houver 2FA).
    Login {
        /// Codigo 2FA recebido por e-mail.
        #[arg(long)]
        codigo: Option<String>,
    },
    /// Encerra a sessao no painel.
    Logout,
    /// Lista as revendas da franquia.
    Revendas {
        #[command(flatten)]
        list: ListArgs,
    },
    /// Cria uma sub-revenda (POST /merger/api/cliente/revendas).
    RevendasCriar {
        /// Corpo JSON (username, password, credits, unlimited, pode_criar_*).
        #[arg(long, conflicts_with = "file")]
        json: Option<String>,
        /// Arquivo com o corpo JSON.
        #[arg(long, value_name = "caminho", conflicts_with = "json")]
        file: Option<PathBuf>,
        /// Dias de vencimento inicial do painel (0 = nao mexer).
        #[arg(long, default_value_t = 30)]
        dias: u32,
    },
    /// Edita uma sub-revenda (PUT /merger/api/cliente/revendas).
    RevendasEditar {
        #[arg(long, conflicts_with = "file")]
        json: Option<String>,
        #[arg(long, value_name = "caminho", conflicts_with = "json")]
        file: Option<PathBuf>,
    },
    /// Liga/desliga a revenda e a subarvore.
    RevendasToggle {
        /// ID da revenda.
        id: i64,
        /// Habilita a revenda.
        #[arg(long)]
        enable: bool,
        /// Desabilita a revenda.
        #[arg(long)]
        disable: bool,
    },
    /// Remove a revenda e a subarvore (destrutivo).
    RevendasRemover {
        /// ID da revenda.
        id: i64,
        /// Confirma a exclusao em cascata.
        #[arg(long)]
        confirm: bool,
    },
    /// Renova o painel da revenda por +30 dias (sem cobranca).
    RevendasRenovar {
        /// ID da revenda.
        id: i64,
        /// Vencimento atual exibido na lista (YYYY-MM-DD). Omita para pular
        /// a conferencia.
        #[arg(long)]
        vencimento_atual: Option<String>,
    },
    /// Grava nota/contato/vencimento na revenda.
    RevendasNota {
        #[arg(long, conflicts_with = "file")]
        json: Option<String>,
        #[arg(long, value_name = "caminho", conflicts_with = "json")]
        file: Option<PathBuf>,
    },
    /// Liga/desliga o modo ilimitado da revenda.
    RevendasUnlimited {
        #[arg(long, conflicts_with = "file")]
        json: Option<String>,
        #[arg(long, value_name = "caminho", conflicts_with = "json")]
        file: Option<PathBuf>,
    },
    /// Transfere a revenda para outro pai.
    RevendasTransferir {
        #[arg(long, conflicts_with = "file")]
        json: Option<String>,
        #[arg(long, value_name = "caminho", conflicts_with = "json")]
        file: Option<PathBuf>,
    },
    /// Lista os pais possiveis para transferir a revenda.
    RevendasTransferirDestinos {
        /// Username da revenda a mover.
        origem: String,
    },
    /// Mostra o link de auto-cadastro (ou regenera com --regenerar).
    RevendasConvite {
        /// Invalida o link atual e gera outro.
        #[arg(long)]
        regenerar: bool,
    },
    /// Lista adesoes manuais pendentes do convite.
    RevendasConvitePendentes,
    /// Confirma o recebimento de uma adesao manual.
    RevendasConviteConfirmar {
        /// ID do pedido de adesao.
        pedido_id: String,
    },
    /// Le (sem --json) ou salva a oferta de mensalista do convite.
    RevendasConviteConfig {
        #[arg(long, conflicts_with = "file")]
        json: Option<String>,
        #[arg(long, value_name = "caminho", conflicts_with = "json")]
        file: Option<PathBuf>,
    },
    /// Lista/busca clientes finais.
    Usuarios {
        #[command(flatten)]
        list: ListArgs,
    },
    /// Lista pedidos de conteudo.
    Pedidos {
        #[command(flatten)]
        list: ListArgs,
    },
    /// Lista reportes de canal/filme.
    Reportes {
        #[command(flatten)]
        list: ListArgs,
    },
    /// Lista pendencias do catalogo.
    Catalogo {
        #[command(flatten)]
        list: ListArgs,
    },
    /// Lista tickets de suporte.
    Tickets {
        #[command(flatten)]
        list: ListArgs,
    },
    /// Fecha um chamado de suporte.
    TicketsFechar {
        /// ID do chamado.
        id: i64,
    },
    /// Lista avisos do painel.
    Avisos {
        #[command(flatten)]
        list: ListArgs,
    },
    /// Estatisticas do painel.
    Stats,
    /// Dashboard financeiro.
    Dashboard,
    /// Lista as contas Apps Ibo (msproapp).
    AppsIboContas,
    /// Lista os aplicativos de uma conta Apps Ibo.
    AppsIboApps {
        /// ID da conta.
        conta: i64,
    },
    /// Lista os clientes de uma conta Apps Ibo.
    AppsIboClientes {
        /// ID da conta.
        conta: i64,
    },
    /// Ativa um MAC na conta Apps Ibo.
    AppsIboAtivar {
        #[arg(long, conflicts_with = "file")]
        json: Option<String>,
        #[arg(long, value_name = "caminho", conflicts_with = "json")]
        file: Option<PathBuf>,
    },
    /// Remove um cliente Apps Ibo (destrutivo).
    AppsIboExcluir {
        /// ID externo do cliente.
        id: String,
        /// ID da conta.
        #[arg(long)]
        conta: i64,
        /// Confirma a exclusao.
        #[arg(long)]
        confirm: bool,
    },
    /// Atribui um cliente Apps Ibo a uma revenda.
    AppsIboAtribuir {
        /// ID externo do cliente.
        id: String,
        /// ID da conta.
        #[arg(long)]
        conta: i64,
        /// Username da revenda.
        #[arg(long)]
        dono: String,
    },
    /// Saldo Apps Ativa (R$ em centavos).
    AppsAtivaSaldo,
    /// Catalogo de aplicativos Apps Ativa.
    AppsAtivaApps,
    /// Movimentacoes de credito Apps Ativa.
    AppsAtivaHistorico,
    /// Ativacoes Apps Ativa (com status).
    AppsAtivaAtivacoes {
        /// Filtra por status (Solicitado, Ativado, Reprovado, ...).
        #[arg(long)]
        status: Option<String>,
    },
    /// Solicita uma ativacao paga Apps Ativa.
    AppsAtivaAtivar {
        #[arg(long, conflicts_with = "file")]
        json: Option<String>,
        #[arg(long, value_name = "caminho", conflicts_with = "json")]
        file: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Carrega .env (ignorado pelo Git) sem sobrescrever o ambiente.
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "merger_cli=warn".into()),
        )
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    let cfg = ClientConfig::from_env()
        .context("defina ZENITH_URL, ZENITH_USER e ZENITH_PASS (veja .env.example)")?;
    let client = MergerClient::new(cfg)?;

    match cli.cmd {
        Command::Login { codigo } => {
            let codigo = codigo
                .as_deref()
                .map(str::trim)
                .filter(|code| !code.is_empty());
            let outcome = match codigo {
                Some(code) => client.login_with_code(code).await?,
                None => client.login().await?,
            };
            print_json(&serde_json::to_value(outcome)?)?;
        }
        Command::Logout => {
            client.logout().await?;
            print_json(&serde_json::json!({ "ok": true }))?;
        }
        Command::Whoami => {
            client.ensure_login().await?;
            print_json(&client.whoami().await?)?;
        }
        Command::Get { path, list } => {
            client.ensure_login().await?;
            let query = parse_query(&list.query)?;
            let response = client.raw_get(&path, &query).await?;
            print_json(&response.json_or_text())?;
        }
        Command::Post {
            path,
            json,
            file,
            confirm,
        } => {
            let body = read_json_arg(json, file)?;
            client.ensure_login().await?;
            let request = RawRequest::post(path, body).confirm(confirm);
            let response = client.raw(request).await?;
            print_json(&response.json_or_text())?;
        }
        Command::Upload {
            path,
            field,
            file,
            confirm,
        } => {
            let mut upload = RawUpload::new(path).confirm(confirm);
            for par in field {
                let (chave, valor) = par
                    .split_once('=')
                    .with_context(|| format!("campo invalido {par:?} (use chave=valor)"))?;
                upload = upload.field(chave, valor);
            }
            for par in file {
                let (campo, caminho) = par
                    .split_once('=')
                    .with_context(|| format!("arquivo invalido {par:?} (use campo=caminho)"))?;
                let bytes = std::fs::read(caminho)
                    .with_context(|| format!("nao consegui ler {caminho}"))?;
                let filename = std::path::Path::new(caminho)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "arquivo.bin".to_string());
                upload = upload.file(UploadFile::new(campo, filename, bytes));
            }
            client.ensure_login().await?;
            let response = client.raw_upload(upload).await?;
            print_json(&response.json_or_text())?;
        }
        Command::Call {
            json,
            file,
            confirm,
        } => {
            let value = read_json_arg(json, file)?;
            let mut request = raw_request_from(value)?;
            if confirm {
                request.confirm = true;
            }
            client.ensure_login().await?;
            let response = client.raw(request).await?;
            print_json(&serde_json::to_value(response)?)?;
        }
        Command::Revendas { list } => {
            list_panel(&client, "/merger/api/cliente/revendas", &list).await?;
        }
        Command::RevendasCriar { json, file, dias } => {
            let body = read_json_arg(json, file)?;
            client.ensure_login().await?;
            let resposta = client.revendas_criar_com_vencimento(body, dias).await?;
            print_json(&resposta)?;
        }
        Command::RevendasEditar { json, file } => {
            let body = read_json_arg(json, file)?;
            client.ensure_login().await?;
            let resposta = client.revendas_editar(body).await?;
            print_json(&resposta)?;
        }
        Command::RevendasToggle {
            id,
            enable,
            disable,
        } => {
            if enable == disable {
                bail!("use --enable ou --disable (nao os dois)");
            }
            client.ensure_login().await?;
            let resposta = client.revendas_toggle(id, enable).await?;
            print_json(&resposta)?;
        }
        Command::RevendasRemover { id, confirm } => {
            if !confirm {
                bail!("exclusao em cascata exige --confirm");
            }
            client.ensure_login().await?;
            let resposta = client.revendas_remover(id, confirm).await?;
            print_json(&resposta)?;
        }
        Command::RevendasRenovar {
            id,
            vencimento_atual,
        } => {
            client.ensure_login().await?;
            let resposta = client
                .revendas_renovar(serde_json::json!({
                    "id": id,
                    "vencimento_atual": vencimento_atual.unwrap_or_default(),
                }))
                .await?;
            print_json(&resposta)?;
        }
        Command::RevendasNota { json, file } => {
            let body = read_json_arg(json, file)?;
            client.ensure_login().await?;
            let resposta = client.revendas_nota(body).await?;
            print_json(&resposta)?;
        }
        Command::RevendasUnlimited { json, file } => {
            let body = read_json_arg(json, file)?;
            client.ensure_login().await?;
            let resposta = client.revenda_unlimited(body).await?;
            print_json(&resposta)?;
        }
        Command::RevendasTransferir { json, file } => {
            let body = read_json_arg(json, file)?;
            client.ensure_login().await?;
            let resposta = client.transfer_revenda(body).await?;
            print_json(&resposta)?;
        }
        Command::RevendasTransferirDestinos { origem } => {
            client.ensure_login().await?;
            let resposta = client.transfer_revenda_destinos(&origem).await?;
            print_json(&resposta)?;
        }
        Command::RevendasConvite { regenerar } => {
            client.ensure_login().await?;
            let resposta = if regenerar {
                client.convite_regenerar().await?
            } else {
                client.convite().await?
            };
            print_json(&resposta)?;
        }
        Command::RevendasConvitePendentes => {
            client.ensure_login().await?;
            let resposta = client.convite_adesao_pendentes().await?;
            print_json(&resposta)?;
        }
        Command::RevendasConviteConfirmar { pedido_id } => {
            client.ensure_login().await?;
            let resposta = client.convite_adesao_confirmar(&pedido_id).await?;
            print_json(&resposta)?;
        }
        Command::RevendasConviteConfig { json, file } => {
            client.ensure_login().await?;
            let resposta = if json.is_none() && file.is_none() {
                client.convite_mensalista_config().await?
            } else {
                let body = read_json_arg(json, file)?;
                client.convite_mensalista_config_salvar(body).await?
            };
            print_json(&resposta)?;
        }
        Command::Usuarios { list } => {
            list_panel(&client, "/merger/api/cliente/sigma_users", &list).await?;
        }
        Command::Pedidos { list } => {
            list_panel(&client, "/merger/api/pedidos/listar", &list).await?;
        }
        Command::Reportes { list } => {
            list_panel(&client, "/merger/api/reportes/listar", &list).await?;
        }
        Command::Catalogo { list } => {
            list_panel(&client, "/merger/api/cliente/catalogo/pendencias", &list).await?;
        }
        Command::Tickets { list } => {
            let path = match client.config().role {
                merger_core::PanelRole::Admin => "/merger/api/admin/tickets/listar",
                merger_core::PanelRole::Franchise => "/merger/api/cliente/tickets/listar",
            };
            list_panel(&client, path, &list).await?;
        }
        Command::Avisos { list } => {
            list_panel(&client, "/merger/api/avisos", &list).await?;
        }
        Command::TicketsFechar { id } => {
            client.ensure_login().await?;
            let resposta = client.tickets_fechar(id).await?;
            print_json(&resposta)?;
        }
        Command::AppsIboContas => {
            client.ensure_login().await?;
            print_json(&client.apps_ibo_contas().await?)?;
        }
        Command::AppsIboApps { conta } => {
            client.ensure_login().await?;
            print_json(&client.apps_ibo_apps(conta).await?)?;
        }
        Command::AppsIboClientes { conta } => {
            client.ensure_login().await?;
            print_json(&client.apps_ibo_clientes(conta).await?)?;
        }
        Command::AppsIboAtivar { json, file } => {
            let body = read_json_arg(json, file)?;
            client.ensure_login().await?;
            print_json(&client.apps_ibo_ativar(body).await?)?;
        }
        Command::AppsIboExcluir { id, conta, confirm } => {
            if !confirm {
                bail!("exclusao de cliente Apps Ibo exige --confirm");
            }
            client.ensure_login().await?;
            print_json(&client.apps_ibo_excluir(&id, conta, confirm).await?)?;
        }
        Command::AppsIboAtribuir { id, conta, dono } => {
            client.ensure_login().await?;
            print_json(&client.apps_ibo_atribuir(conta, &id, &dono).await?)?;
        }
        Command::AppsAtivaSaldo => {
            client.ensure_login().await?;
            print_json(&client.apps_ativa_saldo().await?)?;
        }
        Command::AppsAtivaApps => {
            client.ensure_login().await?;
            print_json(&client.apps_ativa_apps().await?)?;
        }
        Command::AppsAtivaHistorico => {
            client.ensure_login().await?;
            print_json(&client.apps_ativa_historico().await?)?;
        }
        Command::AppsAtivaAtivacoes { status } => {
            client.ensure_login().await?;
            print_json(&client.apps_ativa_ativacoes(status.as_deref()).await?)?;
        }
        Command::AppsAtivaAtivar { json, file } => {
            let body = read_json_arg(json, file)?;
            client.ensure_login().await?;
            print_json(&client.apps_ativa_ativar(body).await?)?;
        }
        Command::Stats => {
            client.ensure_login().await?;
            let response = client.raw_get("/merger/api/stats", &[]).await?;
            print_json(&response.json_or_text())?;
        }
        Command::Dashboard => {
            client.ensure_login().await?;
            let response = client
                .raw_get("/merger/api/dashboard_financeiro", &[])
                .await?;
            print_json(&response.json_or_text())?;
        }
    }

    Ok(())
}

async fn list_panel(client: &MergerClient, path: &str, args: &ListArgs) -> Result<()> {
    let query = parse_query(&args.query)?;
    client.ensure_login().await?;
    let response = client.raw_get(path, &query).await?;
    print_json(&response.json_or_text())
}

fn parse_query(items: &[String]) -> Result<Vec<(String, String)>> {
    items
        .iter()
        .map(|item| {
            let (key, value) = item
                .split_once('=')
                .with_context(|| format!("query invalida {item:?}: use chave=valor"))?;
            if key.is_empty() {
                bail!("query invalida {item:?}: chave vazia");
            }
            Ok((key.to_string(), value.to_string()))
        })
        .collect()
}

fn read_json_arg(inline: Option<String>, file: Option<PathBuf>) -> Result<Value> {
    match (inline, file) {
        (Some(text), _) => serde_json::from_str(&text).context("JSON invalido em --json"),
        (None, Some(path)) => {
            let text = std::fs::read_to_string(&path)
                .with_context(|| format!("nao consegui ler {}", path.display()))?;
            serde_json::from_str(&text)
                .with_context(|| format!("JSON invalido em {}", path.display()))
        }
        (None, None) => bail!("informe --json '<json>' ou --file <caminho>"),
    }
}

/// Constroi um `RawRequest` aceitando `query` como objeto (`{"page":1}`) ou
/// lista de pares (`[["page","1"]]`), igual ao `POST /v1/raw` do servidor.
fn raw_request_from(value: Value) -> Result<RawRequest> {
    let Value::Object(mut map) = value else {
        bail!("RawRequest precisa ser um objeto JSON");
    };
    let query = match map.remove("query") {
        None | Some(Value::Null) => Vec::new(),
        Some(Value::Object(obj)) => obj
            .into_iter()
            .map(|(key, value)| {
                Ok((
                    key,
                    scalar_to_query(value)
                        .with_context(|| "query aceita somente valores escalares")?,
                ))
            })
            .collect::<Result<Vec<_>>>()?,
        Some(Value::Array(items)) => items
            .into_iter()
            .map(|item| match item {
                Value::Array(pair) if pair.len() == 2 => {
                    let key = scalar_to_query(pair[0].clone())
                        .context("query: chave do par precisa ser escalar")?;
                    let value = scalar_to_query(pair[1].clone())
                        .context("query: valor do par precisa ser escalar")?;
                    Ok((key, value))
                }
                _ => bail!("query: use objeto {{\"chave\":\"valor\"}} ou lista de pares"),
            })
            .collect::<Result<Vec<_>>>()?,
        Some(_) => bail!("query deve ser objeto ou lista de pares"),
    };
    let method = map
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or("GET")
        .trim()
        .to_ascii_uppercase();
    let path = map
        .get("path")
        .and_then(Value::as_str)
        .context("campo path obrigatorio")?
        .to_string();
    let body = map.remove("body").filter(|value| !value.is_null());
    let confirm = map.get("confirm").and_then(Value::as_bool).unwrap_or(false);
    Ok(RawRequest {
        method,
        path,
        query,
        body,
        confirm,
    })
}

fn scalar_to_query(value: Value) -> Result<String> {
    match value {
        Value::String(text) => Ok(text),
        Value::Number(number) => Ok(number.to_string()),
        Value::Bool(flag) => Ok(flag.to_string()),
        Value::Null => Ok(String::new()),
        Value::Array(_) | Value::Object(_) => bail!("valor nao escalar"),
    }
}

fn print_json(value: &Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
