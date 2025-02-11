mod programs;

use clap::Parser;
use clap::Subcommand;
use std::error::Error;
// cargo run fetchpdb 4hhb 8pvw 9cq5 9bwf asf --output downloaded/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .json()
        .flatten_event(true)
        .init();

    let cli = CLI::parse();

    match &cli.command {
        Commands::Fetchpdb(arguments) => programs::fetchpdb::run(arguments).await,
        Commands::Parquetify(arguments) => programs::parquetify::run(arguments),
    }
}

#[derive(Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Program to download list of PDB structure files")]
    Fetchpdb(programs::fetchpdb::Arguments),
    #[command(about = "Program to convert text table file to parquet file")]
    Parquetify(programs::parquetify::Arguments),
}
