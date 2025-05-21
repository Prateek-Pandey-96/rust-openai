use std::env;
use dotenv::dotenv;
use crate::function_calling::provider::CallerProvider;
use crate::function_calling::traits::Caller;
use crate::function_calling::weather_caller;
use crate::function_calling::weather_caller::WeatherCaller;
use crate::structured_output::provider::SolverProvider;
use crate::utils::{CallerType, QuestionType};

mod structured_output;
mod function_calling;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match dotenv().ok() {
        Some(_) => eprintln!(".env file loaded successfully"),
        None => eprintln!(".env file not loaded"),
    }
    let openai_key = env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "OPENAI_API_KEY".to_string());

    // Get the appropriate solver for the question type
    // let solver = SolverProvider::get_solver(QuestionType::EquationSolver, &openai_key, "8x+5=53");
    // let response = solver.execute().await?;
    // println!("{:?}", response);
    // solver.print_formatted_response(&response);

    // let solver = SolverProvider::get_solver(QuestionType::EventInfo, &openai_key, "Ram and Shyam are going to the party in Mumbai.");
    // let response = solver.execute().await?;
    // println!("{:?}", response);
    // solver.print_formatted_response(&response);

    let caller = CallerProvider::get_caller(CallerType::Weather, &openai_key);
    let query = "What is the weather like in Bengaluru today?";
    let resp = caller.process_function_calling(query).await?;
    println!("{:?}", resp.get("temperature").unwrap().as_f64().unwrap());

    Ok(())
}
