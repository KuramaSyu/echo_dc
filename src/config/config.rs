use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    webhook: String,
}

impl Config {
    pub fn new(webhook: String) -> Self {
        Config { webhook }
    }

    fn default_path() -> String {
        return "/etc/echo_dc/config.json".into();
    }
    pub fn from_etc() -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(file_path)?;
        let config: Config = serde_json::from_reader(file)?;
        Ok(config)
    }

    pub fn to_etc(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(file_path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn template() -> Self {
        Config {
            webhook: "https://discord.com/api/webhooks/133742013374206969/only-this-part-here"
                .to_string(),
        }
    }
}
