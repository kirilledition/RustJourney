use config::Config;
use feed::FeedsConfig;
use serde::Deserialize;
mod feed;
mod model;
mod timerange;

pub(crate) use feed::FeedConfig;
pub(crate) use model::ModelConfig;
pub(crate) use timerange::TimeRange;

use crate::errors::ConfigError;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct AppConfig {
    pub(crate) title: String,
    pub(crate) output_file: String,
    pub(crate) feeds: FeedsConfig,
    pub(crate) model: ModelConfig,
}

impl AppConfig {
    pub(crate) fn new<S: AsRef<str>>(basename: S) -> Result<Self, ConfigError> {
        let cfg = Config::builder()
            .add_source(config::File::with_name(basename.as_ref()))
            .build()
            .map_err(|e| ConfigError::Read { source: e })?;

        cfg.try_deserialize()
            .map_err(|e| ConfigError::Deserialize { source: e })
    }
}
