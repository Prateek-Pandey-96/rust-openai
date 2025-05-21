use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Caller: Send + Sync {
    fn get_tool_schema(&self) -> Value;
    async fn call_openai_with_function(&self, user_query: &str) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
    fn handle_function_call(&self, function_name: &str, arguments: Value) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
    async fn process_function_calling(&self, user_query: &str) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
}
