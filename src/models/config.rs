use log::LevelFilter;
use log4rs::config;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use crate::models::audit::ScriptGroup;

use super::logger::LoggerConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    socket_key: String,
    client_name: String,
    api_uri: String,
    audit_options: Vec<ScriptGroup>,
    logger: LoggerConfig,
    #[serde(default = "default_plugins_path")]
    plugin_path: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            socket_key: "CLIENT_ORIG-dummy".to_string(),
            client_name: "arch-server".to_string(),
            api_uri: "https://3c41ca28-7235-4fb0-8d1f-17947f8a053b.mock.pstmn.io".to_string(),
            audit_options: Vec::<ScriptGroup>::new(),
            logger: LoggerConfig::new(),
            plugin_path: default_plugins_path(),
        }
    }

    pub fn get_socket_key(&self) -> &str {
        &self.socket_key
    }

    pub fn get_client_name(&self) -> &str {
        &self.client_name
    }

    pub fn get_api_uri(&self) -> &str {
        &self.api_uri
    }

    pub fn get_logger_config(&self) -> &LoggerConfig {
        &self.logger
    }

    pub fn get_audit_options(&self) -> &Vec<ScriptGroup> {
        &self.audit_options
    }

    pub fn get_plugin_path(&self) -> &str {
        &self.plugin_path
    }

    pub fn load_from_file(file_path: &str) -> Result<Config, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        let cfg: Config = serde_yaml::from_str(&contents)?;
        Ok(cfg)
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(file_path)?;
        let contents = serde_yaml::to_string(&self)?;
        let bytes = file.write(contents.as_bytes()).unwrap();

        Ok(())
    }
}

fn default_plugins_path() -> String {
    std::env::var("HOME").unwrap_or(".".to_string()) + "/plugins"
}
