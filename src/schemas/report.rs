use crate::errors::AppError;
use crate::schemas::{
    download::DownloadSpec,
    platform::{PairKind, Pairs, Platform},
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use struct_field_names_as_array::FieldNamesAsSlice;

/// ENA file report containing metadata and download information for a sequencing run.
///
/// This structure is deserialized from ENA API responses and contains all necessary
/// information to download and validate FASTQ files.
#[derive(Debug, Serialize, Deserialize, FieldNamesAsSlice)]
pub struct EnaFileReport {
    /// Study accession (e.g., PRJNA123456)
    pub study_accession: String,
    /// Sample accession (e.g., SAMN01234567)
    pub sample_accession: String,
    /// Experiment accession (e.g., SRX123456)
    pub experiment_accession: String,
    /// Run accession (e.g., SRR123456)
    pub run_accession: String,
    /// NCBI taxonomy ID
    pub tax_id: String,
    /// Species scientific name
    pub scientific_name: String,
    /// Sequencing platform
    pub instrument_platform: Platform,
    /// Semicolon-separated FTP URLs for FASTQ files
    pub fastq_ftp: String,
    /// Semicolon-separated MD5 checksums
    pub fastq_md5: String,
}

impl EnaFileReport {
    /// Constructs a local filename for a FASTQ file.
    ///
    /// Format: `{sample}_{run}_{platform}_{pair}.fastq.gz`
    pub fn get_single_end(&self, outdir: &Path, pair: &Pairs) -> String {
        format!(
            "{}/{}_{}_{}_{}.fastq.gz",
            outdir.display(),
            self.sample_accession,
            self.run_accession,
            self.instrument_platform.abbreviation(),
            pair.suffix()
        )
    }

    /// Validates that the number of files matches the platform's expected read type.
    ///
    /// Single-end platforms expect 1 file, paired-end platforms expect 2 files.
    pub fn validate(&self, vec: Vec<String>) -> Result<Vec<String>, AppError> {
        match (self.instrument_platform.end_kind(), vec.len()) {
            (PairKind::SingleEnd, 1) => Ok(vec),
            (PairKind::PairedEnd, 2) => Ok(vec),
            _ => Err(AppError::PlatformMismatchError("".into())),
        }
    }

    /// Parses and validates FTP URLs from the semicolon-separated field.
    pub fn get_fastq_ftp(&self) -> Result<Vec<String>, AppError> {
        let fastq_ftps: Vec<String> = self.fastq_ftp.split(";").map(String::from).collect();
        self.validate(fastq_ftps)
    }

    /// Parses and validates MD5 checksums from the semicolon-separated field.
    pub fn get_fastq_md5(&self) -> Result<Vec<String>, AppError> {
        let fastq_md5s: Vec<String> = self.fastq_md5.split(";").map(String::from).collect();
        self.validate(fastq_md5s)
    }

    /// Generates local file paths for FASTQ files based on platform type.
    ///
    /// Returns 1 path for single-end, 2 paths for paired-end.
    pub fn get_fastq_local(&self, outdir: &Path) -> Vec<String> {
        match self.instrument_platform.end_kind() {
            PairKind::SingleEnd => {
                let pe1 = self.get_single_end(outdir, &Pairs::First);
                vec![pe1]
            }
            PairKind::PairedEnd => {
                let pe1 = self.get_single_end(outdir, &Pairs::First);
                let pe2 = self.get_single_end(outdir, &Pairs::Second);
                vec![pe1, pe2]
            }
        }
    }

    /// Creates a complete download specification with FTP URLs, checksums, and local paths.
    pub fn download_spec(&self, outdir: &Path) -> Result<DownloadSpec, AppError> {
        let fastq_ftps = self.get_fastq_ftp()?;
        let fastq_md5s = self.get_fastq_md5()?;
        let fastq_locals = self.get_fastq_local(outdir);

        let download_spec = DownloadSpec {
            fastq_ftps,
            fastq_md5s,
            fastq_locals,
        };

        Ok(download_spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_report(platform: Platform, fastq_ftp: &str, fastq_md5: &str) -> EnaFileReport {
        EnaFileReport {
            study_accession: "PRJNAXXXXXX".to_string(),
            sample_accession: "SAMNXXXXXX".to_string(),
            experiment_accession: "SRXXXXXXX".to_string(),
            run_accession: "SRRXXXXXX".to_string(),
            tax_id: "X".to_string(),
            scientific_name: "Some Species".to_string(),
            instrument_platform: platform,
            fastq_ftp: fastq_ftp.to_string(),
            fastq_md5: fastq_md5.to_string(),
        }
    }

    #[test]
    fn test_get_fastq_ftp_single_end() {
        let report = create_test_report(
            Platform::Nanopore,
            "ftp.sra.ebi.ac.uk/vol1/file1.fastq.gz",
            "abc123",
        );

        let result = report.get_fastq_ftp().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "ftp.sra.ebi.ac.uk/vol1/file1.fastq.gz");
    }

    #[test]
    fn test_get_fastq_ftp_paired_end() {
        let report = create_test_report(
            Platform::Illumina,
            "ftp.sra.ebi.ac.uk/vol1/file1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/file2.fastq.gz",
            "abc123;def456",
        );

        let result = report.get_fastq_ftp().unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "ftp.sra.ebi.ac.uk/vol1/file1.fastq.gz");
        assert_eq!(result[1], "ftp.sra.ebi.ac.uk/vol1/file2.fastq.gz");
    }

    #[test]
    fn test_get_fastq_md5_single_end() {
        let report = create_test_report(
            Platform::PacBio,
            "ftp.sra.ebi.ac.uk/vol1/file1.fastq.gz",
            "abc123",
        );

        let result = report.get_fastq_md5().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "abc123");
    }

    #[test]
    fn test_get_fastq_md5_paired_end() {
        let report = create_test_report(
            Platform::Illumina,
            "file1.fastq.gz;file2.fastq.gz",
            "abc123;def456",
        );

        let result = report.get_fastq_md5().unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "abc123");
        assert_eq!(result[1], "def456");
    }

    #[test]
    fn test_validate_single_end_accepts_one_item() {
        let report = create_test_report(Platform::Nanopore, "file.fastq.gz", "abc123");
        let vec = vec!["item1".to_string()];

        let result = report.validate(vec);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_paired_end_accepts_two_items() {
        let report = create_test_report(
            Platform::Illumina,
            "file1.fastq.gz;file2.fastq.gz",
            "abc123;def456",
        );
        let vec = vec!["item1".to_string(), "item2".to_string()];

        let result = report.validate(vec);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_rejects_mismatch_single_end_with_two_items() {
        let report = create_test_report(Platform::PacBio, "file.fastq.gz", "abc123");
        let vec = vec!["item1".to_string(), "item2".to_string()];

        let result = report.validate(vec);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_rejects_mismatch_paired_end_with_one_item() {
        let report = create_test_report(Platform::Illumina, "file.fastq.gz", "abc123");
        let vec = vec!["item1".to_string()];

        let result = report.validate(vec);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_fastq_local_single_end() {
        let report = create_test_report(Platform::IonTorrent, "file.fastq.gz", "abc123");
        let outdir = PathBuf::from("/output");

        let result = report.get_fastq_local(&outdir);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "/output/SAMNXXXXXX_SRRXXXXXX_it_1.fastq.gz");
    }

    #[test]
    fn test_get_fastq_local_paired_end() {
        let report = create_test_report(
            Platform::Illumina,
            "file1.fastq.gz;file2.fastq.gz",
            "abc123;def456",
        );
        let outdir = PathBuf::from("/output");

        let result = report.get_fastq_local(&outdir);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "/output/SAMNXXXXXX_SRRXXXXXX_il_1.fastq.gz");
        assert_eq!(result[1], "/output/SAMNXXXXXX_SRRXXXXXX_il_2.fastq.gz");
    }

    #[test]
    fn test_get_single_end_filename_format() {
        let report = create_test_report(Platform::Nanopore, "file.fastq.gz", "abc123");
        let outdir = PathBuf::from("/data");

        let filename = report.get_single_end(&outdir, &Pairs::First);
        assert_eq!(filename, "/data/SAMNXXXXXX_SRRXXXXXX_np_1.fastq.gz");
    }
}
