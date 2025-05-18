use std::error::Error;
use async_trait::async_trait;
use serde_json::{json, Value};
use crate::structured_output::traits::Solver;
use crate::structured_output::utils;

pub struct EventInfo {
    api_key: String,
    question: String
}
impl EventInfo {
    pub fn new(api_key: &str, equation: &str) -> Self {
        Self {
            api_key:api_key.to_string(),
            question: equation.to_string()
        }
    }
}

#[async_trait]
impl Solver for EventInfo {
    fn get_reasoning_schema(&self) -> Value {
        json!({
            "name": "event_info",
            "type": "json_schema",
            "description": "Provides a detailed information about the event",
            "strict": true,
            "schema": {
                "type": "object",
                "properties": {
                    "question": {
                        "type": "string",
                        "description": "The original question"
                    },
                    "people": {
                        "type": "array",
                        "items": {
                            "type": "string",
                            "description": "The name of the person"
                        },
                    },
                    "event_location": {
                        "type": "string",
                        "description": "The location at which event is occurring"
                    }
                },
                "required": ["question", "event_location", "people"],
                "additionalProperties": false
            }
        })
    }

    async fn execute(&self) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let payload = json!({
            "model": "gpt-4.1",
            "input": [
                {
                    "role": "system",
                    "content": "You are a helpful event info extractor. Extract the event info and provide a structured response in JSON format"
                },
                {
                    "role": "user",
                    "content": format!("What is the solution of {}?", &self.question)
                }
            ],
            "text": {
                "format": self.get_reasoning_schema()
            }
        });

        let response = utils::HTTP_CLIENT
            .post("https://api.openai.com/v1/responses")
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(response)
    }

    fn print_formatted_response(&self, response: &Value) {
        if let Some(text) = response.pointer("/output/0/content/0/text") {
            if let Ok(solution) = serde_json::from_str::<Value>(text.as_str().unwrap_or("")) {
                println!("\nQuestion: {}", solution["question"]);
                println!("\nPeople attending the event:");

                if let Some(people) = solution["people"].as_array() {
                    for person in people {
                        println!("Person {}:", person);
                    }
                }

                println!("\nEvent location: {}", solution["event_location"]);
            } else {
                println!("Raw Response:\n{}", text);
            }
        }
    }
}