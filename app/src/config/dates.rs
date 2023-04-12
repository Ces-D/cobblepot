pub struct DateConfig {
    date_format: String,
}

impl DateConfig {
    pub fn new(date_format: Option<String>) -> DateConfig {
        DateConfig { date_format: date_format.unwrap_or_else(|| String::from("%Y-%m-%d")) }
    }
    pub fn format(&self) -> &str {
        &self.date_format
    }
}
