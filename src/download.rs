use indicatif::ProgressBar;
use tokio::io::AsyncWriteExt;

use crate::AppError;
use crate::errors::FailedFastq;
use crate::schemas::report::EnaFileReport;
use console::style;
use std::time::Duration;

pub async fn download_single_file_with_retry(
    fq_ftp: &str,
    fq_md5: &str,
    fq_local: &str,
    ena_report: &EnaFileReport,
    bar: &ProgressBar,
) -> Result<usize, AppError> {
    let max_num_retries: usize = 3;
    let mut num_retries: usize = 0;

    loop {
        let timeout_limit = (num_retries * 5) + 5;
        let seconds_sleep = num_retries * 5;

        // Run download with timeout (in seconds).
        let _ = tokio::time::sleep(Duration::from_secs(seconds_sleep as u64)).await;
        let result_with_timeout = tokio::time::timeout(
            Duration::from_secs(timeout_limit as u64 * 60),
            download_single_file(fq_ftp, fq_md5, fq_local, ena_report),
        )
        .await;

        // If timed out, we need to retry
        let download_result = match result_with_timeout {
            Ok(download_result) => download_result,
            Err(err) => {
                // Clean up partial file if it exists
                let _ = tokio::fs::remove_file(fq_local).await;

                if num_retries >= max_num_retries {
                    return Err(AppError::TimeoutError(err.to_string()));
                }

                num_retries += 1;
                let msg = format!(
                    "{} timeout exceeded, retrying ({num_retries}/{max_num_retries}).",
                    &ena_report.run_accession
                );
                bar.println(style(msg).yellow().to_string());
                continue;
            }
        };

        match download_result {
            Ok(bytes_written) => return Ok(bytes_written),
            Err(err) => {
                // Clean up partial file if it exists (MD5 failures already clean up)
                let _ = tokio::fs::remove_file(fq_local).await;

                if num_retries >= max_num_retries {
                    return Err(AppError::FastqDownloadError(err));
                }
                num_retries += 1;
                let msg = format!(
                    "{} download failed, retrying ({num_retries}/{max_num_retries}).",
                    &ena_report.run_accession
                );
                bar.println(style(msg).yellow().to_string());
                continue;
            }
        }
    }
}
/// Downloads a single FASTQ file and validates its MD5 checksum.
///
/// # Arguments
/// * `fq_ftp` - FTP path to the FASTQ file
/// * `fq_md5` - Expected MD5 checksum
/// * `fq_local` - Local file path to save the file
/// * `ena_report` - ENA report containing sample metadata (for error context)
///
/// # Returns
/// `Ok(bytes_written)` on success, `Err(FailedFastq)` on failure.
pub async fn download_single_file(
    fq_ftp: &str,
    fq_md5: &str,
    fq_local: &str,
    ena_report: &EnaFileReport,
) -> Result<usize, FailedFastq> {
    let make_error = |reason: String| FailedFastq {
        sample_accession: ena_report.sample_accession.clone(),
        run_accession: ena_report.run_accession.clone(),
        platform: ena_report.instrument_platform.clone(),
        ftp: fq_ftp.to_string(),
        reason,
    };

    let fq_url = format!("https://{}", fq_ftp);

    let mut response = reqwest::get(&fq_url)
        .await
        .map_err(|e| make_error(e.to_string()))?;

    let outfile = tokio::fs::File::create(fq_local)
        .await
        .map_err(|e| make_error(e.to_string()))?;
    let mut writer = tokio::io::BufWriter::new(outfile);
    let mut md5_context = md5::Context::new();
    let mut total_bytes: usize = 0;

    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|e| make_error(e.to_string()))?
    {
        writer
            .write_all(&chunk)
            .await
            .map_err(|e| make_error(e.to_string()))?;
        md5_context.consume(&chunk);
        total_bytes += chunk.len();
    }

    writer
        .flush()
        .await
        .map_err(|e| make_error(e.to_string()))?;

    let digest = md5_context.finalize();
    let actual_md5 = format!("{:x}", digest);

    if actual_md5 != fq_md5 {
        drop(writer);
        tokio::fs::remove_file(fq_local).await.ok();
        return Err(make_error("MD5 mismatch".to_string()));
    }

    Ok(total_bytes)
}
