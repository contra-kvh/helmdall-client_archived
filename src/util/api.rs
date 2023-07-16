use std::error::Error;

use log::{debug, error, info};
use reqwest::{header, Client};
use std::thread::sleep;

use crate::models::{
    api_comms::{ConnectionRequest, PingRequest},
    config::Config,
};

pub struct APIClient {
    client: Client,
    api_uri: String,
    provisioned_socket: Option<String>,
}

impl APIClient {
    pub fn new() -> APIClient {
        APIClient {
            client: reqwest::Client::new(),
            api_uri: String::new(),
            provisioned_socket: None,
        }
    }

    pub async fn bootstrap(&mut self, app_config: &Config) {
        info!("bootstrapping API client...");

        debug!("creating the header map...");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("helmdall-client"),
        );
        let mut auth_header = header::HeaderValue::from_str(app_config.get_socket_key()).unwrap();
        auth_header.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_header);
        debug!("header map created successfully.");

        debug!("building the client...");
        let builder = reqwest::Client::builder().default_headers(headers);
        self.client = builder.build().unwrap();
        info!("client built successfully.");

        self.api_uri = app_config.get_api_uri().to_string();

        self.validate_connection().await;
        self.provisioned_socket = Some(app_config.get_socket_key().to_string());
    }

    async fn get_ping(&self) -> Result<PingRequest, Box<dyn Error>> {
        info!(
            "trying to ping configured API server at: {}...",
            self.api_uri
        );
        let res = self
            .client
            .get(format!("{}/ping", self.api_uri))
            .send()
            .await?;
        debug!("ping response: {res:#?}");
        let ping_request: PingRequest = res.json().await?;
        if ping_request.status == 200 {
            info!("API server is up and running.");
            Ok(ping_request)
        } else {
            error!("API server is not up and running.");
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                ping_request.message,
            )))
        }
    }

    async fn validate_connection(&self) {
        let mut timeout_s = vec![60, 30, 15, 15];

        while let Err(e) = self.get_ping().await {
            let sleep_time = timeout_s.pop().unwrap_or(120);
            error!("failed to ping the API server:\n{e:#?}");
            info!("trying again in {} seconds...", sleep_time);
            sleep(tokio::time::Duration::from_secs(sleep_time));
        }
    }

    async fn get_connection(&self) -> Result<ConnectionRequest, Box<dyn Error>> {
        info!("getting connection information from: {}...", self.api_uri);
        let res = self
            .client
            .get(format!("{}/get_connection/", self.api_uri))
            .send()
            .await?;
        let connection_request: ConnectionRequest = res.json().await?;
        Ok(connection_request)
    }
}
