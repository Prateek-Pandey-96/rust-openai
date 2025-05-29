use serde_json::{json, Value};

pub struct OpenAiClient {
    client: reqwest::Client,
    api_key: String,
}

impl OpenAiClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: String::from(api_key),
        }
    }

    pub async fn get_response(&self, messages: &[Value], tools: &[Value]) -> Result<Value, Box<dyn std::error::Error>> {
        let payload = json!({
            "model": "gpt-4.1",
            "messages": messages,
            "tools": tools,
        });

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        let response = response.json().await?;

        Ok(response)
    }
}