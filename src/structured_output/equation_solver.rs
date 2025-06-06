use std::error::Error;
use async_trait::async_trait;
use serde_json::{json, Value};
use crate::structured_output::traits::{Solver};
use crate::utils;

pub struct EquationSolver {
    api_key: String,
    equation: String
}
impl EquationSolver{
    pub fn new(api_key: &str, equation: &str) -> Self {
        Self { 
            api_key:api_key.to_string(),
            equation: equation.to_string()
        }
    }
}

#[async_trait]
impl Solver for EquationSolver {
    fn get_reasoning_schema(&self) -> Value {
        json!({
            "name": "math_solution",
            "type": "json_schema",
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
                            "required": ["step_number", "explanation", "equation"],
                            "additionalProperties": false
                        }
                    },
                    "final_answer": {
                        "type": "string",
                        "description": "The final answer to the equation"
                    }
                },
                "required": ["equation", "steps", "final_answer"],
                "additionalProperties": false
            }
        })
    }
    async fn execute(&self) -> Result<Value, Box<dyn Error + Send + Sync>>{
        let payload = json!({
            "model": "gpt-4.1",
            "input": [
                {
                    "role": "system",
                    "content": "You are a helpful math tutor. Solve the equation step by step and provide a structured response in JSON format"
                },
                {
                    "role": "user",
                    "content": format!("What is the solution of {}?", &self.equation)
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
                println!("Raw Response:\n{}", text);
            }
        }
    }
}




