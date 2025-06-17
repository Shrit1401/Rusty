use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String, //me or AI
    content: String,
}

#[derive(Debug, Serialize)]
struct UserMessage {
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct ManyResponses {
    choices: Vec<SingleResponse>,
}

#[derive(Debug, Deserialize)]
struct SingleResponse {
    message: Message,
}

pub async fn ai_response(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let res = client
        .post("https://ai.hackclub.com/chat/completions")
        .json(&UserMessage {
            messages: vec![
                Message {
                    role: "system".to_string(),
                    // TODO: make it more dynamic
                    content: "You are Rusty, a helpful CLI assistant. Keep responses concise and avoid markdown formatting.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
            ],
        })
        .send()
        .await?
        .json::<ManyResponses>()
        .await?;

    Ok(res.choices[0].message.content.clone())
}

pub fn display_ai_response(response: &str) {
    println!("{}", "\nRusty Says:".green().bold());
    println!("{}", response.trim());
}
