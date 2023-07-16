#![allow(dead_code, unused)]
mod app;
mod errors;
mod models;
mod util;

#[tokio::main]
async fn main() {
    let cfg_path = "config.yaml";
    let app = app::Helmdall::bootstrap(cfg_path).await;
}
