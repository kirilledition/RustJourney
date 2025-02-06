mod arguments;
mod context;

use tokio::fs;
use tokio::io::AsyncWriteExt;

use bytes::Bytes;
use clap::Parser;
use std::error::Error;
// cargo run 4hhb 8pvw 9cq5 9bwf

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().json().flatten_event(true).init();

    let arguments = arguments::Arguments::parse();
    let context = context::Context::new();

    download_multiple_pdb(arguments.codes, &context).await
}

async fn download_pdb(code: &str, context: &context::Context) -> Result<Bytes, Box<dyn Error>> {
    let url = context.base_url.join(code)?;
    let response = context.client.get(url).send().await?;
    response
        .bytes()
        .await
        .map_err(|error| Box::new(error) as Box<dyn Error>)
}

async fn download_multiple_pdb(
    codes: Vec<String>,
    context: &context::Context,
) -> Result<(), Box<dyn Error>> {
    let download_jobs = codes.iter().map(move |code| async move {
        let pdb_text = download_pdb(&code, &context).await?;
        let filename = format!("{code}.pdb");
        tracing::info!("Writing {}", filename);
        let mut file = fs::File::create(filename).await?;
        file.write_all(&pdb_text).await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    });

    futures::future::join_all(download_jobs).await;
    Ok(())
}
