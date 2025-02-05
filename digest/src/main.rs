mod config;
mod errors;
mod models;

use chrono::Utc;
use config::{AppConfig, ModelConfig, TimeRange};
use errors::{ConfigError, NewsletterError};
use models::{Fetched, Newsletter};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::sync;
use tracing::{debug, info, Level};

const CONFIG_BASENAME: &str = "digest";

static UNNECESSARY_SYMBOLS_REGEX: sync::LazyLock<Regex> =
    sync::LazyLock::new(|| Regex::new(r"[\[\]\*\n]").unwrap());

static URL_REGEX: sync::LazyLock<Regex> = sync::LazyLock::new(|| {
    Regex::new(r"(http|ftp|https):\/\/[\w\-_]+(\.[\w\-_]+)+([\w\-\.,@?^=%&amp;:/~\+#]*[\w\-\@?^=%&amp;/~\+#])?").unwrap()
});

pub(crate) type Result<T, E = NewsletterError> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    run().await
}

async fn run() -> Result<()> {
    let config = AppConfig::new(CONFIG_BASENAME)?;
    debug!("Received config: {config:?}");

    let mut output_file = File::create(&config.output_file)?;
    info!("Will write to {}", config.output_file);

    info!("Fetching newsletter");
    let mut newsletter = fetch_newsletter(config.clone()).await?;
    let newsletter_text = newsletter.to_html(&config.model).await;

    write_newsletter_to_file(newsletter_text, &mut output_file)?;

    info!("Posting to telegraph");
    let page = post_to_telegraph(&mut newsletter, &config.model).await;
    debug!("{page:?}");

    Ok(())
}

pub(crate) async fn fetch_newsletter(config: AppConfig) -> Result<Newsletter<Fetched>> {
    let tr = TimeRange::parse_feeds_config(&config.feeds).ok_or_else(|| ConfigError::ParseDate)?;
    let newsletter = Newsletter::from(config);
    let newsletter = newsletter.into_fetched(tr).await;
    Ok(newsletter)
}

#[inline(always)]
fn write_newsletter_to_file(newsletter_text: String, output_file: &mut File) -> Result<()> {
    output_file
        .write(newsletter_text.as_bytes())
        .map(|_| ())
        .map_err(|e| e.into())
}

pub(crate) async fn post_to_telegraph(
    newsletter: &mut Newsletter<Fetched>,
    config: &ModelConfig,
) -> Result<telegraph_rs::Page> {
    let telegraph = telegraph_rs::Telegraph::new(&newsletter.title)
        .create()
        .await?;
    debug!("created telegraph object");
    let newsletter_text = newsletter.to_html(config).await;
    debug!("created newsletter text");

    let newsletter_title = format!(
        "{} for {}",
        &newsletter.title,
        Utc::now().format("week %W (%e %B)")
    );
    debug!("created newsletter title");

    let node_text = telegraph_rs::html_to_node(newsletter_text.as_str());
    debug!("created newsletter nodes {node_text:?}");

    let page = telegraph
        .create_page(newsletter_title.as_str(), &node_text, false)
        .await?;
    debug!("posted page");

    Ok(page)
}
