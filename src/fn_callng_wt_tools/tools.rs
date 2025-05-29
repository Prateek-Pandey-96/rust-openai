use serde_json::{json, Value};

pub fn get_weather_tool() -> Value {
    json!({
        "type": "function",
        "function":{
            "name": "get_weather",
            "description": "Return the weather for a city given its latitude and longitude",
            "parameters": {
                "type":"object",
                "properties":{
                    "latitude":{
                        "type":"number",
                        "description":"The latitude of the city"   
                    },
                    "longitude":{
                        "type":"number",
                        "description":"The longitude of the city"   
                    }
                }
            },
            "required": ["latitude", "longitude"]       
        }
    })
}

pub fn get_addition_tool() -> Value {
    json!({
        "type": "function",
        "function":{
            "name": "get_addition",
            "description": "Return the addition of two numbers",
            "parameters": {
                "type":"object",
                "properties":{
                    "num1":{
                        "type":"number",
                        "description":"The first number"   
                    },
                    "num1":{
                        "type":"number",
                        "description":"The second number"   
                    }
                }
            },
            "required": ["num1", "num2"]      
        }
    })
}