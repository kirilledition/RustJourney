use bytes::Bytes;
use clap::Parser;
use reqwest::Url;
use std::{error::Error, fs, io::Write};

const URL_BASE: &str = "https://files.rcsb.org/download";
// cargo run 4hhb

#[derive(Parser, Debug)]
#[command(
    version,
    author = "Kirill Denisov",
    about = "Program that downloads pdb structure files"
)]
struct Arguments {
    #[arg(required = true, help = "PDB codes to download")]
    codes: Vec<String>,
}

struct Context {
    client: reqwest::blocking::Client,
    base_url: Url,
}

impl Context {
    fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: Url::parse(URL_BASE).expect("pdb base url was wrong"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().json().flatten_event(true).init();

    let arguments = Arguments::parse();
    let context = Context::new();

    for code in arguments.codes {
        let pdb_text = download_pdb(&code, &context)?;
        let filename = format!("{code}.pdb");
        tracing::info!("Writing {}", filename);
        let mut file = fs::File::create(filename)?;
        file.write_all(&pdb_text)?;
    }

    Ok(())
}

fn download_pdb(code: &str, context: &Context) -> Result<Bytes, Box<dyn Error>> {
    let url = context.base_url.join(code)?;
    let response = context.client.get(url).send()?;
    Ok(response.bytes()?)
}
