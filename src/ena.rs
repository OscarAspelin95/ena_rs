use crate::download_utils::{DownloadStatus, progress_bar};
use crate::errors::AppError;
use crate::schemas::{download::DownloadSpec, report::EnaFileReport};
use std::path::PathBuf;

use crate::download::download_single_file_with_retry;

/// Fetches ENA file metadata from the given URL.
///
/// # Arguments
/// * `url` - ENA API URL to fetch metadata from
///
/// # Returns
/// Vec of ENA file reports containing download information.
pub async fn fetch_metadata(url: &str) -> Result<Vec<EnaFileReport>, AppError> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let data: Vec<EnaFileReport> = serde_json::from_slice(&bytes)?;

    Ok(data)
}

/// Validates file contents against expected MD5 checksum.
///
/// # Arguments
/// * `bytes` - File contents to validate
/// * `expected_md5` - Expected MD5 hash as hex string
///
/// # Returns
/// `true` if MD5 matches, `false` otherwise.
pub fn valid_md5<T: AsRef<[u8]>>(bytes: T, expected_md5: &str) -> bool {
    let digest = md5::compute(&bytes);
    let actual_md5_sum = format!("{:x}", digest);

    actual_md5_sum == expected_md5
}

/// Downloads FASTQ files based on EnaFileReports.
///
/// Downloads the associated FASTQ files and validates them using MD5 checksums.
/// Invalid files are automatically deleted and appended to a vec of failed samples.
///
/// # Arguments
/// * `data` - Slice of ENA file reports
/// * `outdir` - Directory to save downloaded files
///
/// # Returns
/// `Ok(Vec<AppError>)` containing any download failures.
pub async fn fetch_fastqs(
    data: &[EnaFileReport],
    outdir: PathBuf,
) -> Result<Vec<AppError>, AppError> {
    let mut failed_samples: Vec<AppError> = Vec::new();
    let bar = progress_bar(data.len() as u64);

    for ena_report in data {
        // We should fix this. Preferably, we want to continue
        // downloading the remaining files.
        let download_spec: DownloadSpec = match ena_report.get_download_spec(&outdir) {
            Ok(spec) => spec,
            Err(err) => {
                failed_samples.push(AppError::MetadataDownloadError(err.to_string()));
                bar.println(format!("{} {}", ena_report.run_accession, err.to_string()));
                bar.inc(1);
                continue;
            }
        };

        let mut download_status = DownloadStatus::success();
        let mut total_bytes = 0usize;

        for (fq_ftp, fq_md5, fq_local) in download_spec {
            // Extract filename for progress display
            let filename = fq_local.rsplit('/').next().unwrap_or(&fq_local);
            bar.set_message(filename.to_string());

            match download_single_file_with_retry(&fq_ftp, &fq_md5, &fq_local, ena_report).await {
                Ok(bytes) => total_bytes += bytes,
                Err(err) => {
                    download_status = DownloadStatus::failure(&err.reason());
                    failed_samples.push(err);
                }
            }
        }

        bar.inc(1);
        bar.println(download_status.fmt_for_bar(
            &ena_report.run_accession,
            &ena_report.instrument_platform,
            total_bytes,
        ));
    }

    bar.finish();
    Ok(failed_samples)
}
