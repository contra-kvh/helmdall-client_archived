#![allow(dead_code, unused)]

use std::{process, time::SystemTime};

use fern::Dispatch;
use log::{debug, error, info, warn};
use util::logging::setup_logger;

mod app;
mod errors;
mod models;
mod util;

#[tokio::main]
async fn main() -> Result<(), fern::InitError> {
    setup_logger(&log::LevelFilter::Debug);
    let path = match std::env::var("HELMDALL_PATH") {
        Ok(path) => path,
        Err(_) => ".".to_string(),
    };
    info!("starting helmdall at path: {}", path);

    let app = app::Helmdall::bootstrap(&path).await;

    Ok(())
}
