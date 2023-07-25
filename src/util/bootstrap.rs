use log::{debug, error, info};
use std::process;

use crate::models::config::Config;
use crate::util::api::APIClient;

use super::logging::Logger;

pub async fn bootstrap(cfg_path: &str, logger: &Logger) -> (Config, APIClient) {
    info!("initializing the config...");
    let cfg = initialize_config(cfg_path);
    debug!("config initialized: {cfg:#?}");

    info!("initializing the logger according to the given config...");
    logger.update_verbosity(cfg.get_logger_config());
    info!("logger initialized successfully.");

    info!("initializing the API client");
    let mut api_client = APIClient::new();
    api_client.bootstrap(&cfg).await;
    info!("api client initialized and bootstrapped successfully.");
    (cfg, api_client)
}

fn initialize_config(cfg_path: &str) -> Config {
    info!("initializing the config...");
    let cfg = match Config::load_from_file(cfg_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("failed to process the config file.\n{e:#?}");
            info!("attempting to save a dummy config at the config path...");
            let dummy_example = cfg_path.to_string() + ".example";
            write_cfg(Config::new(), dummy_example.as_str());
            info!("consider generating a config from your helmdall account :)");
            process::exit(2);
        }
    };
    cfg
}

fn write_cfg(cfg: Config, cfg_path: &str) {
    if let Err(e) = cfg.save_to_file(cfg_path) {
        error!("failed to save the config:\n{e:?}");
        process::exit(3);
    }
}
