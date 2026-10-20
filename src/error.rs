//! Error types
//!
//! - [`BotError`] is returned by commands; its [`Display`](std::fmt::Display) is shown to the
//!   Discord user, so it should always be a friendly, complete sentence
//! - [`CdClientLoadError`], [`LocaleLoadError`], and [`ConfigError`] happen while starting up or
//!   reloading; they are logged and (for `/reload`) shown to the admin who ran the command

use std::path::PathBuf;

pub type BotResult<T> = Result<T, BotError>;

#[derive(Debug, thiserror::Error)]
pub enum BotError {
    /// Something the user asked for does not exist (message is already formatted)
    #[error("{0}")]
    NotFound(String),

    /// The user gave an argument that could not be understood
    #[error("{0}")]
    InvalidArgument(String),

    /// The thing exists, but the requested information does not apply to it
    /// (e.g. `/vendor` on an object that sells nothing)
    #[error("{0}")]
    NotApplicable(String),

    /// The user is not allowed to run this
    #[error("{0}")]
    Forbidden(String),

    /// A bug or a data problem; logged with full detail, shown to the user generically
    #[error("Internal error: {0}")]
    Internal(String),
}

impl BotError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn invalid(msg: impl Into<String>) -> Self {
        Self::InvalidArgument(msg.into())
    }

    pub fn not_applicable(msg: impl Into<String>) -> Self {
        Self::NotApplicable(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// Short title for the error embed
    pub fn title(&self) -> &'static str {
        match self {
            BotError::NotFound(_) => "Not Found",
            BotError::InvalidArgument(_) => "Invalid Input",
            BotError::NotApplicable(_) => "Nothing To Show",
            BotError::Forbidden(_) => "Not Allowed",
            BotError::Internal(_) => "Something Went Wrong",
        }
    }

    /// Whether this should be logged as an error (as opposed to normal user mistakes)
    pub fn is_internal(&self) -> bool {
        matches!(self, BotError::Internal(_))
    }

    /// What the user should see
    pub fn user_message(&self) -> String {
        match self {
            BotError::Internal(_) => String::from(
                "An unexpected error occurred while running this command. It has been logged.",
            ),
            other => other.to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CdClientLoadError {
    #[error("cdclient.sqlite was not found at `{0}`")]
    NotFound(PathBuf),

    #[error("could not open `{path}` as an SQLite database: {source}")]
    Open {
        path: PathBuf,
        source: rusqlite::Error,
    },

    #[error("could not load required table `{table}` from cdclient.sqlite: {source}")]
    Table {
        table: &'static str,
        source: rusqlite::Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum LocaleLoadError {
    #[error("locale.xml was not found at `{0}`")]
    NotFound(PathBuf),

    #[error("could not read `{path}`: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("locale `{locale}` was not found in locale.xml (available: {available})")]
    MissingLocale { locale: String, available: String },
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing required environment variable `{0}` (see .env.template)")]
    Missing(&'static str),

    #[error("environment variable `{name}` is invalid: {reason} (got `{value}`)")]
    Invalid {
        name: &'static str,
        value: String,
        reason: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum DataLoadError {
    #[error(transparent)]
    CdClient(#[from] CdClientLoadError),

    #[error(transparent)]
    Locale(#[from] LocaleLoadError),
}
