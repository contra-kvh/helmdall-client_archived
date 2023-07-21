use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggerConfig {
    pub verbosity: Option<LevelFilter>,
    pub log_path: String,
}

impl LoggerConfig {
    pub fn new() -> LoggerConfig {
        LoggerConfig {
            verbosity: None,
            log_path: "/var/log/helmdall.log".to_string(),
        }
    }
}
