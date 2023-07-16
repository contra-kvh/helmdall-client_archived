use log::{debug, error, info};
use std::process;

use crate::models::config::Config;
use crate::util::{api::APIClient, logging::setup_logger};

pub async fn bootstrap(cfg_path: &str) -> (Config, APIClient) {
    let dispatch = fern::Dispatch::new();
    info!("starting the client...");
    let cfg = initialize_config(&cfg_path);
    debug!("config initialized: {cfg:#?}");

    info!("initializing the logger according to the given config...");
    setup_logger(dispatch, cfg.get_verbosity());

    info!("initializing the API client");
    let mut api_client = APIClient::new();
    api_client.bootstrap(&cfg).await;
    info!("api client initialized and bootstrapped successfully.");

    info!("initialization complete.");
    (cfg, api_client)
}

fn initialize_config(cfg_path: &str) -> Config {
    info!("initializing the config...");
    let cfg = match Config::load_from_file(&cfg_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("failed to process the config file.\n{e:#?}");
            info!("attempting to save a dummy config at the config path...");
            write_cfg(Config::new(), cfg_path);
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