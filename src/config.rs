use std::env;

use crate::constants;

#[derive(Debug, Clone)]
pub struct ForwardAnnouncementsConfig {
    pub whatsapp_source_chat_id: String,
    pub signal_destination_chat_id: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub waha_base_url: String,
    pub waha_api_key: String,
    pub signal_base_url: String,
    pub forward_announcements_config: ForwardAnnouncementsConfig,
}

impl Default for ForwardAnnouncementsConfig {
    fn default() -> Self {
        Self {
            whatsapp_source_chat_id: constants::WHATSAPP_TEST_CHAT_ID.to_string(),
            signal_destination_chat_id: constants::SIGNAL_TEST_CHAT_ID.to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: "test_key".to_string(),
            waha_base_url: "http://localhost:3000".to_string(),
            waha_api_key: "test_key".to_string(),
            signal_base_url: "http://localhost:8080".to_string(),
            forward_announcements_config: ForwardAnnouncementsConfig::default(),
        }
    }
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
            forward_announcements_config: ForwardAnnouncementsConfig {
                whatsapp_source_chat_id: constants::WHATSAPP_TEST_CHAT_ID.to_string(),
                signal_destination_chat_id: constants::SIGNAL_TEST_CHAT_ID.to_string(),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.api_key, "test_key");
        assert_eq!(config.waha_base_url, "http://localhost:3000");
    }

    #[test]
    fn test_config_from_env() {
        unsafe {
            std::env::set_var("WAHA_API_KEY", "env_key");
            std::env::set_var("WAHA_BASE_URL", "http://env_url");
            std::env::set_var("SIGNAL_BASE_URL", "http://env_signal_url");
        }

        let config = Config::from_env().unwrap();
        assert_eq!(config.api_key, "env_key");
        assert_eq!(config.waha_base_url, "http://env_url");
        assert_eq!(config.signal_base_url, "http://env_signal_url");
    }
}
