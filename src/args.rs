use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Command-line arguments for the ENA tool.
#[derive(Debug, Parser)]
#[command(version, about = "CLI tool for working with ENA data.", long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Download FASTQ files from ENA
    Download {
        #[arg(
            short,
            long,
            help = "ENA accession. Can be run, sample or study accession."
        )]
        accession: String,

        #[arg(short, long, help = "Where to output files.")]
        outdir: PathBuf,
    },
    /// Summarize ENA metadata without downloading files
    Summary {
        #[arg(
            short,
            long,
            help = "ENA accession. Can be run, sample or study accession."
        )]
        accession: String,

        #[arg(
            short,
            long,
            help = "Output file path for summary JSON. Defaults to stdout."
        )]
        output: Option<PathBuf>,
    },
}
