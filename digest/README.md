# Digest

Digest project is going to be a tool for collecting important news and blog posts from selected bloggers

## MVP

Collect posts from selected substacks, summarize, collect to markdown file

## Next steps

- [x] remove images and links from post body (maybe need just plain text from html)
- [x] format summary with mock summary function
- [x] issue markdown version of digest
- [ ] multi source version with post iterator


## Ideas

Config file with specified sources. For each source you specify regex for title, that will be used to filter posts, can construct regex including and excluding specific terms. For each source specify name, that will be displayed as subsection in digest.


## Code scheme

main
    ask for config path
    read toml config
    create newsletter structure
    write structure to file


NewsletterStructure
    feeds_structure_list

    to_markdown

FeedStructure
    name
    feed_url
    posts_vector

    fn fetch_posts

PostStructure
    title
    content
    summarized_content
    publication_date
    link

    fn summarize
    fn to_markdown
