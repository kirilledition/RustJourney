use chrono::{DateTime, Duration, Utc};
use core::fmt;
use regex::Regex;
use std::error;

const SECONDS_IN_WEEK: i64 = 604800;
const ASTRAL_CODEX_TEN_FEED: &str = "https://www.astralcodexten.com/feed";

fn main() {
    let post_collection = feed_to_post_collection(ASTRAL_CODEX_TEN_FEED).unwrap();

    let mut final_text = String::from("# Kirusha weekly digest\n## Scott Alexander\n");

    post_collection.iter().for_each(|post| {
        let post_summary = mock_summarize(post.content.clone());
        let post_text = format!(
            "### {}\n publication date: *{}* [**link**]({})\n\ncontent: {}\n\n",
            post.title, post.link, post.publication_date, post_summary,
        );

        final_text.push_str(&post_text);
    });
}

fn mock_summarize(text: String) -> String {
    text[0..280].to_string()
}

#[derive(Debug)]
struct Post {
    title: String,
    content: String,
    publication_date: DateTime<Utc>,
    link: String,
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "title: {}\nlink: {}\ncontent: {}\npublication date: {}",
            self.title, self.link, self.content, self.publication_date,
        )
    }
}

impl From<&rss::Item> for Post {
    fn from(item: &rss::Item) -> Self {
        // want to compile only once, probably singleton
        let brackets_regex = regex::Regex::new(r"[\[\]]").unwrap();
        let url_regex = regex::Regex::new(
            r"(http|ftp|https):\/\/[\w\-_]+(\.[\w\-_]+)+([\w\-\.,@?^=%&amp;:/~\+#]*[\w\-\@?^=%&amp;/~\+#])?",
        )
        .unwrap();
        Self {
            title: item.title().map_or_else(String::new, String::from),
            link: item.link().map_or_else(String::new, String::from),
            content: item.content().map_or_else(String::new, |text| {
                let plain_text = html2text::from_read(&text.as_bytes()[..], text.len());
                let text_without_urls = url_regex.replace_all(plain_text.as_str(), "").to_string();
                brackets_regex
                    .replace_all(&text_without_urls.as_str(), " ")
                    .to_string()
            }),
            publication_date: DateTime::parse_from_rfc2822(item.pub_date().unwrap_or_default())
                .unwrap_or_default()
                .with_timezone(&Utc),
        }
    }
}

fn feed_to_post_collection(feed_url: &str) -> Result<Vec<Post>, Box<dyn error::Error>> {
    let feed_bytes = reqwest::blocking::get(feed_url)?.bytes()?;
    let channel = rss::Channel::read_from(&feed_bytes[..])?;

    let filter_title = Regex::new(r"(?i)(open thread|classfields)").unwrap();

    let post_collection = channel
        .items()
        .iter()
        .map(Post::from)
        .filter(|post| {
            let week_ago = post.publication_date > Utc::now() - Duration::seconds(SECONDS_IN_WEEK);
            let filter_words = !filter_title.is_match(&post.title);
            week_ago & filter_words
        })
        .collect::<Vec<Post>>();

    Ok(post_collection)
}
