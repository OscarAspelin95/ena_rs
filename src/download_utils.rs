use crate::schemas::platform::Platform;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn progress_bar(num_tasks: u64) -> ProgressBar {
    let bar = ProgressBar::new(num_tasks);

    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:50.cyan/blue} {msg} {pos:>7}/{len:7}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    bar.enable_steady_tick(Duration::from_millis(1000));

    bar
}

/// Formats a byte count into a human-readable string (KB, MB, GB).
pub fn format_bytes(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub struct DownloadStatus {
    pub success: bool,
    pub message: Option<String>,
}

impl DownloadStatus {
    pub fn success() -> Self {
        Self {
            success: true,
            message: None,
        }
    }

    pub fn failure(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(msg.into()),
        }
    }

    pub fn fmt(&self) -> String {
        match (self.success, &self.message) {
            (true, _) => "✓".to_string(),
            (false, Some(msg)) => format!("✗ {}", msg),
            _ => "✗".to_string(),
        }
    }

    pub fn fmt_for_bar(&self, accession: &str, platform: &Platform, total_bytes: usize) -> String {
        match self.success {
            true => format!(
                "{accession} {} {} {}",
                platform.abbreviation(),
                self.fmt(),
                format_bytes(total_bytes)
            ),
            false => format!("{accession} {} {}", platform.abbreviation(), self.fmt()),
        }
    }
}
