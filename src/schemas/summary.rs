use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::platform::PairKind;
use super::report::EnaFileReport;

/// Breakdown of library types by read layout.
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryTypeBreakdown {
    pub paired_end: usize,
    pub single_end: usize,
}

/// Aggregated summary of ENA metadata for a set of runs.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnaSummary {
    pub total_runs: usize,
    pub unique_samples: usize,
    pub unique_studies: usize,
    pub unique_experiments: usize,
    pub species_breakdown: HashMap<String, usize>,
    pub platform_breakdown: HashMap<String, usize>,
    pub library_type_breakdown: LibraryTypeBreakdown,
    pub total_fastq_files: usize,
}

impl EnaSummary {
    /// Build an aggregated summary from a slice of ENA file reports.
    pub fn from_reports(reports: &[EnaFileReport]) -> Self {
        let mut samples = HashSet::new();
        let mut studies = HashSet::new();
        let mut experiments = HashSet::new();
        let mut species_breakdown: HashMap<String, usize> = HashMap::new();
        let mut platform_breakdown: HashMap<String, usize> = HashMap::new();
        let mut paired_end = 0usize;
        let mut single_end = 0usize;
        let mut total_fastq_files = 0usize;

        for report in reports {
            samples.insert(&report.sample_accession);
            studies.insert(&report.study_accession);
            experiments.insert(&report.experiment_accession);

            *species_breakdown
                .entry(report.scientific_name.clone())
                .or_insert(0) += 1;

            *platform_breakdown
                .entry(report.instrument_platform.name().to_string())
                .or_insert(0) += 1;

            match report.instrument_platform.end_kind() {
                PairKind::PairedEnd => paired_end += 1,
                PairKind::SingleEnd => single_end += 1,
            }

            total_fastq_files += report
                .fastq_ftp
                .split(';')
                .filter(|s| !s.is_empty())
                .count();
        }

        EnaSummary {
            total_runs: reports.len(),
            unique_samples: samples.len(),
            unique_studies: studies.len(),
            unique_experiments: experiments.len(),
            species_breakdown,
            platform_breakdown,
            library_type_breakdown: LibraryTypeBreakdown {
                paired_end,
                single_end,
            },
            total_fastq_files,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::platform::Platform;

    fn make_report(
        platform: Platform,
        scientific_name: &str,
        fastq_ftp: &str,
        sample: &str,
        study: &str,
        experiment: &str,
        run: &str,
    ) -> EnaFileReport {
        EnaFileReport {
            study_accession: study.to_string(),
            sample_accession: sample.to_string(),
            experiment_accession: experiment.to_string(),
            run_accession: run.to_string(),
            tax_id: "0".to_string(),
            scientific_name: scientific_name.to_string(),
            instrument_platform: platform,
            fastq_ftp: fastq_ftp.to_string(),
            fastq_md5: "".to_string(),
        }
    }

    #[test]
    fn test_mixed_platforms() {
        let reports = vec![
            make_report(
                Platform::Illumina,
                "Homo sapiens",
                "f1.fq.gz;f2.fq.gz",
                "SAMN1",
                "PRJNA1",
                "SRX1",
                "SRR1",
            ),
            make_report(
                Platform::Nanopore,
                "Homo sapiens",
                "f3.fq.gz",
                "SAMN2",
                "PRJNA1",
                "SRX2",
                "SRR2",
            ),
        ];

        let summary = EnaSummary::from_reports(&reports);

        assert_eq!(summary.total_runs, 2);
        assert_eq!(summary.unique_samples, 2);
        assert_eq!(summary.unique_studies, 1);
        assert_eq!(summary.unique_experiments, 2);
        assert_eq!(summary.platform_breakdown["ILLUMINA"], 1);
        assert_eq!(summary.platform_breakdown["OXFORD_NANOPORE"], 1);
        assert_eq!(summary.library_type_breakdown.paired_end, 1);
        assert_eq!(summary.library_type_breakdown.single_end, 1);
        assert_eq!(summary.total_fastq_files, 3);
    }

    #[test]
    fn test_multiple_species() {
        let reports = vec![
            make_report(
                Platform::Illumina,
                "Homo sapiens",
                "f1.fq.gz;f2.fq.gz",
                "SAMN1",
                "PRJNA1",
                "SRX1",
                "SRR1",
            ),
            make_report(
                Platform::Illumina,
                "Mus musculus",
                "f3.fq.gz;f4.fq.gz",
                "SAMN2",
                "PRJNA2",
                "SRX2",
                "SRR2",
            ),
            make_report(
                Platform::Illumina,
                "Homo sapiens",
                "f5.fq.gz;f6.fq.gz",
                "SAMN3",
                "PRJNA1",
                "SRX3",
                "SRR3",
            ),
        ];

        let summary = EnaSummary::from_reports(&reports);

        assert_eq!(summary.species_breakdown["Homo sapiens"], 2);
        assert_eq!(summary.species_breakdown["Mus musculus"], 1);
        assert_eq!(summary.unique_studies, 2);
        assert_eq!(summary.total_fastq_files, 6);
    }

    #[test]
    fn test_single_run() {
        let reports = vec![make_report(
            Platform::PacBio,
            "Escherichia coli",
            "f1.fq.gz",
            "SAMN1",
            "PRJNA1",
            "SRX1",
            "SRR1",
        )];

        let summary = EnaSummary::from_reports(&reports);

        assert_eq!(summary.total_runs, 1);
        assert_eq!(summary.unique_samples, 1);
        assert_eq!(summary.unique_studies, 1);
        assert_eq!(summary.unique_experiments, 1);
        assert_eq!(summary.platform_breakdown["PACBIO"], 1);
        assert_eq!(summary.library_type_breakdown.paired_end, 0);
        assert_eq!(summary.library_type_breakdown.single_end, 1);
        assert_eq!(summary.total_fastq_files, 1);
    }

    #[test]
    fn test_fastq_file_counting() {
        let reports = vec![
            make_report(
                Platform::Illumina,
                "Homo sapiens",
                "f1.fq.gz;f2.fq.gz",
                "SAMN1",
                "PRJNA1",
                "SRX1",
                "SRR1",
            ),
            make_report(
                Platform::Nanopore,
                "Homo sapiens",
                "f3.fq.gz",
                "SAMN2",
                "PRJNA1",
                "SRX2",
                "SRR2",
            ),
        ];

        let summary = EnaSummary::from_reports(&reports);

        // Illumina paired = 2 files, Nanopore single = 1 file
        assert_eq!(summary.total_fastq_files, 3);
    }
}
