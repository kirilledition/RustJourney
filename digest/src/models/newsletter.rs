use crate::config::{AppConfig, ModelConfig, TimeRange};

use super::feed::{Feed, Fetched, Unfetched};

#[derive(Debug)]
pub(crate) struct Newsletter<S> {
    pub(crate) title: String,
    pub(crate) feeds: Vec<Feed<S>>,
}

impl From<AppConfig> for Newsletter<Unfetched> {
    fn from(value: AppConfig) -> Self {
        let feeds = value
            .feeds
            .feeds
            .into_iter()
            .filter_map(|feed_config| {
                let feed = Feed::new(feed_config);
                feed.ok()
            })
            .collect();

        Self {
            title: value.title,
            feeds,
        }
    }
}

impl Newsletter<Unfetched> {
    pub(crate) async fn into_fetched(self, time_range: TimeRange) -> Newsletter<Fetched> {
        let Newsletter {
            title,
            feeds: old_feeds,
        } = self;

        // TODO: Rewrite this to a true async closure when 1.85 lands
        let feeds: Vec<_> = old_feeds
            .into_iter()
            .map(move |feed| async move { feed.fetch(time_range).await.ok() })
            .collect::<Vec<_>>();

        let feeds = futures::future::join_all(feeds)
            .await
            .into_iter()
            .flatten()
            .collect();

        Newsletter { title, feeds }
    }
}

impl Newsletter<Fetched> {
    pub(crate) async fn to_html(&self, config: &ModelConfig) -> String {
        let mut newsletter_text = String::new();

        for feed in self.feeds.iter() {
            let feed_title = format!("<h3>{}</h3>", feed.name);
            newsletter_text.push_str(&feed_title);

            for post in feed.posts().iter() {
                let post_text = post.to_html(config).await;

                newsletter_text.push_str(&post_text);
            }
        }

        newsletter_text
    }
}
