use clap::Parser;
use log::{LevelFilter, info};
use simple_logger::SimpleLogger;
use tokio::{self, fs::create_dir};

mod args;
use args::App;

use ena_rs::AppError;
use ena_rs::ena::{fetch_fastqs, fetch_metadata};
use ena_rs::schemas::url::EnaUrl;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    SimpleLogger::new().with_level(LevelFilter::Info).init()?;

    let args = App::parse();

    let ena_url = EnaUrl::new().build(&args.accession);
    create_dir(&args.outdir).await?;

    info!("Fetching data from {}", ena_url);
    let data = fetch_metadata(&ena_url).await?;

    match fetch_fastqs(&data, args.outdir).await {
        Ok(failed_samples) => match failed_samples.len() {
            0 => info!("All samples downloaded successfully"),
            _ => info!("{} samples failed to download", failed_samples.len()),
        },
        Err(err) => return Err(err),
    }

    Ok(())
}
