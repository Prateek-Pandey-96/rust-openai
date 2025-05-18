use crate::structured_output::traits::Solver;
use crate::structured_output::equation_solver::EquationSolver;
use crate::structured_output::event_info::EventInfo;
use super::utils::QuestionType;

pub struct SolverProvider;

impl SolverProvider {
    pub fn get_solver(question_type: QuestionType, api_key: &str, question: &str) -> Box<dyn Solver + Send + Sync> {
        match question_type {
            QuestionType::EquationSolver => Box::new(EquationSolver::new(api_key, question)),
            QuestionType::EventInfo => Box::new(EventInfo::new(api_key, question)),
            // Add more cases as new question types are added
        }
    }
} 