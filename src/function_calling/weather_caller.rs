use std::error::Error;
use serde_json::{json, Value};
use crate::function_calling::traits::Caller;
use crate::utils::HTTP_CLIENT;

pub struct WeatherCaller{
    api_key: String
}

impl WeatherCaller{
    pub fn new(api_key: &str) -> Self{
        Self { api_key: api_key.to_string() }
    }
    pub fn get_weather(latitude: f32, longitude: f32) -> f32 {
        println!("Getting weather for {} and {}", latitude, longitude);
        25.0
    }
}

#[async_trait::async_trait]
impl Caller for WeatherCaller{
    fn get_tool_schema(&self) -> Value {
        json!({
            "name": "get_weather",
            "strict": true,
            "description": "Get the current weather for a given location",
            "type": "function",
            "parameters": {
                "type": "object",
                "properties": {
                    "latitude": {
                        "type": "number",
                        "description": "The latitude of the location"
                    },
                    "longitude": {
                        "type": "number",
                        "description": "The longitude of the location"
                    }
                },
                "required": ["latitude", "longitude"],
                "additionalProperties": false
            }
        })
    }

    async fn call_openai_with_function(&self, user_query: &str) -> Result<Value, Box<dyn Error + Send + Sync>>{
        let payload = json!({
            "model": "gpt-4.1",
            "input": [
                {
                    "role": "user",
                    "content": user_query
                }
            ],
            "tools": [self.get_tool_schema()],
            "tool_choice": "auto"
        });

        let response = HTTP_CLIENT
            .post("https://api.openai.com/v1/responses")
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response)
    }

    fn handle_function_call(&self, function_name: &str, arguments: Value) -> Result<Value, Box<dyn Error + Send + Sync>> {
        match function_name {
            "get_weather" => {
                let latitude = arguments["latitude"].as_f64().unwrap() as f32;
                let longitude = arguments["longitude"].as_f64().unwrap() as f32;
                let temperature = Self::get_weather(latitude, longitude);
                Ok(json!({
                    "temperature": temperature
                }))
            }
            _ => Err("Unknown function".into())
        }
    }

    async fn process_function_calling(&self, user_query: &str) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let response = self.call_openai_with_function(user_query).await?;
        if let Some(text) = response.pointer("/output/0"){
            let arguments = serde_json::from_str(text["arguments"].as_str().unwrap())?;
            let function_name = text.get("name")
                .and_then(|n| n.as_str())
                .ok_or("Missing or invalid function name in output")?;
            let resp =  self.handle_function_call(function_name, arguments)?;

            Ok(resp)
        }else{
            Err("No response".into())
        }
    }
}