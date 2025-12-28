use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to serialize")]
    SerializationError(#[from] serde_json::Error),

    #[error("Platform mismatch: {0}")]
    PlatformMismatchError(String),

    #[error("Io error")]
    IoError(#[from] std::io::Error),

    #[error("Request error")]
    RequestError(#[from] reqwest::Error),

    #[error("Log error")]
    LogError(#[from] log::SetLoggerError),
}
