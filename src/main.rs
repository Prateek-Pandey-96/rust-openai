use std::env;
use dotenv::dotenv;
use serde_json::Value;

mod structured_output;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenv().ok() {
        Some(_) => eprintln!(".env file loaded successfully"),
        None => eprintln!(".env file not loaded"),
    }
    let openai_key = env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "OPENAI_API_KEY".to_string());

    let response = structured_output::playground::execute(&openai_key).await.expect("Failed to execute");
    // println!("Raw Response:\n{:?}", response);
    // Extract and format the structured response
    structured_output::playground::print_formatted_response(&response);

    Ok(())
}
