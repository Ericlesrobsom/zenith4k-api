use clap::{Parser, Subcommand};
use merger_core::{ClientConfig, MergerClient};

#[derive(Debug, Parser)]
#[command(name = "zenith4k", version, about = "CLI do painel goiptv merger")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Valida a sessao no painel.
    Whoami,
    /// Faz uma chamada GET crua.
    Get {
        path: String,
        /// Par de query no formato chave=valor (repetivel).
        #[arg(short, long)]
        query: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "merger_cli=info".into()),
        )
        .init();

    let cli = Cli::parse();
    let cfg = ClientConfig::from_env()?;
    let client = MergerClient::new(cfg)?;
    client.login().await?;

    match cli.cmd {
        Command::Whoami => {
            let who = client.whoami().await?;
            println!("{}", serde_json::to_string_pretty(&who)?);
        }
        Command::Get { path, query } => {
            let q: Vec<(String, String)> = query
                .iter()
                .filter_map(|s| s.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())))
                .collect();
            let resp = client.raw_get(&path, &q).await?;
            println!("{}", serde_json::to_string_pretty(&resp.json_or_text())?);
        }
    }
    Ok(())
}
