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

    /// Metadata download failed.
    #[error("Failed to download ENA metadata")]
    MetadataDownloadError(String),

    /// Fastq download failed.
    #[error("Fastq download failed")]
    FastqDownloadError(FailedFastq),

    #[error("Fastq download timeout")]
    TimeoutError(String),
}

#[derive(Debug, Clone)]
pub struct FailedFastq {
    pub sample_accession: String,
    pub run_accession: String,
    pub platform: Platform,
    pub ftp: String,
    pub reason: String,
}

impl AppError {
    /// Extract a human-readable reason from the error.
    pub fn reason(&self) -> String {
        match self {
            AppError::FastqDownloadError(f) => f.reason.clone(),
            AppError::TimeoutError(s) => s.clone(),
            _ => self.to_string(),
        }
    }
}
