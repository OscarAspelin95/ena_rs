use crate::errors::AppError;
use crate::schema::{DownloadSpec, EnaFileReport};
use log::{error, info};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub enum EnaUrlType {
    Https,
}

impl Display for EnaUrlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnaUrlType::Https => write!(f, "https://"),
        }
    }
}

pub enum EnaFormat {
    Json,
}

impl Display for EnaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnaFormat::Json => write!(f, "json"),
        }
    }
}
pub struct EnaUrl {
    base: String,
    fields: String,
    url_type: EnaUrlType,
    format: EnaFormat,
}

impl EnaUrl {
    pub fn new() -> Self {
        Self{
			base: "www.ebi.ac.uk/ena/portal/api/filereport".to_string(),
			fields: "study_accession,sample_accession,experiment_accession,run_accession,tax_id,scientific_name,instrument_platform,fastq_bytes,fastq_ftp,fastq_md5".to_string(),
			url_type: EnaUrlType::Https,
			format: EnaFormat::Json,
		}
    }

    pub fn build(&self, accession: &str) -> String {
        let url = format!(
            "{}{}?accession={}&result=read_run&fields={}&format={}",
            self.url_type, self.base, accession, self.fields, self.format
        );
        url
    }
}

pub async fn fetch_data(url: &str) -> Result<Vec<EnaFileReport>, AppError> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let data: Vec<EnaFileReport> = serde_json::from_slice(&bytes)?;

    Ok(data)
}

/// Make classmethod later on.
pub async fn parse_data(data: &[EnaFileReport], outdir: PathBuf) -> Result<(), AppError> {
    for ena_report in data {
        // Get ftps for this file run.
        let download_spec: DownloadSpec = match ena_report.download_spec(&outdir) {
            Ok(download_spec) => download_spec,
            Err(err) => {
                error!("{err}");
                continue;
            }
        };

        for ((fq_ftp, fq_md5), fq_local) in download_spec
            .fastq_ftps
            .into_iter()
            .zip(download_spec.fastq_md5s)
            .zip(download_spec.fastq_locals)
        {
            println!(
                "{} {} {} {:?}",
                fq_ftp, fq_md5, fq_local, ena_report.instrument_platform
            );

            info!("Downloading {}", fq_ftp);

            let outfile = File::create(&fq_local)?;
            let mut writer = BufWriter::new(outfile);

            let fq_url = format!("https://{}", fq_ftp);

            let response = reqwest::get(fq_url).await?;
            let bytes = response.bytes().await?;

            writer.write_all(&bytes)?;
            writer.flush()?;

            let p = PathBuf::from(&fq_local);
            match p.exists() {
                true => info!("File {} downloaded successfully", fq_local.as_str()),
                false => {
                    error!("Failed to download {}", fq_local.as_str());
                    continue;
                }
            }

            let digest = md5::compute(&bytes);
            let actual_md5_sum = format!("{:x}", digest);

            match actual_md5_sum == fq_md5 {
                true => {
                    info!("Md5 sum matches")
                }
                false => {
                    error!("MD5 sum does not match. Deleting file.");
                    std::fs::remove_file(&fq_local)?;
                }
            }
        }
    }

    Ok(())
}
