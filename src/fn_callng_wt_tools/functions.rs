pub fn get_weather(latitude: f64, longitude: f64) -> String {
    println!("latitude: {latitude}, longitude: {longitude}");
    String::from("The temperature is 24°C")
}

pub fn get_addition(num1: i64, num2: i64) -> i64 {
    num1 + num2
}