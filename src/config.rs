use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = env::var("WAHA_API_KEY")?;
        let base_url =
            env::var("WAHA_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

        Ok(Config { api_key, base_url })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        unsafe {
            std::env::set_var("WAHA_API_KEY", "test_key");
            std::env::set_var("WAHA_BASE_URL", "http://test_url");
        }

        let config = Config::from_env().unwrap();
        assert_eq!(config.api_key, "test_key");
        assert_eq!(config.base_url, "http://test_url");
    }
}
