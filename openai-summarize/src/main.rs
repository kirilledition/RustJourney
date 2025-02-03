use std::error::Error;

use async_openai::{
    types::ChatCompletionRequestSystemMessageArgs, types::CreateChatCompletionRequestArgs, Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let text_to_summarize = String::from("To craft an efficient prompt for summarizing a piece of text into a concise, tweet-sized message (280 characters), while including the main takeaway and presenting it as a trailer, you can use the following structure: Prompt: Summarize the following text into a single tweet of no more than 280 characters. Focus on capturing the main takeaway and presenting it as a teaser or trailer to spark curiosity. Avoid excessive detail but ensure the core message is clear and engaging. Text: [Insert text here]Key Features of This Prompt:Character Limit: Specifies the 280-character constraint for Twitter.Main Takeaway: Ensures the summary highlights the most critical point.Trailer Style: Directs the tone to be intriguing, encouraging further exploration.Clarity and Brevity: Avoids unnecessary details while maintaining focus on the essence of the text.This format aligns with best practices for summarization prompts, such as providing clear instructions, defining output specifications, and tailoring tone and style to the desired audience126.");

    let summary = summarize(text_to_summarize).await?;
    println!("{}", summary);

    Ok(())
}

async fn summarize(text_to_summarize: String) -> Result<String, Box<dyn Error>> {
    let summarization_prompt = String::from("Summarize the following text into a single text of no more than 280 characters. Focus on capturing the main takeaway and presenting it as a teaser or trailer to spark curiosity. Avoid excessive detail but ensure the core message is clear and engaging. Text:");
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

    response.choices[0]
        .message
        .content
        .clone()
        .ok_or("error with message option".into())
}
