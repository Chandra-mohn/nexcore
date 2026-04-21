use thiserror::Error;

#[derive(Debug, Error)]
pub enum DoctorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error in {file}: {message}")]
    Parse { file: String, message: String },

    #[error("Connectivity error: {0}")]
    Connectivity(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
