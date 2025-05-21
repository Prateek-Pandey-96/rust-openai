use crate::function_calling::traits::Caller;
use crate::function_calling::weather_caller::WeatherCaller;
use crate::utils::{CallerType};

pub struct CallerProvider;

impl CallerProvider {
    pub fn get_caller(caller_type: CallerType, api_key: &str) -> Box<dyn Caller + Send + Sync> {
        match caller_type {
            CallerType::Weather => Box::new(WeatherCaller::new(api_key)),
            // Add more cases as new question types are added
        }
    }
}