use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PairKind {
    SingleEnd,
    PairedEnd,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Pairs {
    First,
    Second,
}

impl Pairs {
    pub fn suffix(&self) -> &str {
        match self {
            Pairs::First => "1",
            Pairs::Second => "2",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "ILLUMINA")]
    Illumina,
    #[serde(rename = "IONTORRENT")]
    IonTorrent,
    #[serde(rename = "PACBIO")]
    PacBio,
    #[serde(rename = "OXFORD_NANOPORE")]
    Nanopore,
}

impl Platform {
    pub fn abbreviation(&self) -> &str {
        match self {
            Platform::Illumina => "il",
            Platform::IonTorrent => "it",
            Platform::PacBio => "pb",
            Platform::Nanopore => "np",
        }
    }

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
            PairKind::PairedEnd => {},
            _ => panic!("Illumina should be PairedEnd"),
        }

        match Platform::IonTorrent.end_kind() {
            PairKind::SingleEnd => {},
            _ => panic!("IonTorrent should be SingleEnd"),
        }

        match Platform::PacBio.end_kind() {
            PairKind::SingleEnd => {},
            _ => panic!("PacBio should be SingleEnd"),
        }

        match Platform::Nanopore.end_kind() {
            PairKind::SingleEnd => {},
            _ => panic!("Nanopore should be SingleEnd"),
        }
    }

    #[test]
    fn test_pairs_suffix() {
        assert_eq!(Pairs::First.suffix(), "1");
        assert_eq!(Pairs::Second.suffix(), "2");
    }
}
