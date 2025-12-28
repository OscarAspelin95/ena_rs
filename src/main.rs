use clap::Parser;
use log::{LevelFilter, info};
use simple_logger::SimpleLogger;
use tokio::{self, fs::create_dir};

mod args;
use args::App;

use ena_rs::AppError;
use ena_rs::schemas::url::EnaUrl;
use ena_rs::ena::{fetch_data, parse_data};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    SimpleLogger::new().with_level(LevelFilter::Info).init()?;

    let args = App::parse();

    let ena_url = EnaUrl::new().build(&args.accession);
    create_dir(&args.outdir).await?;

    info!("Fetching data from {}", ena_url);
    let data = fetch_data(&ena_url).await?;

    parse_data(&data, args.outdir).await?;

    Ok(())
}
