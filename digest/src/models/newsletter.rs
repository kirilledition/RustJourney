use serde::Deserialize;
use std::fs::File;

use super::feed::{Feed, FeedConfig};

#[derive(Debug)]
pub struct Newsletter {
    pub title: String,
    pub output_file: File,
    pub feeds: Vec<Feed>,
}

#[derive(Debug, Deserialize)]
pub struct NewsletterConfig {
    title: String,
    output_file: String,
    feeds: Vec<FeedConfig>,
}

impl Newsletter {
    pub fn from(config: NewsletterConfig) -> Self {
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

    pub fn to_html(&mut self) -> String {
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
