use crate::platform::{PairKind, Platform};
use crate::{errors::AppError, platform::Pairs};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnaFileReport {
    pub study_accession: String,
    pub sample_accession: String,
    pub experiment_accession: String,
    pub run_accession: String,
    pub tax_id: String,
    pub scientific_name: String,
    pub instrument_platform: Platform,
    pub fastq_ftp: String,
    pub fastq_md5: String,
}

pub struct DownloadSpec {
    pub fastq_ftps: Vec<String>,
    pub fastq_md5s: Vec<String>,
    pub fastq_locals: Vec<String>,
}

impl EnaFileReport {
    pub fn get_single_end(&self, outdir: &PathBuf, pair: Pairs) -> String {
        format!(
            "{}/{}_{}_{}_{}.fastq.gz",
            outdir.display(),
            self.sample_accession,
            self.run_accession,
            self.instrument_platform.abbreviation(),
            pair.suffix()
        )
    }

    pub fn validate_vec(&self, vec: Vec<String>) -> Result<Vec<String>, AppError> {
        match (self.instrument_platform.end_kind(), vec.len()) {
            (PairKind::SingleEnd, 1) => Ok(vec),
            (PairKind::PairedEnd, 2) => Ok(vec),
            _ => Err(AppError::PlatformMismatchError("".into())),
        }
    }

    pub fn get_fastq_ftp(&self) -> Result<Vec<String>, AppError> {
        let fastq_ftps: Vec<String> = self.fastq_ftp.split(";").map(String::from).collect();
        self.validate_vec(fastq_ftps)
    }

    pub fn get_fastq_md5(&self) -> Result<Vec<String>, AppError> {
        let fastq_md5s: Vec<String> = self.fastq_md5.split(";").map(String::from).collect();
        self.validate_vec(fastq_md5s)
    }

    pub fn get_fastq_local(&self, outdir: &PathBuf) -> Vec<String> {
        match self.instrument_platform.end_kind() {
            PairKind::SingleEnd => {
                let pe1 = self.get_single_end(outdir, Pairs::First);
                vec![pe1]
            }
            PairKind::PairedEnd => {
                let pe1 = self.get_single_end(outdir, Pairs::First);
                let pe2 = self.get_single_end(outdir, Pairs::Second);
                vec![pe1, pe2]
            }
        }
    }

    pub fn download_spec(&self, outdir: &PathBuf) -> Result<DownloadSpec, AppError> {
        let fastq_ftps = self.get_fastq_ftp()?;
        let fastq_md5s = self.get_fastq_md5()?;
        let fastq_locals = self.get_fastq_local(outdir);

        let download_spec = DownloadSpec {
            fastq_ftps: fastq_ftps,
            fastq_md5s: fastq_md5s,
            fastq_locals: fastq_locals,
        };

        Ok(download_spec)
    }
}
