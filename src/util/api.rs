use std::{error::Error, process};

use log::{debug, error, info};
use reqwest::{header, Client};
use std::thread::sleep;

use crate::{
    errors::api::{APIError, APIErrorKind},
    models::{
        api_comms::{ConnectionRequest, PingRequest},
        config::Config,
    },
};

pub struct APIClient {
    client: Client,
    api_uri: String,
}

impl APIClient {
    pub fn new() -> APIClient {
        APIClient {
            client: reqwest::Client::new(),
            api_uri: String::new(),
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
        info!("validating connection with the api server...");
        self.validate_connection().await;
        info!("connection with the api server validated.");
        info!("API bootstrap complete");
    }

    pub async fn get_socket_config(&self) -> Result<ConnectionRequest, Box<dyn Error>> {
        info!("attempting to acquire socket information.");
        let res = self
            .client
            .get(format!("{}/get_connection/", self.api_uri))
            .send()
            .await?;
        let connection_request: ConnectionRequest = res.json().await?;
        Ok(connection_request)
    }

    async fn get_ping(&self) -> Result<PingRequest, Box<APIError>> {
        info!(
            "trying to ping configured API server at: {}...",
            self.api_uri
        );

        let res = self
            .client
            .get(format!("{}/ping", self.api_uri))
            .send()
            .await
            .map_err(|err| APIError::new(APIErrorKind::APIConnectionError, err.to_string(), 502))?;

        debug!("ping response: {res:#?}");
        let ping_request: PingRequest = res
            .json()
            .await
            .map_err(|err| APIError::new(APIErrorKind::APIConnectionError, err.to_string(), 502))?;
        if ping_request.status == 200 {
            info!("API server is up and running.");
            info!("received message: {}", ping_request.message);
            Ok(ping_request)
        } else {
            error!("API server refused authentication.");
            Err(Box::new(APIError::new(
                APIErrorKind::APIAuthError,
                ping_request.message,
                ping_request.status,
            )))
        }
    }

    async fn validate_connection(&self) {
        let mut timeout_s = vec![60, 30, 15, 15];

        while let Err(e) = self.get_ping().await {
            println!("{:#?}", e);
            match e.kind() {
                APIErrorKind::APIAuthError => {
                    error!(
                        "encountered an authentication error. consider regenerating the socket key"
                    );
                    process::exit(e.code() as i32);
                }
                APIErrorKind::APIConnectionError => {
                    let sleep_time = timeout_s.pop().unwrap_or(120);
                    error!("failed to ping the API server:\n{:#?}", *e);
                    info!("trying again in {} seconds...", sleep_time);
                    sleep(tokio::time::Duration::from_secs(sleep_time));
                }
            }
        }
    }
}
