use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Schema Registry error {code}: {message}")]
    Api { code: i32, message: String },

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
