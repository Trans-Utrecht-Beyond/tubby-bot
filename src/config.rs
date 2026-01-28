use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub waha_base_url: String,
    pub waha_api_key: String,
    pub signal_base_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = env::var("WAHA_API_KEY")?;
        let waha_base_url = env::var("WAHA_BASE_URL")?;
        let waha_api_key = env::var("WAHA_API_KEY")?;
        let signal_base_url = env::var("SIGNAL_BASE_URL")?;

        Ok(Config {
            api_key,
            waha_base_url,
            waha_api_key,
            signal_base_url,
        })
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
            std::env::set_var("WAHA_API_KEY", "test_key");
            std::env::set_var("SIGNAL_BASE_URL", "http://signal_url");
        }

        let config = Config::from_env().unwrap();
        assert_eq!(config.api_key, "test_key");
        assert_eq!(config.waha_base_url, "http://test_url");
        assert_eq!(config.waha_api_key, "test_key");
        assert_eq!(config.signal_base_url, "http://signal_url");
    }
}
