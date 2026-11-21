use url::Url;

use crate::error::{ApiError, Result};

/// Papel usado no login contra o painel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PanelRole {
    /// Franquia / revenda: `POST /merger/api/cliente_login`.
    Franchise,
    /// Admin mestre ou equipe: `POST /merger/api/login`.
    Admin,
}

impl Default for PanelRole {
    fn default() -> Self {
        Self::Franchise
    }
}

impl std::str::FromStr for PanelRole {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "cliente" | "franquia" | "revenda" | "franchise" | "client" => Ok(Self::Franchise),
            "admin" | "mestre" | "master" => Ok(Self::Admin),
            outro => Err(ApiError::Config(format!(
                "papel invalido {outro:?} (use cliente ou admin)"
            ))),
        }
    }
}

/// Configuracao do cliente.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// URL do painel como o usuario acessa (pode incluir `/zenith4k/`).
    pub panel_url: Url,
    pub username: String,
    pub password: String,
    /// Franquia raiz quando o login e de uma sub-revenda.
    pub franquia: Option<String>,
    pub role: PanelRole,
    pub timeout_secs: u64,
    pub user_agent: String,
}

impl ClientConfig {
    pub fn new(
        panel_url: impl AsRef<str>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self> {
        let raw = panel_url.as_ref().trim();
        let mut url = Url::parse(raw)?;
        if url.scheme() != "http" && url.scheme() != "https" {
            return Err(ApiError::Config(
                "a URL do painel precisa ser http ou https".into(),
            ));
        }
        // normaliza: sem query, sem fragmento, com barra final
        url.set_query(None);
        url.set_fragment(None);
        if !url.path().ends_with('/') {
            let p = format!("{}/", url.path());
            url.set_path(&p);
        }

        Ok(Self {
            panel_url: url,
            username: username.into(),
            password: password.into(),
            franquia: None,
            role: PanelRole::default(),
            timeout_secs: 30,
            user_agent: concat!("zenith4k-api/", env!("CARGO_PKG_VERSION")).to_string(),
        })
    }

    /// Le `ZENITH_*` do ambiente (veja `.env.example`).
    pub fn from_env() -> Result<Self> {
        let panel_url = std::env::var("ZENITH_URL")
            .or_else(|_| std::env::var("ZENITH_BASE_URL"))
            .map_err(|_| ApiError::Config("defina ZENITH_URL".into()))?;
        let username =
            std::env::var("ZENITH_USER").map_err(|_| ApiError::Config("defina ZENITH_USER".into()))?;
        let password =
            std::env::var("ZENITH_PASS").map_err(|_| ApiError::Config("defina ZENITH_PASS".into()))?;

        let mut cfg = Self::new(panel_url, username, password)?;
        if let Ok(f) = std::env::var("ZENITH_FRANQUIA") {
            let f = f.trim();
            if !f.is_empty() {
                cfg.franquia = Some(f.to_string());
            }
        }
        if let Ok(role) = std::env::var("ZENITH_ROLE") {
            cfg.role = role.parse()?;
        }
        if let Ok(t) = std::env::var("ZENITH_TIMEOUT_SECS") {
            if let Ok(secs) = t.trim().parse::<u64>() {
                if secs > 0 {
                    cfg.timeout_secs = secs;
                }
            }
        }
        Ok(cfg)
    }

    /// Origem sem caminho: `https://host[:porta]`.
    pub fn api_origin(&self) -> Result<Url> {
        let mut origin = self.panel_url.clone();
        origin.set_path("/");
        Ok(origin)
    }
}
