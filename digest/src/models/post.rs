use chrono::{DateTime, Utc};

use crate::{
    config::ModelConfig, errors::NewsletterError, Result, UNNECESSARY_SYMBOLS_REGEX, URL_REGEX,
};

use async_openai::{
    types::ChatCompletionRequestSystemMessageArgs, types::CreateChatCompletionRequestArgs, Client,
};

#[derive(Debug)]
pub(crate) struct Post {
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) publication_date: DateTime<Utc>,
    pub(crate) link: String,
}

impl Post {
    async fn summarize(&self, config: &ModelConfig) -> Result<String> {
        let prompt = format!("{} {}", config.prompt, self.content);

        let client = Client::new();

        // single
        let request = CreateChatCompletionRequestArgs::default()
            .model(config.name.as_str())
            .messages([ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt)
                .build()?
                .into()])
            .max_tokens(280_u32)
            .build()?;

        let response = client.chat().create(request).await?;

        response
            .choices
            .first()
            .and_then(|m| m.message.content.clone())
            .ok_or(NewsletterError::EmptyResponse)
    }

    pub(crate) async fn to_html(&self, config: &ModelConfig) -> String {
        let mut html = format!(
            "<h4><a href={}>{}</a></h4><i>On {}: </i>",
            self.link,
            self.title,
            self.publication_date.format("%A"),
        );
        if let Ok(summary) = self.summarize(config).await {
            html.push_str(format!("<p> {summary}</p>").as_str());
        }
        html
    }
}

impl From<&rss::Item> for Post {
    fn from(item: &rss::Item) -> Self {
        Self {
            title: item.title().map_or_else(String::new, String::from),
            link: item.link().map_or_else(String::new, String::from),
            content: item.content().map_or_else(String::new, |text| {
                let plain_text = html2text::from_read(text.as_bytes(), text.len()).unwrap();
                let text_without_urls = URL_REGEX.replace_all(plain_text.as_str(), "").to_string();
                UNNECESSARY_SYMBOLS_REGEX
                    .replace_all(text_without_urls.as_str(), "")
                    .to_string()
            }),
            publication_date: DateTime::parse_from_rfc2822(item.pub_date().unwrap_or_default())
                .unwrap_or_default()
                .with_timezone(&Utc),
        }
    }
}
