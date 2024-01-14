// Get environment variable
pub fn get_env(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val.to_lowercase(),
        Err(_e) => default.to_string().to_lowercase(),
    }
}
