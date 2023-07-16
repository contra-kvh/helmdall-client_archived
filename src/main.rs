#![allow(dead_code, unused)]

use reqwest::Client;

mod errors;
mod models;
mod util;

use models::config::Config;
use util::bootstrap;

async fn drive(cfg: Config) {}

#[tokio::main]
async fn main() {
    let cfg_path = "config.yaml";
    let (cfg, api_client) = bootstrap::bootstrap(cfg_path).await;
    drive(cfg).await;
}
