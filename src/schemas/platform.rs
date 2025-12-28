use serde::{Deserialize, Serialize};

/// Sequencing read type (single-end or paired-end).
#[derive(Debug, Serialize, Deserialize)]
pub enum PairKind {
    /// Single-end sequencing (one file per run)
    SingleEnd,
    /// Paired-end sequencing (two files per run)
    PairedEnd,
}

/// Paired-end read designation.
#[derive(Debug, Serialize, Deserialize)]
pub enum Pairs {
    /// First read in a pair (R1)
    First,
    /// Second read in a pair (R2)
    Second,
}

impl Pairs {
    /// Returns the numeric suffix for filename construction (1 or 2).
    pub fn suffix(&self) -> &str {
        match self {
            Pairs::First => "1",
            Pairs::Second => "2",
        }
    }
}

/// Sequencing platform/technology.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Platform {
    /// Illumina sequencing platform (paired-end)
    #[serde(rename = "ILLUMINA")]
    Illumina,
    /// Ion Torrent platform (single-end)
    #[serde(rename = "IONTORRENT")]
    IonTorrent,
    /// Pacific Biosciences platform (single-end)
    #[serde(rename = "PACBIO")]
    PacBio,
    /// Oxford Nanopore platform (single-end)
    #[serde(rename = "OXFORD_NANOPORE")]
    Nanopore,
}

impl Platform {
    /// Returns platform abbreviation for filename construction.
    pub fn abbreviation(&self) -> &str {
        match self {
            Platform::Illumina => "il",
            Platform::IonTorrent => "it",
            Platform::PacBio => "pb",
            Platform::Nanopore => "np",
        }
    }

    /// Returns the read type (single-end or paired-end) for this platform.
    pub fn end_kind(&self) -> PairKind {
        match self {
            Platform::Illumina => PairKind::PairedEnd,
            Platform::IonTorrent => PairKind::SingleEnd,
            Platform::PacBio => PairKind::SingleEnd,
            Platform::Nanopore => PairKind::SingleEnd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_abbreviations() {
        assert_eq!(Platform::Illumina.abbreviation(), "il");
        assert_eq!(Platform::IonTorrent.abbreviation(), "it");
        assert_eq!(Platform::PacBio.abbreviation(), "pb");
        assert_eq!(Platform::Nanopore.abbreviation(), "np");
    }

    #[test]
    fn test_platform_end_kinds() {
        match Platform::Illumina.end_kind() {
            PairKind::PairedEnd => {}
            _ => panic!("Illumina should be PairedEnd"),
        }

        match Platform::IonTorrent.end_kind() {
            PairKind::SingleEnd => {}
            _ => panic!("IonTorrent should be SingleEnd"),
        }

        match Platform::PacBio.end_kind() {
            PairKind::SingleEnd => {}
            _ => panic!("PacBio should be SingleEnd"),
        }

        match Platform::Nanopore.end_kind() {
            PairKind::SingleEnd => {}
            _ => panic!("Nanopore should be SingleEnd"),
        }
    }

    #[test]
    fn test_pairs_suffix() {
        assert_eq!(Pairs::First.suffix(), "1");
        assert_eq!(Pairs::Second.suffix(), "2");
    }
}
