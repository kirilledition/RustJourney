use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct FeedsConfig {
    pub(crate) feeds: Vec<FeedConfig>,
    // For relative durations like "week", "month", "all"
    pub(crate) period: Option<String>,
    pub(crate) date_from: Option<String>,
    pub(crate) date_to: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct FeedConfig {
    pub(crate) name: String,
    pub(crate) feed_url: String,
    pub(crate) regex_filter: String,
}
