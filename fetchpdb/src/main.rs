use bytes;
use std::{
    error::Error,
    io::{self, Write},
};

const URL_BASE: &str = "https://files.rcsb.org/download";
// cargo run 4hhb

struct Context {
    client: reqwest::blocking::Client,
}

impl Context {
    fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let pdb_code = std::env::args().nth(1).ok_or("no code provided")?;

    let context = Context::new();
    let pdb_text = download_pdb(&pdb_code, context)?;
    io::stdout().write_all(&pdb_text)?;
    Ok(())
}

fn download_pdb(code: &str, context: Context) -> Result<bytes::Bytes, Box<dyn Error>> {
    let url = format!("{URL_BASE}/{code}.pdb");
    let response = context.client.get(url).send()?;

    Ok(response.bytes()?)
}
