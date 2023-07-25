use std::{
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use log::{error, info};

use crate::{
    models::{api_comms::ConnectionRequest, config::Config},
    util::{api::APIClient, bootstrap, logging::Logger, watchdog},
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
    connection: ConnectionRequest,
}

impl Helmdall {
    pub async fn bootstrap(path: &str, logger: Logger) -> Helmdall {
        info!("bootstrapping the application...");
        let config_path = path.to_string() + "/config.yaml";
        let state = AppState::new(&config_path, &logger).await;
        let socket_config = state.api_client.get_socket_config().await.unwrap();

        let local_socket_path = path.to_string() + "/helmdall.sock";

        let app = Helmdall {
            path: path.to_string(),
            state,
            connection: socket_config,
        };
        info!("bootstrapping complete.");
        app
    }

    pub fn get_config(&self) -> &Config {
        &self.state.cfg
    }

    pub fn get_connection(&self) -> &ConnectionRequest {
        &self.connection
    }

    pub fn local_listen(local_socket_path: &str) {}
}
