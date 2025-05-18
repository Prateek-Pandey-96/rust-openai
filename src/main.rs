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
    let solver = SolverProvider::get_solver(
        QuestionType::EquationSolver,
        &openai_key,
        "8x+5=-27+88"
    );

    // Execute the solver and get the response
    let response = solver.execute().await?;
    println!("{:?}", response);
    
    // Print the formatted response
    solver.print_formatted_response(&response);

    Ok(())
}
