# Digest

Digest project is going to be a tool for collecting important news and blog posts from selected bloggers

## MVP

Collect posts from selected substacks, summarize, collect to telegraph post and post to telegram channel.

## Next steps

- [x] remove images and links from post body (maybe need just plain text from html)
- [x] format summary with mock summary function
- [x] issue markdown version of digest
- [x] multi source version with post iterator
- [ ] implement telegraph api
- [ ] implement telegram posting
- [ ] implement openai api
- [ ] implement pubmed

## Ideas

Implement bionic reading. 


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
