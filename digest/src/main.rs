mod errors;
mod models;

use chrono::Utc;
use errors::{ConfigError, NewsletterError};
use models::{Newsletter, NewsletterConfig};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::sync;

const CONFIG_PATH: &str = "digest.toml";
// const SECONDS_IN_WEEK: i64 = 604800;
const SECONDS_IN_WEEK: i64 = 1204800;

static UNNECESSARY_SYMBOLS_REGEX: sync::LazyLock<Regex> =
    sync::LazyLock::new(|| Regex::new(r"[\[\]\*\n]").unwrap());

static URL_REGEX: sync::LazyLock<Regex> = sync::LazyLock::new(|| {
    Regex::new(r"(http|ftp|https):\/\/[\w\-_]+(\.[\w\-_]+)+([\w\-\.,@?^=%&amp;:/~\+#]*[\w\-\@?^=%&amp;/~\+#])?").unwrap()
});

#[tokio::main]
async fn main() -> Result<(), NewsletterError> {
    run().await
}

async fn run() -> Result<(), NewsletterError> {
    let config = read_config(CONFIG_PATH)?;

    let (mut newsletter, newsletter_text) = create_newsletter_html(config)?;

    write_newsletter_to_file(&mut newsletter, newsletter_text)?;

    let page = post_to_telegraph(&mut newsletter).await;
    println!("{page:?}");

    Ok(())
}

fn read_config(config_path: &str) -> Result<NewsletterConfig, ConfigError> {
    let mut config_string = String::new();
    let _ = File::open(config_path)?.read_to_string(&mut config_string)?;

    toml::from_str::<NewsletterConfig>(&config_string).map_err(|e| e.into())
}

fn create_newsletter_html(
    config: NewsletterConfig,
) -> Result<(Newsletter, String), NewsletterError> {
    let mut newsletter = Newsletter::from(config);
    let newsletter_text = newsletter.to_html();
    Ok((newsletter, newsletter_text))
}

fn write_newsletter_to_file(
    newsletter: &mut Newsletter,
    newsletter_text: String,
) -> Result<(), NewsletterError> {
    newsletter
        .output_file
        .write(newsletter_text.as_bytes())
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn post_to_telegraph(newsletter: &mut Newsletter) -> telegraph_rs::Page {
    let telegraph = telegraph_rs::Telegraph::new(&newsletter.title)
        .create()
        .await
        .unwrap();
    println!("created telegraph object");
    let newsletter_text = newsletter.to_html();
    println!("created newsletter text");

    let newsletter_title = format!(
        "{} for {}",
        &newsletter.title,
        Utc::now().format("week %W (%e %B)")
    );
    println!("created newsletter title");

    let node_text = telegraph_rs::html_to_node(newsletter_text.as_str());
    println!("created newsletter nodes {node_text:?}");

    let page = telegraph
        .create_page(newsletter_title.as_str(), &node_text, false)
        .await
        .unwrap();
    println!("posted page");

    page
}
