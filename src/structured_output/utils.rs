use once_cell::sync::Lazy;
use reqwest::Client;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestionType {
    EquationSolver,
    EventInfo
}

pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .pool_max_idle_per_host(100)
        .build()
        .expect("Failed to build reqwest client!")
});