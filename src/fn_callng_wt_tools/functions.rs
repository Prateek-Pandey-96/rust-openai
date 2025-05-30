pub fn get_weather(latitude: f64, longitude: f64) -> String {
    println!("latitude: {latitude}, longitude: {longitude}");
    String::from("The temperature is 24°C")
}

pub fn get_addition(num1: i64, num2: i64) -> i64 {
    num1 + num2
}

pub fn get_user_info(name: String, designation: String) -> String {
    format!("UserInfo -> Name: {name}, Designation: {designation}")
}

pub fn get_event_info(name: String, location: String) -> String {
    format!("EventInfo -> Name: {name}, Location: {location}")
}