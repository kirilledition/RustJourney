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
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Fetchpdb(programs::fetchpdb::Arguments),
}
