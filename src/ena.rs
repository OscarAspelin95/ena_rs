use crate::download_utils::{format_bytes, progress_bar};
use crate::errors::AppError;
use crate::schemas::{download::DownloadSpec, report::EnaFileReport};
use console::style;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

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

fn get_writer(outdir: &Path) -> BufWriter<File> {
    let f = outdir.join("failed_samples.txt");
    let file = File::create(&f).expect("Failed to create output file for failed samples.");
    BufWriter::new(file)
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
pub async fn fetch_fastqs(data: &[EnaFileReport], outdir: PathBuf) -> Result<bool, AppError> {
    let mut all_succeeded = true;
    let bar = progress_bar(data.len() as u64);

    let mut writer = get_writer(&outdir);

    for ena_report in data {
        let download_spec: DownloadSpec = match ena_report.get_download_spec(&outdir) {
            Ok(spec) => spec,
            Err(err) => {
                //
                let msg = format!(
                    "{} {} {}",
                    ena_report.run_accession,
                    ena_report.instrument_platform.abbreviation(),
                    err.reason()
                );
                bar.println(style(msg).red().to_string());
                bar.inc(1);

                //
                all_succeeded = false;
                writer.write_all(
                    format!(
                        "{}\t{}\t{}\n",
                        ena_report.run_accession,
                        "invalid ENA report format",
                        err.reason(),
                    )
                    .as_bytes(),
                )?;
                continue;
            }
        };

        let mut total_bytes = 0usize;

        for (fq_ftp, fq_md5, fq_local) in download_spec {
            // Extract filename for progress display
            let filename = fq_local.rsplit('/').next().unwrap_or(&fq_local);
            bar.set_message(filename.to_string());

            let download_result =
                download_single_file_with_retry(&fq_ftp, &fq_md5, &fq_local, ena_report, &bar)
                    .await;

            match download_result {
                Ok(bytes) => total_bytes += bytes,
                Err(err) => {
                    all_succeeded = false;
                    // Progress bar
                    let msg = format!(
                        "{} {} {}",
                        &ena_report.run_accession,
                        &ena_report.instrument_platform.abbreviation(),
                        err.reason()
                    );
                    bar.println(style(msg).red().to_string());

                    // File
                    writer.write_all(
                        format!(
                            "{}\t{}\t{}\n",
                            ena_report.run_accession,
                            "download failed",
                            err.reason(),
                        )
                        .as_bytes(),
                    )?;
                }
            };
        }

        bar.inc(1);

        if total_bytes > 0 {
            let msg = format!(
                "{} {} {}",
                &ena_report.run_accession,
                &ena_report.instrument_platform.abbreviation(),
                format_bytes(total_bytes)
            );
            bar.println(style(msg).green().to_string());
        }
    }

    bar.finish();
    writer.flush()?;

    Ok(all_succeeded)
}
