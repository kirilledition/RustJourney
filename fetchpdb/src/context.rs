use reqwest::Url;

const URL_BASE: &str = "https://files.rcsb.org/download";

pub struct Context {
    pub client: reqwest::Client,
    pub base_url: Url,
}

impl Context {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: Url::parse(URL_BASE).expect("PDB base URL was wrong"),
        }
    }
}
