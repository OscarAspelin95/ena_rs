use thiserror::Error;

use crate::schemas::platform::Platform;

/// Application error types.
#[derive(Debug, Error)]
pub enum AppError {
    /// JSON serialization/deserialization failed.
    #[error("Failed to serialize")]
    SerializationError(#[from] serde_json::Error),

    /// Sequencing platform doesn't match expected file count (e.g., paired-end vs single-end).
    #[error("Platform mismatch: {0}")]
    PlatformMismatchError(String),

    /// File system operation failed.
    #[error("Io error")]
    IoError(#[from] std::io::Error),

    /// HTTP request failed.
    #[error("Request error")]
    RequestError(#[from] reqwest::Error),

    /// Logger initialization failed.
    #[error("Log error")]
    LogError(#[from] log::SetLoggerError),

    /// Fastq download failed.
    #[error("Fastq download failed")]
    FastqMd5MismatchError(FailedFastq),
}

#[derive(Debug, Clone)]
pub struct FailedFastq {
    pub sample_accession: String,
    pub run_accession: String,
    pub platform: Platform,
    pub ftp: String,
    pub reason: String,
}
