use chrono::{Duration, Utc};
use regex::Regex;
use serde::Deserialize;

use crate::{errors::NewsletterError, SECONDS_IN_WEEK};

use super::post::Post;

#[derive(Debug)]
pub struct Feed {
    pub name: String,
    pub feed_url: String,
    pub regex_filter: Regex,
    pub posts: Vec<Post>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct FeedConfig {
    name: String,
    feed_url: String,
    regex_filter: String,
}

impl Feed {
    pub fn from(config: FeedConfig) -> Self {
        let regex_filter = Regex::new(&config.regex_filter).unwrap();
        Self {
            name: config.name,
            feed_url: config.feed_url,
            regex_filter,
            posts: vec![],
        }
    }

    pub async fn fetch_posts(&mut self) -> Result<(), NewsletterError> {
        let feed_bytes = reqwest::get(&self.feed_url).await?.bytes().await?;

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
