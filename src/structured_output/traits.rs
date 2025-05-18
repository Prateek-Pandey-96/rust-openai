use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Solver: Send + Sync {
    fn get_reasoning_schema(&self) -> Value;
    async fn execute(&self) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
    fn print_formatted_response(&self, response: &Value);
}
