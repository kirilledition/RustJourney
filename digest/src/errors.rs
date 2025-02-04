use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    SourceNotValid(#[from] std::io::Error),
    #[error("Could not parse toml")]
    TomlParseError(#[from] toml::de::Error),
}

#[derive(Debug, Error)]
pub enum NewsletterError {
    #[error(transparent)]
    WriteIOError(#[from] std::io::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    RSSError(#[from] rss::Error),

    #[error(transparent)]
    OpenAIError(#[from] async_openai::error::OpenAIError),

    #[error("Provider returned empty response")]
    EmptyResponseError,

    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}
