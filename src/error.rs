//! Error types for the FMP client.

use thiserror::Error;

/// Result type alias using our Error type
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when using the FMP client
#[derive(Debug, Error)]
pub enum Error {
    /// Error from the HTTP client
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Error parsing JSON response
    #[error("Failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),

    /// API returned an error response
    #[error("API error: {message} (status: {status})")]
    Api { status: u16, message: String },

    /// Missing API key
    #[error("API key not found. Please set FMP_API_KEY environment variable")]
    MissingApiKey,

    /// Invalid API key format
    #[error("Invalid API key format")]
    InvalidApiKey,

    /// Invalid parameter
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Please try again later")]
    RateLimitExceeded,

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// URL parsing error
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Generic error with custom message
    #[error("{0}")]
    Custom(String),
}

impl Error {
    /// Create a new API error
    pub fn api(status: u16, message: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: message.into(),
        }
    }

    /// Create a new custom error
    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }

    /// Check if this is a rate limit error
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Error::RateLimitExceeded)
            || matches!(self, Error::Api { status, .. } if *status == 429)
    }

    /// Check if this is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::NotFound(_))
            || matches!(self, Error::Api { status, .. } if *status == 404)
    }
}
