use thiserror::Error;

use config::ConfigError as UConfigError;

#[derive(Debug, Error)]
pub(crate) enum NewsletterError {
    #[error(transparent)]
    WriteIO(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Rss(#[from] rss::Error),

    #[error(transparent)]
    OpenAI(#[from] async_openai::error::OpenAIError),

    #[error("Provider returned empty response")]
    EmptyResponse,

    #[error("Could not set global tracing subscriber")]
    TracingGlobalDefault(#[from] tracing::dispatcher::SetGlobalDefaultError),

    #[error(transparent)]
    Telegraph(#[from] telegraph_rs::Error),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    Config(#[from] ConfigError),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read configuration file: {source}")]
    Read {
        #[source]
        source: UConfigError,
    },

    #[error("Failed to deserialize configuration: {source}")]
    Deserialize {
        #[source]
        source: UConfigError,
    },

    #[error("Failed to parse date from config")]
    ParseDate,
}
