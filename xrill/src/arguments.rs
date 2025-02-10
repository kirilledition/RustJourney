use std::path;

use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Fetchpdb(FetchpdbArguments),
}

#[derive(Args, Debug)]
pub struct FetchpdbArguments {
    #[arg(required = true, help = "PDB codes to download")]
    pub codes: Vec<String>,
    #[arg(
        short,
        long = "output",
        help = "Path to directory where to download files",
        default_value = "."
    )]
    pub output_path: path::PathBuf,
}
