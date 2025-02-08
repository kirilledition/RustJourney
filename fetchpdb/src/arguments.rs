use std::path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    author = "Kirill Denisov",
    about = "Program that downloads PDB structure files"
)]
pub struct Arguments {
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
