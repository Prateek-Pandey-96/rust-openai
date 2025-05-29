use serde_json::Value;
use crate::fn_callng_wt_tools::functions::{get_addition, get_weather};

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
        _ => Err(format!("Unknown function: {}", name).into()),
    }
}