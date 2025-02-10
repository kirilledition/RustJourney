mod arguments;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
// cargo run fetchpdb 4hhb 8pvw 9cq5 9bwf asf --output downloaded/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .json()
        .flatten_event(true)
        .init();

    let cli = arguments::Cli::parse();

    match &cli.command {
        arguments::Commands::Fetchpdb(arguments) => fetchpdb(arguments).await,
    }
}

async fn fetchpdb(arguments: &arguments::FetchpdbArguments) -> Result<(), Box<dyn Error>> {
    const BASE_URL: &str = "https://files.rcsb.org/download/";
    let client = reqwest::Client::new();
    let base_url = reqwest::Url::parse(BASE_URL)?;

    if !arguments.output_path.is_dir() {
        fs::create_dir_all(arguments.output_path.clone()).await?;
        tracing::info!("Creating directory {}", arguments.output_path.display());
    };

    let download_jobs = arguments.codes.iter().map(move |code| {
        let client = client.clone();
        let base_url = base_url.clone();
        let path = arguments.output_path.clone();

        async move {
            let pdb_filename = PathBuf::from(code).with_extension("pdb");
            let url = base_url.join(pdb_filename.to_str().unwrap())?;

            let response = client.get(url).send().await?;
            if !response.status().is_success() {
                tracing::warn!("Request for {code} was unsuccessfull");
                return Ok(());
            }

            let pdb_text = response.bytes().await?;
            let filename = path.join(pdb_filename);

            tracing::info!("Writing {}", filename.display());
            let mut file = fs::File::create(filename).await?;
            file.write_all(&pdb_text).await?;
            Ok::<(), Box<dyn Error>>(())
        }
    });

    futures::future::join_all(download_jobs).await;
    Ok(())
}
