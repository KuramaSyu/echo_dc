use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use serenity::model::webhook;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub webhooks: HashMap<String, String>,
}

impl Config {
    pub fn new(webhooks: HashMap<String, String>) -> Self {
        Config { webhooks }
    }

    pub fn get_webhook(&self, name: &str) -> Option<String> {
        return self.webhooks.get(name).cloned();
    }

    fn default_path() -> String {
        return "/etc/echo_dc/config.toml".into();
    }
    pub fn from_etc() -> Result<Self, Box<dyn std::error::Error>> {
        let config = std::fs::read_to_string(Config::default_path())?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }

    pub fn to_etc(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::create(Config::default_path())?;
        let string = toml::to_string_pretty(self)?;
        file.write_all(string.as_bytes())?;
        Ok(())
    }

    pub fn template() -> Self {
        let mut webhooks = HashMap::new();
        webhooks.insert(
            "default_webhook".to_string(),
            "https://discord.com/api/webhooks/133742013374206969/only-this-part-here".to_string(),
        );
        Config { webhooks: webhooks }
    }
}
