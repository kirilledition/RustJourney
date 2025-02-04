use regex::Regex;

use crate::{
    config::{FeedConfig, TimeRange},
    errors::NewsletterError,
    Result,
};

use super::post::Post;

pub(crate) struct Unfetched;
pub(crate) struct Fetched {
    posts: Vec<Post>,
}

#[derive(Debug, Clone)]
pub(crate) struct Feed<S> {
    pub(crate) name: String,
    pub(crate) feed_url: String,
    pub(crate) regex_filter: Regex,
    pub(crate) state: S,
}

impl Feed<Unfetched> {
    pub(crate) fn new(config: FeedConfig) -> Result<Self> {
        let regex_filter = Regex::new(&config.regex_filter)?;
        Ok(Feed {
            name: config.name,
            feed_url: config.feed_url,
            regex_filter,
            state: Unfetched,
        })
    }

    pub(crate) async fn fetch(self, time_range: TimeRange) -> Result<Feed<Fetched>> {
        let feed_bytes = reqwest::get(&self.feed_url).await?.bytes().await?;
        let channel = rss::Channel::read_from(&feed_bytes[..])?;

        let posts = channel
            .items()
            .iter()
            .map(Post::from)
            .filter(|post| {
                let in_time_range = time_range.contains(post.publication_date);
                let filter_words = !self.regex_filter.is_match(&post.title);

                in_time_range && filter_words
            })
            .collect::<Vec<Post>>();

        Ok(Feed {
            name: self.name,
            feed_url: self.feed_url,
            regex_filter: self.regex_filter,
            state: Fetched { posts },
        })
    }
}

impl Feed<Fetched> {
    // Accessor for posts.
    pub(crate) fn posts(&self) -> &[Post] {
        let Fetched { posts } = &self.state;
        posts
    }
    #[allow(dead_code)]
    pub(crate) fn posts_mut(&mut self) -> &mut [Post] {
        let Fetched { posts } = &mut self.state;
        posts
    }
}

// Implement TryFrom for convenience, mapping a FeedConfig to an unfetched Feed.
impl TryFrom<FeedConfig> for Feed<Unfetched> {
    type Error = NewsletterError;

    fn try_from(value: FeedConfig) -> Result<Self, Self::Error> {
        Feed::new(value)
    }
}
