use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct App {
    #[arg(
        short,
        long,
        help = "ENA accession. Can be run, sample or study accession."
    )]
    pub accession: String,

    #[arg(short, long, help = "Where to output files.")]
    pub outdir: PathBuf,
}
