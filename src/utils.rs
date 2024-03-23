
use std::time::Duration;

use log::info;

pub fn empty_string() -> String {
    "".to_string()
}

pub fn empty_json() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

pub fn format_maybe_string(maybe_string: &Option<String>) -> &str {
    match maybe_string {
        Some(string) => string,
        None => "",
    }
}

pub fn format_url_path(path: &str) -> String {
    let base_url = std::env::var("BASE_URL").expect("BASE_URL must be set");
    format!("{}{}", base_url, path)
}

// Estrutura para armazenar os tempos de execução
#[derive(Debug)]
pub struct ExecutionTimer {
    timings: Vec<(String, Duration)>,
}

impl ExecutionTimer {
    pub fn new() -> Self {
        Self {
            timings: Vec::new(),
        }
    }

    pub fn add_timing(&mut self, name: String, duration: Duration) {
        self.timings.push((name, duration));
    }

    pub fn log_timings(&self) {
        let api_requests = self.timings.iter().filter(|(name, _)| name.starts_with("api_request"));
        let code_execution = self.timings.iter().filter(|(name, _)| name.starts_with("code_execution"));
        let mut api_requests_total = Duration::new(0, 0);
        let mut code_execution_total = Duration::new(0, 0);

        for (_name, duration) in api_requests {
            api_requests_total += *duration;
        }

        for (_name, duration) in code_execution {
            code_execution_total += *duration;
        }

        let code_execution_time = code_execution_total  - api_requests_total;

        info!("Total time spent on API requests: {:.2} seconds", api_requests_total.as_secs_f64());
        info!("Total time spent on code execution: {:.2} seconds", code_execution_time.as_secs_f64());
        info!("Total time spent: {:.2} seconds", code_execution_total.as_secs_f64());
    }
}