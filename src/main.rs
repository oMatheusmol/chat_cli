use clap::{App, Arg};
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ChatCompletion {
    id: String,
    object: String,
    created: i64,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Usage {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("CHAT CLI")
        .version("0.1.0")
        .author("Matheus Mol <matheusmol@hotmail.com>")
        .about("CLI to have a conversation with CHATGPT")
        .arg(
            Arg::with_name("question")
                .required(true)
                .takes_value(true)
                .help("question"),
        )
        .get_matches();
    
    let question = matches.value_of("question").unwrap();
    request(&question).await?;


    Ok(())
}

async fn request(question: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let api_key = env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY environment variable not found");
    let url = "https://api.openai.com/v1/chat/completions";
    let body = format!(
        r#"{{
            "model": "gpt-3.5-turbo",
            "messages": [{{"role": "user", "content": "{}"}}]
        }}"#,
        question.replace("\"", "\\\"")
    );
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body)
        .send()
        .await?;
    let text = response.text().await?;
    let chat_completion: ChatCompletion = serde_json::from_str(&text).unwrap();
    let first_message = &chat_completion.choices[0].message;
    let content = &first_message.content;
    println!("{}", content);

    Ok(text)
}
