use log::{error, info};

use crate::{
    models::config::Config,
    util::{api::APIClient, bootstrap, logging::Logger, socket_watchdog},
};

struct AppState {
    cfg: Config,
    api_client: APIClient,
}

impl AppState {
    async fn new(cfg_path: &str, logger: &Logger) -> AppState {
        let (cfg, api_client) = bootstrap::bootstrap(cfg_path, logger).await;
        AppState { cfg, api_client }
    }
}

pub struct Helmdall {
    path: String,
    state: AppState,
}

impl Helmdall {
    pub async fn bootstrap(path: &str, logger: Logger) -> Helmdall {
        info!("bootstrapping the application...");
        let config_path = path.to_string() + "/config.yaml";
        let state = AppState::new(&config_path, &logger).await;
        let socket_config = state.api_client.get_socket_config().await.unwrap();
        let app = Helmdall {
            path: path.to_string(),
            state,
        };
        info!("bootstrapping complete.");
        app
    }
}
