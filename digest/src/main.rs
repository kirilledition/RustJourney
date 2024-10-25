use chrono::{DateTime, Duration, Utc};
use regex::Regex;
use serde_derive::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::sync;
// use telegraph_rs::{html_to_node, Telegraph};

const CONFIG_PATH: &str = "digest.toml";
const SECONDS_IN_WEEK: i64 = 604800;

static UNNECESSARY_SYMBOLS_REGEX: sync::LazyLock<Regex> =
    sync::LazyLock::new(|| Regex::new(r"[\[\]\*\n]").unwrap());

static URL_REGEX: sync::LazyLock<Regex> = sync::LazyLock::new(|| {
    Regex::new(r"(http|ftp|https):\/\/[\w\-_]+(\.[\w\-_]+)+([\w\-\.,@?^=%&amp;:/~\+#]*[\w\-\@?^=%&amp;/~\+#])?").unwrap()
});

fn main() {
    let mut config_file = File::open(CONFIG_PATH).unwrap();
    let mut config_string = String::new();
    let _ = config_file.read_to_string(&mut config_string);

    let config: NewsletterConfig = toml::from_str(&config_string).unwrap();
    println!("Read config");

    let mut newsletter = Newsletter::from(config);
    println!("Construct newsletter");
    // let newsletter_text = newsletter.to_markdown();
    let newsletter_text = newsletter.to_html();
    println!("Transform newsletter to html");

    newsletter
        .output_file
        .write(newsletter_text.as_bytes())
        .unwrap();
    println!("Wrote newsletter");
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
                feed.fetch_posts().unwrap();
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

    fn to_markdown(&mut self) -> String {
        let mut newsletter_text = String::new();

        let header = format!(
            "# {} for {}\n",
            self.title,
            Utc::now().format("**week %W** (%e %B)")
        );

        newsletter_text.push_str(&header);

        for feed in self.feeds.iter_mut() {
            let feed_title = format!("## {}\n", feed.name);
            newsletter_text.push_str(&feed_title);

            for post in feed.posts.iter_mut() {
                let post_text = post.to_markdown();

                newsletter_text.push_str(&post_text);
            }
        }

        newsletter_text
    }

    fn to_html(&mut self) -> String {
        let mut newsletter_text = String::new();

        let header = format!(
            "<h1>{} for {}</h1>",
            self.title,
            Utc::now().format("week %W (%e %B)")
        );

        newsletter_text.push_str(&header);

        for feed in self.feeds.iter_mut() {
            let feed_title = format!("<h2>{}<h2>", feed.name);
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

    fn to_markdown(&mut self) -> String {
        self.summarize();
        format!(
            "### [{}]({})\n*On {}*\n\n{}\n",
            self.title,
            self.link,
            self.publication_date.format("%A"),
            self.summary
        )
    }

    fn to_html(&mut self) -> String {
        self.summarize();
        format!(
            "<h3> <a href={}>{}</a> </h3><i>On {}</i><p>{}<p>",
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
                let plain_text = html2text::from_read(&text.as_bytes()[..], text.len());
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
