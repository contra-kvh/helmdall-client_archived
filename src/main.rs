#![allow(dead_code, unused)]

use std::{process, time::SystemTime};

use log::{debug, error, info, trace, warn};
use util::logging::Logger;

mod app;
mod errors;
mod models;
mod util;

#[tokio::main]
async fn main() {
    let logger = Logger::init();
    info!("hello");
    let path = match std::env::var("HOME") {
        Ok(path) => path,
        Err(_) => ".".to_string(),
    };
    info!("starting helmdall at path: {}", path);

    let app = app::Helmdall::bootstrap(&path, logger).await;
    error!("Goes to stderr and file");
    warn!("Goes to stderr and file");
    info!("Goes to stderr and file");
    debug!("Goes to file only");
    trace!("Goes to file only");
}
