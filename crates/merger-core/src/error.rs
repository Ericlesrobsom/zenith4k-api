/// Erros do cliente.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("configuracao invalida: {0}")]
    Config(String),

    #[error("erro de rede: {0}")]
    Http(#[from] reqwest::Error),

    #[error("resposta nao e JSON valido: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL invalida: {0}")]
    Url(#[from] url::ParseError),

    #[error("nao autenticado: chame login() antes")]
    NotAuthenticated,

    #[error("sessao expirada no painel")]
    SessionExpired,

    #[error("sem permissao no painel: {0}")]
    Forbidden(String),

    #[error("rota nao encontrada no painel")]
    NotFound,

    #[error("rate limit do painel (tente de novo em {retry_after_secs:?}s)")]
    RateLimited { retry_after_secs: Option<u64> },

    #[error("2FA necessario; codigo enviado para {email_mascarado:?}")]
    TwoFactorRequired { email_mascarado: Option<String> },

    #[error("codigo 2FA invalido ou expirado")]
    InvalidTwoFactor,

    #[error("login recusado: {0}")]
    LoginRejected(String),

    #[error("rota fora da allowlist: {0}")]
    PathNotAllowed(String),

    #[error("operacao potencialmente destrutiva exige confirm=true: {0}")]
    ConfirmationRequired(String),

    #[error("painel respondeu {status}: {body}")]
    Panel { status: u16, body: String },

    #[error("funcionalidade ainda nao implementada: {0}")]
    NotImplemented(&'static str),
}

pub type Result<T> = std::result::Result<T, ApiError>;
