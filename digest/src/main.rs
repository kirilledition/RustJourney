use serde_derive::Deserialize;
use std::fs;
use std::io::prelude::*;

const CONFIG_PATH: &str = "digest.toml";

fn main() {
    let mut config_file = fs::File::open(CONFIG_PATH).unwrap();
    let mut config_string = String::new();
    let _ = config_file.read_to_string(&mut config_string);

    let config: NewsletterConfig = toml::from_str(&config_string).unwrap();

    println!("{config:?}");

    let newsletter = Newsletter::from(config);

    println!("{newsletter:?}")
}

#[derive(Debug, Deserialize)]
struct NewsletterConfig {
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
    feeds: Vec<Feed>,
}

impl Newsletter {
    fn from(config: NewsletterConfig) -> Self {
        let feeds = config
            .feeds
            .iter()
            .cloned()
            .map(|feed_config| Feed::from(feed_config))
            .collect();
        Self { feeds }
    }
}
#[derive(Debug)]
struct Feed {
    name: String,
    feed_url: String,
    regex_filter: String,
    posts: Vec<Post>,
}

impl Feed {
    fn from(config: FeedConfig) -> Self {
        Self {
            name: config.name,
            feed_url: config.feed_url,
            regex_filter: config.regex_filter,
            posts: vec![],
        }
    }

    fn fetch_posts(&self) -> Vec<Post> {
        vec![]
    }
}

#[derive(Debug)]
struct Post {
    title: String,
    content: String,
    summary: String,
    publication_data: String,
    link: String,
}
