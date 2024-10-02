use chrono::{DateTime, Utc};
use core::fmt;
use std::error;

const ASTRAL_CODEX_TEN_FEED: &str = "https://www.astralcodexten.com/feed";

fn main() {
    let post_collection = feed_to_post_collection(ASTRAL_CODEX_TEN_FEED).unwrap();

    post_collection.iter().for_each(|post| {
        let markdown_post = html2md::parse_html(&post.content);
        println!(
            "# title: {}\nlink: {}\ncontent: {}\npublication data: {}",
            post.title, post.link, markdown_post, post.publication_date,
        )
    });

    println!("Length ofpost collection is {}", post_collection.len())
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
        Self {
            title: item.title().map_or_else(String::new, String::from),
            link: item.link().map_or_else(String::new, String::from),
            content: item
                .content()
                .map_or_else(String::new, String::from)
                // .get(0..1000)
                // .unwrap_or_default()
                .to_string(),
            publication_date: DateTime::parse_from_rfc2822(item.pub_date().unwrap_or_default())
                .unwrap_or_default()
                .with_timezone(&Utc),
        }
    }
}

fn feed_to_post_collection(feed_url: &str) -> Result<Vec<Post>, Box<dyn error::Error>> {
    // vec![Post {}]
    let feed_bytes = reqwest::blocking::get(feed_url)?.bytes()?;
    let channel = rss::Channel::read_from(&feed_bytes[..])?;

    let post_collection = channel
        .items()
        .iter()
        .map(Post::from)
        .filter(|post| {
            let week_ago = DateTime::from_timestamp(1727280000, 0).unwrap();
            let date_condition = post.publication_date > week_ago;
            let filter_words = !post.title.contains("Open Thread");
            date_condition & filter_words
            // true
        })
        .collect::<Vec<Post>>();

    Ok(post_collection)
}
