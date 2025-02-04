use chrono::{DateTime, Utc};

use crate::{errors::NewsletterError, UNNECESSARY_SYMBOLS_REGEX, URL_REGEX};

use async_openai::{
    types::ChatCompletionRequestSystemMessageArgs, types::CreateChatCompletionRequestArgs, Client,
};

#[derive(Debug)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub summary: String,
    pub publication_date: DateTime<Utc>,
    pub link: String,
}

async fn summarize(text_to_summarize: String) -> Result<String, NewsletterError> {
    let summarization_prompt = String::from("Summarize the following text into a single text of no more than 280 characters. Focus on capturing the main takeaway and presenting it. Avoid excessive detail but ensure the core message is clear and engaging. Text:");
    let prompt = format!("{} {}", summarization_prompt, text_to_summarize);

    let client = Client::new();

    // single
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
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
        .map(|m| m.message.content.clone())
        .flatten()
        .ok_or(NewsletterError::EmptyResponseError)
}

impl Post {
    // fn summarize(&mut self) {
    //     if self.summary.len() < 280 {
    //         self.summary = self.content[0..280].to_string()
    //     }
    // }

    fn summarize(&mut self) {
        self.summary = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(summarize(self.content.clone()))
        })
        .unwrap_or_else(|_| String::from("Empty summary"));
    }

    pub fn to_html(&mut self) -> String {
        self.summarize();
        format!(
            "<h4><a href={}>{}</a></h4><i>On {}: </i><p> {}</p>",
            self.link,
            self.title,
            self.publication_date.format("%A"),
            self.summary
        )
    }
}

impl From<&rss::Item> for Post {
    fn from(item: &rss::Item) -> Self {
        Self {
            title: item.title().map_or_else(String::new, String::from),
            link: item.link().map_or_else(String::new, String::from),
            content: item.content().map_or_else(String::new, |text| {
                let plain_text = html2text::from_read(&text.as_bytes()[..], text.len()).unwrap();
                let text_without_urls = URL_REGEX.replace_all(plain_text.as_str(), "").to_string();
                UNNECESSARY_SYMBOLS_REGEX
                    .replace_all(&text_without_urls.as_str(), "")
                    .to_string()
            }),
            publication_date: DateTime::parse_from_rfc2822(item.pub_date().unwrap_or_default())
                .unwrap_or_default()
                .with_timezone(&Utc),
            summary: String::new(),
        }
    }
}
