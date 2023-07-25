use log::{debug, error, info};
use serde_json::Value;
use std::error::Error;
use std::fmt::format;
use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};
use std::panic::catch_unwind;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{process, thread};
use ws::{connect, CloseCode, Message};

use crate::models::api_comms::ConnectionRequest;
use crate::models::config::Config;

struct RemoteWatchdog {
    ws: ws::Sender,
}

impl ws::Handler for RemoteWatchdog {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let msg_text = msg.as_text().unwrap();
        if msg_text.eq("invalid token") {
            self.ws
                .close_with_reason(CloseCode::Error, "authentication error")
                .unwrap();
        }
        let json: Value = serde_json::from_str(msg_text).unwrap_or_else(|err| {
            self.ws.send("invalid json").unwrap();
            Value::Null
        });
        debug!("got message: {:#?}", json);

        let opcode = json["opcode"].as_str().unwrap_or("INVALID");
        info!("opcode: {opcode}",);
        match opcode {
            "ABORT" => {}
            _ => {
                self.ws.send("invalid opcode").unwrap();
            }
        }
        Ok(())
    }
}

pub fn connect_to_socket(
    connection_response: &ConnectionRequest,
    cfg: &Config,
) -> Result<(), Box<dyn Error>> {
    let (provisioned_socket, connection_token) = (
        &connection_response.provisioned_socket,
        &connection_response.connection_token,
    );

    info!("connecting to socket: {}...", provisioned_socket);
    info!("using connection token: {}...", connection_token);
    let connection_msg = format!("{}:{}", connection_token, cfg.get_client_name());

    connect(provisioned_socket.to_owned(), |out| {
        out.send(connection_msg.as_str()).unwrap();
        info!("token: {}", out.connection_id());

        RemoteWatchdog { ws: out }
    })
    .unwrap();
    Ok(())
}

pub struct LocalWatchdog {
    path: String,
    listener: UnixListener,
}

impl LocalWatchdog {
    pub fn init(path: &str) -> LocalWatchdog {
        let path = Path::new(path);
        if path.exists() {
            info!("removing existing socket file...");
            match std::fs::remove_file(path) {
                Ok(_) => info!("removed existing socket file"),
                Err(e) => {
                    error!("failed to remove existing socket file: {}", e);
                    process::exit(1);
                }
            }
        }
        info!("binding to socket...");
        let listener = match UnixListener::bind(path) {
            Ok(listener) => listener,
            Err(e) => {
                error!("failed to bind to socket: {}", e);
                process::exit(1);
            }
        };
        info!("bound to socket successfully.");
        LocalWatchdog {
            path: path.to_str().unwrap().to_string(),
            listener,
        }
    }

    pub fn listen(&self) {
        info!("listening for connections...");
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("new process connected.");
                    thread::spawn(move || {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    error!("failed to accept connection: {}", e);
                }
            }
        }
    }
}

pub fn handle_client(mut stream: UnixStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    info!("client disconnected");
                    return;
                }
                let msg = String::from_utf8_lossy(&buffer[..bytes]);
                debug!("received message: {}", msg);
            }
            Err(e) => {
                error!("failed to read from socket: {}", e);
                break;
            }
        }
    }
}
