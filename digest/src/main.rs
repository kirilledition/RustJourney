use chrono::{DateTime, Duration, Utc};
use regex::Regex;
use serde_derive::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::sync;
use telegraph_rs::{html_to_node, Telegraph};

const CONFIG_PATH: &str = "digest.toml";
const SECONDS_IN_WEEK: i64 = 604800;

static UNNECESSARY_SYMBOLS_REGEX: sync::LazyLock<Regex> =
    sync::LazyLock::new(|| Regex::new(r"[\[\]\*\n]").unwrap());

static URL_REGEX: sync::LazyLock<Regex> = sync::LazyLock::new(|| {
    Regex::new(r"(http|ftp|https):\/\/[\w\-_]+(\.[\w\-_]+)+([\w\-\.,@?^=%&amp;:/~\+#]*[\w\-\@?^=%&amp;/~\+#])?").unwrap()
});

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    match rt.block_on(run()) {
        Ok(_) => (),
        Err(error) => {
            println!("Error {error}");
            exit(1)
        }
    }
    exit(0)
}

async fn run() -> Result<(), Box<dyn Error>> {
    let config = match read_config(CONFIG_PATH) {
        Ok(config) => config,
        Err(error) => return Err(error),
    };
    let (mut newsletter, newsletter_text) =
        match tokio::task::spawn_blocking(move || create_newsletter_html(config)).await? {
            Ok(text) => text,
            Err(error) => return Err(error),
        };

    match write_newsletter_to_file(&mut newsletter, newsletter_text) {
        Ok(_) => (),
        Err(error) => return Err(error),
    };

    let page = post_to_telegraph(&mut newsletter).await;
    println!("{page:?}");

    Ok(())
}

fn read_config(config_path: &str) -> Result<NewsletterConfig, Box<dyn Error>> {
    let mut config_string = String::new();
    let mut config_file = match File::open(config_path) {
        Ok(file) => file,
        Err(error) => return Err(error.into()),
    };

    match config_file.read_to_string(&mut config_string) {
        Ok(_) => (),
        Err(error) => return Err(error.into()),
    }

    match toml::from_str::<NewsletterConfig>(&config_string) {
        Ok(config) => return Ok(config),
        Err(error) => return Err(error.into()),
    };
}

fn create_newsletter_html(
    config: NewsletterConfig,
) -> Result<(Newsletter, String), Box<dyn Error + Send>> {
    let mut newsletter = Newsletter::from(config);
    let newsletter_text = newsletter.to_html();
    Ok((newsletter, newsletter_text))
}

fn write_newsletter_to_file(
    newsletter: &mut Newsletter,
    newsletter_text: String,
) -> Result<(), Box<dyn Error>> {
    match newsletter.output_file.write(newsletter_text.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(error.into()),
    }
}

#[derive(Debug, Deserialize)]
struct NewsletterConfig {
    title: String,
    output_file: String,
    feeds: Vec<FeedConfig>,
}

#[derive(Clone, Debug, Deserialize)]
struct FeedConfig {
    name: String,
    feed_url: String,
    regex_filter: String,
}

#[derive(Debug)]
struct Newsletter {
    title: String,
    output_file: File,
    feeds: Vec<Feed>,
}

impl Newsletter {
    fn from(config: NewsletterConfig) -> Self {
        let feeds = config
            .feeds
            .iter()
            .cloned()
            .map(|feed_config| {
                let mut feed = Feed::from(feed_config);
                println!("Open feed {}", feed.feed_url);

                {
                    feed.fetch_posts().unwrap()
                };
                println!("Fetched feed posts");
                feed
            })
            .collect();

        let output_file = File::create(config.output_file).unwrap();
        println!("Open output file");
        Self {
            title: config.title,
            output_file,
            feeds,
        }
    }

    fn to_html(&mut self) -> String {
        let mut newsletter_text = String::new();

        for feed in self.feeds.iter_mut() {
            let feed_title = format!("<h3>{}</h3>", feed.name);
            newsletter_text.push_str(&feed_title);

            for post in feed.posts.iter_mut() {
                let post_text = post.to_html();

                newsletter_text.push_str(&post_text);
            }
        }

        newsletter_text
    }
}
#[derive(Debug)]
struct Feed {
    name: String,
    feed_url: String,
    regex_filter: Regex,
    posts: Vec<Post>,
}

impl Feed {
    fn from(config: FeedConfig) -> Self {
        let regex_filter = Regex::new(&config.regex_filter).unwrap();
        Self {
            name: config.name,
            feed_url: config.feed_url,
            regex_filter,
            posts: vec![],
        }
    }

    fn fetch_posts(&mut self) -> Result<(), Box<dyn Error>> {
        let feed_bytes = reqwest::blocking::get(&self.feed_url)?.bytes()?;

        let channel = rss::Channel::read_from(&feed_bytes[..])?;

        self.posts = channel
            .items()
            .iter()
            .map(Post::from)
            .filter(|post| {
                println!("Filtering {}", post.link);
                let week_ago =
                    post.publication_date > Utc::now() - Duration::seconds(SECONDS_IN_WEEK);
                let filter_words = !self.regex_filter.is_match(&post.title);
                week_ago & filter_words
            })
            .collect::<Vec<Post>>();

        Ok(())
    }
}

#[derive(Debug)]
struct Post {
    title: String,
    content: String,
    summary: String,
    publication_date: DateTime<Utc>,
    link: String,
}

impl Post {
    fn summarize(&mut self) {
        if self.summary.len() < 280 {
            self.summary = self.content[0..280].to_string()
        }
    }

    fn to_html(&mut self) -> String {
        self.summarize();
        format!(
            "<h4><a href={}>{}</a></h4><i>On {}: </i><p> {}</p>",
            self.link,
            self.title,
            self.publication_date.format("%A"),
            self.summary
        )
    }
}

impl From<&rss::Item> for Post {
    fn from(item: &rss::Item) -> Self {
        Self {
            title: item.title().map_or_else(String::new, String::from),
            link: item.link().map_or_else(String::new, String::from),
            content: item.content().map_or_else(String::new, |text| {
                let plain_text = html2text::from_read(&text.as_bytes()[..], text.len()).unwrap();
                let text_without_urls = URL_REGEX.replace_all(plain_text.as_str(), "").to_string();
                UNNECESSARY_SYMBOLS_REGEX
                    .replace_all(&text_without_urls.as_str(), "")
                    .to_string()
            }),
            publication_date: DateTime::parse_from_rfc2822(item.pub_date().unwrap_or_default())
                .unwrap_or_default()
                .with_timezone(&Utc),
            summary: String::new(),
        }
    }
}

async fn post_to_telegraph(newsletter: &mut Newsletter) -> telegraph_rs::Page {
    let telegraph = Telegraph::new(&newsletter.title).create().await.unwrap();
    println!("created telegraph object");
    let newsletter_text = newsletter.to_html();
    println!("created newsletter text");

    let newsletter_title = format!(
        "{} for {}",
        &newsletter.title,
        Utc::now().format("week %W (%e %B)")
    );
    println!("created newsletter title");

    let node_text = html_to_node(newsletter_text.as_str());
    println!("created newsletter nodes {node_text:?}");

    let page = telegraph
        .create_page(newsletter_title.as_str(), &node_text, false)
        .await
        .unwrap();
    println!("posted page");

    page
}
