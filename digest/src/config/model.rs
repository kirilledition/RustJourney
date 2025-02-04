use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ModelConfig {
    pub(crate) prompt: String,
    pub(crate) name: String,
}
