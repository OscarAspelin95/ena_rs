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
    pub fn suffix<'a>(&'a self) -> &'a str {
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
    pub fn abbreviation<'a>(&'a self) -> &'a str {
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
