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