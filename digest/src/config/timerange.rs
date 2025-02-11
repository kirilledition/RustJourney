use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::FeedsConfig;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum TimeRangeConfig {
    Named(String),
    Explicit {
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum TimeRange {
    Since(DateTime<Utc>),
    Range {
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    },
    AllTime,
}

impl TimeRange {
    fn from_config(config: TimeRangeConfig) -> Option<Self> {
        let now = Utc::now();
        // TODO: Look into https://github.com/uutils/parse_datetime/blob/main/src/parse_relative_time.rs
        match config {
            TimeRangeConfig::Named(name) => match name.to_lowercase().as_str() {
                "all" => Some(TimeRange::AllTime),
                "week" => Some(TimeRange::Since(now - chrono::Duration::weeks(1))),
                "month" => Some(TimeRange::Since(now - chrono::Duration::days(30))),
                "year" => Some(TimeRange::Since(now - chrono::Duration::days(365))),
                _ => None, // Unknown variant
            },
            TimeRangeConfig::Explicit { from, to } => Some(TimeRange::Range { from, to }),
        }
    }

    pub(crate) fn parse_feeds_config(feeds_config: &FeedsConfig) -> Option<TimeRange> {
        match feeds_config {
            FeedsConfig {
                date_from: Some(from_str),
                date_to: Some(to_str),
                ..
            } => {
                let from = from_str.parse::<DateTime<Utc>>().ok()?;
                let to = to_str.parse::<DateTime<Utc>>().ok()?;
                Some(TimeRange::Range { from, to })
            }
            // If only date_from is provided, use it as the start.
            FeedsConfig {
                date_from: Some(from_str),
                ..
            } => {
                let from = from_str.parse::<DateTime<Utc>>().ok()?;
                Some(TimeRange::Since(from))
            }
            // Otherwise, if a relative period is provided, convert that.
            FeedsConfig {
                period: Some(period),
                ..
            } => {
                let cfg = TimeRangeConfig::Named(period.clone());
                TimeRange::from_config(cfg)
            }
            _ => None,
        }
    }
}

impl TimeRange {
    /// Returns true if `date` is within the time range.
    pub(crate) fn contains(&self, date: DateTime<Utc>) -> bool {
        match self {
            TimeRange::Since(since) => date > *since,
            TimeRange::Range { from, to } => date >= *from && date <= *to,
            TimeRange::AllTime => true,
        }
    }
}
