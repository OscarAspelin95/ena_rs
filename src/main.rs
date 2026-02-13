use clap::Parser;
use log::{LevelFilter, info};
use simple_logger::SimpleLogger;
use tokio::{self, fs::create_dir};

mod args;
use args::{App, Commands};

use ena_rs::AppError;
use ena_rs::ena::{fetch_fastqs, fetch_metadata};
use ena_rs::schemas::summary::EnaSummary;
use ena_rs::schemas::url::EnaUrl;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    SimpleLogger::new().with_level(LevelFilter::Info).init()?;

    let args = App::parse();

    match args.command {
        Commands::Download { accession, outdir } => {
            let ena_url = EnaUrl::new().build(&accession);

            if !outdir.exists() {
                create_dir(&outdir).await?;
            }

            info!("Fetching data from {}", ena_url);
            let data = fetch_metadata(&ena_url).await?;

            match fetch_fastqs(&data, outdir).await {
                Ok(all_succeeded) => match all_succeeded {
                    true => info!("All samples downloaded successfully"),
                    false => {
                        info!("Some samples failed to download, check the log file for details")
                    }
                },
                Err(err) => return Err(err),
            }
        }
        Commands::Summary { accession, output } => {
            let ena_url = EnaUrl::new().build(&accession);

            info!("Fetching metadata from {}", ena_url);
            let data = fetch_metadata(&ena_url).await?;

            let summary = EnaSummary::from_reports(&data);
            let json = serde_json::to_string_pretty(&summary)?;

            match output {
                Some(path) => {
                    tokio::fs::write(&path, &json).await?;
                    info!("Summary written to {}", path.display());
                }
                None => {
                    println!("{}", json);
                }
            }
        }
    }

    Ok(())
}
