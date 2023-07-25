#![allow(dead_code, unused)]

use std::{
    process,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::SystemTime,
};

use log::{debug, error, info, trace, warn};
use util::logging::Logger;
use util::watchdog::LocalWatchdog;

use crate::{app::Helmdall, util::watchdog};

mod app;
mod errors;
mod models;
mod util;

#[tokio::main]
async fn main() {
    let logger = Logger::init();
    info!("hello");
    let path = match std::env::var("HOME") {
        Ok(path) => path + "/Documents/helmdall",
        Err(_) => ".".to_string(),
    };
    info!("starting helmdall at path: {}", path);

    let app = app::Helmdall::bootstrap(&path, logger).await;
    info!("application bootstrapped.");

    info!("initializing socket connections...");

    info!("starting local socket listener...");
    let socket_path = path.to_string() + "/helmdall.sock";
    info!("starting local socket listener for file {socket_path}...");
    let local_socket = LocalWatchdog::init(&socket_path);
    let handle = thread::spawn(move || {
        local_socket.listen();
    });

    info!("starting server socket watchdog...");
    let connection_resp = app.get_connection();
    let cfg = app.get_config();
    watchdog::connect_to_socket(connection_resp, cfg);

    handle.join();
}
