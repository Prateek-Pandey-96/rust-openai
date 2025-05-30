use serde_json::Value;
use crate::fn_callng_wt_tools::functions::{get_addition, get_event_info, get_user_info, get_weather};

pub fn executor(name: &str, args: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parsed_args: Value = serde_json::from_str(args)?;

    match name {
        "get_weather" => {
            let latitude = parsed_args["latitude"].as_f64().unwrap_or(0.0);
            let longitude = parsed_args["longitude"].as_f64().unwrap_or(0.0);
            Ok(get_weather(latitude, longitude))
        }
        "get_addition" => {
            let num1 = parsed_args["num1"].as_i64().unwrap_or(0);
            let num2 = parsed_args["num2"].as_i64().unwrap_or(0);
            Ok(get_addition(num1, num2).to_string())
        }
        "get_user_info" => {
            let name = parsed_args["name"].as_str().unwrap_or("").to_string();
            let designation = parsed_args["designation"].as_str().unwrap_or("").to_string();
            Ok(get_user_info(name, designation))
        }
        "get_event_info" => {
            let name = parsed_args["name"].as_str().unwrap_or("").to_string();
            let location = parsed_args["location"].as_str().unwrap_or("").to_string();
            Ok(get_event_info(name, location))
        }
        _ => Err(format!("Unknown function: {}", name).into()),
    }
}