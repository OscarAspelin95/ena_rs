use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments for the ENA downloader.
#[derive(Debug, Parser)]
pub struct App {
    /// ENA accession number (run, sample, or study accession).
    #[arg(
        short,
        long,
        help = "ENA accession. Can be run, sample or study accession."
    )]
    pub accession: String,

    /// Output directory for downloaded files.
    #[arg(short, long, help = "Where to output files.")]
    pub outdir: PathBuf,
}
