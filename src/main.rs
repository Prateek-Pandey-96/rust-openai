use std::env;
use dotenv::dotenv;
use structured_output::utils::QuestionType;
use crate::structured_output::provider::SolverProvider;

mod structured_output;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match dotenv().ok() {
        Some(_) => eprintln!(".env file loaded successfully"),
        None => eprintln!(".env file not loaded"),
    }
    let openai_key = env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "OPENAI_API_KEY".to_string());

    // Get the appropriate solver for the question type
    let solver = SolverProvider::get_solver(QuestionType::EquationSolver, &openai_key, "8x+5=53");
    let response = solver.execute().await?;
    // println!("{:?}", response);
    solver.print_formatted_response(&response);

    let solver = SolverProvider::get_solver(QuestionType::EventInfo, &openai_key, "Ram and Shyam are going to the party in Mumbai.");
    let response = solver.execute().await?;
    // println!("{:?}", response);
    solver.print_formatted_response(&response);
    Ok(())
}
