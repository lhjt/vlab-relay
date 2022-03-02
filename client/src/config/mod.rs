use std::io::{Read, Write};

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Configuration {
    pub(crate) token: Option<String>,
    pub(crate) uri:   Option<String>,
}

impl Configuration {
    pub(crate) fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut config_path = dirs::config_dir().unwrap();
        config_path.push("vlab-relay-client");
        config_path.push("config.toml");

        let mut file = std::fs::File::create(config_path)?;
        let as_string = toml::to_string_pretty(self).unwrap();
        file.write_all(as_string.as_bytes()).unwrap();

        Ok(())
    }
}

pub(crate) fn get_config() -> Configuration {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("vlab-relay-client");
    config_path.push("config.toml");

    // If the config file doesn't exist, create it.
    if !config_path.exists() {
        println!(
            "{}",
            format!("Creating config file at {}", config_path.display()).green()
        );
        let config = Configuration {
            token: None,
            uri:   None,
        };

        // Create missing directories.
        if !config_path.parent().unwrap().exists() {
            std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        }

        let mut file = std::fs::File::create(config_path).unwrap();
        let as_string = toml::to_string_pretty(&config).unwrap();
        file.write_all(as_string.as_bytes()).unwrap();
        return config;
    }

    // If the config file exists, read it.
    let mut file = std::fs::File::open(config_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Configuration = toml::from_str(&contents).unwrap();

    config
}
