use reqwest::StatusCode;
use std::fmt;

/// Errors returned by the SAP Concur API client.
#[derive(Debug, thiserror::Error)]
pub enum ConcurError {
    /// HTTP transport error.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization / deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parsing error.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// An error returned by the SAP Concur API.
    #[error("API error {status}: {message}")]
    Api {
        status: StatusCode,
        message: String,
        correlation_id: Option<String>,
    },

    /// Authentication error.
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Retry budget exhausted.
    #[error("Retry budget exhausted after {attempts} attempts")]
    RetryExhausted { attempts: u32 },

    /// A required field or parameter was missing.
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    /// Generic error message.
    #[error("{0}")]
    Other(String),
}

impl ConcurError {
    pub fn api(status: StatusCode, message: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: message.into(),
            correlation_id: None,
        }
    }

    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        if let Self::Api { correlation_id, .. } = &mut self {
            *correlation_id = Some(id.into());
        }
        self
    }
}

/// A generic result type used throughout the crate.
pub type Result<T> = std::result::Result<T, ConcurError>;

/// Standard SAP Concur error response body.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ConcurApiError {
    pub error: Option<String>,
    #[serde(rename = "error_description")]
    pub error_description: Option<String>,
    pub code: Option<i32>,
    pub timestamp: Option<String>,
    pub error_message: Option<String>,
    pub error_id: Option<String>,
    pub path: Option<String>,
}

impl fmt::Display for ConcurApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(msg) = &self.error_message {
            write!(f, "{}", msg)
        } else if let Some(desc) = &self.error_description {
            write!(f, "{}", desc)
        } else if let Some(err) = &self.error {
            write!(f, "{}", err)
        } else {
            write!(f, "{:?}", self)
        }
    }
}
