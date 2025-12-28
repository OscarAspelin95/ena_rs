use crate::errors::{AppError, FailedFastq};
use crate::schemas::{download::DownloadSpec, report::EnaFileReport};
use indicatif::{ProgressBar, ProgressStyle};
use log::error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Duration;

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
/// `Ok` if all downloads completed.

fn progress_bar(num_tasks: u64) -> ProgressBar {
    let bar = ProgressBar::new(num_tasks).with_message("Samples: ".to_string());

    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:70.cyan/blue} {msg} {pos:>7}/{len:7}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    bar.enable_steady_tick(Duration::from_millis(1000));

    bar
}

struct DownloadStatus {
    success: bool,
    message: Option<String>,
}

impl DownloadStatus {
    fn fmt(&self) -> String {
        match (self.success, &self.message) {
            (true, _) => "✓".to_string(),
            (false, Some(msg)) => format!("✗ {}", msg),
            _ => "✗".to_string(),
        }
    }
}
pub async fn fetch_fastqs(
    data: &[EnaFileReport],
    outdir: PathBuf,
) -> Result<Vec<AppError>, AppError> {
    let mut failed_samples: Vec<AppError> = Vec::new();

    let bar = progress_bar(data.len() as u64);

    for ena_report in data {
        let download_spec: DownloadSpec = match ena_report.download_spec(&outdir) {
            Ok(download_spec) => download_spec,
            Err(err) => {
                error!("{err}");
                continue;
            }
        };

        let mut download_status = DownloadStatus {
            success: true,
            message: None,
        };

        for ((fq_ftp, fq_md5), fq_local) in download_spec
            .fastq_ftps
            .into_iter()
            .zip(download_spec.fastq_md5s)
            .zip(download_spec.fastq_locals)
        {
            let outfile = File::create(&fq_local)?;
            let mut writer = BufWriter::new(outfile);

            let fq_url = format!("https://{}", fq_ftp);

            let response = match reqwest::get(fq_url).await {
                Ok(response) => response,
                Err(err) => {
                    failed_samples.push(AppError::FastqMd5MismatchError(FailedFastq {
                        sample_accession: ena_report.sample_accession.clone(),
                        run_accession: ena_report.run_accession.clone(),
                        platform: ena_report.instrument_platform.clone(),
                        ftp: fq_ftp.clone(),
                        reason: err.to_string(),
                    }));

                    download_status.success = false;
                    download_status.message = Some(err.to_string());
                    continue;
                }
            };

            let bytes = match response.bytes().await {
                Ok(bytes) => bytes,
                Err(err) => {
                    failed_samples.push(AppError::FastqMd5MismatchError(FailedFastq {
                        sample_accession: ena_report.sample_accession.clone(),
                        run_accession: ena_report.run_accession.clone(),
                        platform: ena_report.instrument_platform.clone(),
                        ftp: fq_ftp.clone(),
                        reason: err.to_string(),
                    }));

                    download_status.success = false;
                    download_status.message = Some(err.to_string());
                    continue;
                }
            };

            writer.write_all(&bytes)?;
            writer.flush()?;

            if valid_md5(&bytes, &fq_md5) == false {
                std::fs::remove_file(&fq_local)?;
                failed_samples.push(AppError::FastqMd5MismatchError(FailedFastq {
                    sample_accession: ena_report.sample_accession.clone(),
                    run_accession: ena_report.run_accession.clone(),
                    platform: ena_report.instrument_platform.clone(),
                    ftp: fq_ftp.clone(),
                    reason: "MD5 mismatch".to_string(),
                }));

                download_status.success = false;
                download_status.message = Some("MD5 mismatch".to_string());
            }
        }
        bar.inc(1);
        bar.println(format! {"{} {}", ena_report.run_accession, download_status.fmt()});
    }

    bar.finish();
    Ok(failed_samples)
}
