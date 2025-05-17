use reqwest::Client;
use serde_json::{json, Value};
use once_cell::sync::Lazy;

pub static LLM_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .pool_max_idle_per_host(100)
        .build()
        .expect("Failed to build reqwest client!")
});

pub fn get_reasoning_schema() -> Value {
    json!({
        "name": "math_solution",
        "description": "Provides a structured solution for a math equation",
        "strict": true,
        "schema": {
            "type": "object",
            "properties": {
                "equation": {
                    "type": "string",
                    "description": "The original equation"
                },
                "steps": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "step_number": {
                                "type": "integer",
                                "description": "The step number in the solution"
                            },
                            "explanation": {
                                "type": "string",
                                "description": "Explanation of what is being done in this step"
                            },
                            "equation": {
                                "type": "string",
                                "description": "The equation at this step"
                            }
                        },
                        "required": ["step_number", "explanation", "equation"]
                    }
                },
                "final_answer": {
                    "type": "string",
                    "description": "The final answer to the equation"
                }
            },
            "required": ["equation", "steps", "final_answer"]
        }
    })
}

pub async fn execute(api_key: &str) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let payload = json!({
        "model": "gpt-4.1",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful math tutor. Solve the equation step by step and provide a structured response in JSON format according to the schema. Do not use LaTeX notation, use plain text for equations."
            },
            {
                "role": "user",
                "content": "What is the solution of 8x+5=-27+88? Provide the response in the following JSON format:\n{\n  \"equation\": \"-8x + 5 = -27\",\n  \"steps\": [\n    {\n      \"step_number\": 1,\n      \"explanation\": \"Subtract 5 from both sides\",\n      \"equation\": \"-8x = -32\"\n    },\n    {\n      \"step_number\": 2,\n      \"explanation\": \"Divide both sides by -8\",\n      \"equation\": \"x = 4\"\n    }\n  ],\n  \"final_answer\": \"x = 4\"\n}"
            }
        ],
        "format": get_reasoning_schema()
    });

    let response = LLM_HTTP_CLIENT
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(response)
}

pub fn print_formatted_response(response: &Value) {
    if let Some(choices) = response.get("choices").and_then(|c| c.as_array()) {
        if let Some(first_choice) = choices.first() {
            if let Some(message) = first_choice.get("message") {
                if let Some(content) = message.get("content") {
                    if let Ok(solution) = serde_json::from_str::<Value>(content.as_str().unwrap_or("")) {
                        println!("\nEquation: {}", solution["equation"]);
                        println!("\nSolution Steps:");

                        if let Some(steps) = solution["steps"].as_array() {
                            for step in steps {
                                println!("\nStep {}:", step["step_number"]);
                                println!("Explanation: {}", step["explanation"]);
                                println!("Equation: {}", step["equation"]);
                            }
                        }

                        println!("\nFinal Answer: {}", solution["final_answer"]);
                    } else {
                        println!("Raw Response:\n{}", content);
                    }
                }
            }
        }
    }
}