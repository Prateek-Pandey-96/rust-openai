use serde_json::json;
use crate::fn_callng_wt_tools::executor::executor;
use crate::fn_callng_wt_tools::open_ai_client::OpenAiClient;
use crate::fn_callng_wt_tools::tools::{get_addition_tool, get_event_info_tool, get_user_info_tool, get_weather_tool};

pub async fn fn_calling() -> Result<(), Box<dyn std::error::Error>> {

    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY environment variable not set");

    // Define available tools/functions
    let tools = [
        get_weather_tool(),
        get_addition_tool(),
        get_user_info_tool(),
        get_event_info_tool()
    ];

    let system_msg = json!({
        "role": "system",
        "content":"You are an expert assistant. Given a set of tools you know which tool to invoke!"
    });
    let user_msg = json!({
        "role": "user",
        "content": "What's the weather like in New York? \
        Prateek who is a software engineer is going to a party today in Mumbai?\
        What is 12 + 27?"
    });
    let messages = [
        system_msg, user_msg,
    ];

    let client = OpenAiClient::new(&api_key);
    match client.get_response(&messages, &tools).await{
        Ok(response) => {
            if let Some(tool_calls) = response.pointer("/choices/0/message/tool_calls"){
                for tool_call in tool_calls.as_array().unwrap(){
                    let name = tool_call.pointer("/function/name").unwrap().as_str().unwrap();
                    let arguments = tool_call.pointer("/function/arguments").unwrap().as_str().unwrap();
                    let result = executor(name, arguments);
                    match result {
                        Ok(r) => println!("{:?}", r),
                        Err(e) => println!("{:?}", e),
                    }
                }
            }

        },
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}