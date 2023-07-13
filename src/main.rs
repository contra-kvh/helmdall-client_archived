use std::error::Error;
use std::io::{Write, Read};
use std::net::TcpStream;
use std::process;
use std::thread::sleep;
use reqwest::{Client};

mod models;
use models::config::Config;
use models::api_comms::*;

fn connect_to_socket(connection_response: ConnectionRequest, cfg: Config) -> Result<(), Box<dyn Error>> {
    println!("connecting to socket: {}...", connection_response.provisioned_socket);
    println!("using connection token: {}...", connection_response.connection_token);

    let mut tcp_stream = TcpStream::connect(connection_response.provisioned_socket)?;
    let connection_str = format!("{}:{}", cfg.get_socket_key(), connection_response.connection_token);
    tcp_stream.write_all(connection_str.as_bytes())?;

    loop {
        let mut buffer = [0; 512];
        match tcp_stream.read(&mut buffer){
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("connection closed");
                    return Ok(());
                }
            },
            Err(e) => {
                println!("failed to read from socket:\n{e:?}");
                println!("exiting...");
                return Err(Box::new(e));
            }
        };
        let received = String::from_utf8_lossy(&buffer);
        let received = received.trim();
        println!("received data: {}", received);
    }
}

async fn get_connection_response(api_uri: &str, client: &Client) -> Result<ConnectionRequest, Box<dyn Error>> {
    println!("getting connection information from: {api_uri}...");
    let res = client.get(format!("{api_uri}/get_connection")).send().await?;
    let connection_request: ConnectionRequest = res.json().await?;
    Ok(connection_request)
}

async fn get_ping(api_uri: &str, client: &Client) -> Result<PingRequest, Box<dyn Error>> {
    println!("trying to ping configured API server at: {api_uri}...");
    let res = client.get(format!("{api_uri}/ping")).send().await?;
    let ping_request: PingRequest = res.json().await?;
    Ok(ping_request)
}

async fn drive(cfg: Config){
    let client = Client::new();
    let mut timeout_s = vec![15, 15, 30, 60];

    while let Err(e) = get_ping(cfg.get_api_uri(), &client).await {
        let sleep_time = timeout_s.pop().unwrap_or(120);
        println!("failed to ping the API server:\n{e:?}");
        println!("trying again in {} seconds...", sleep_time);
        sleep(tokio::time::Duration::from_secs(sleep_time));
    }

    let connection_response = get_connection_response(cfg.get_api_uri(), &client).await.unwrap();
    println!("connection response: {connection_response:?}");
    connect_to_socket(connection_response, cfg).unwrap();
}

fn initialize_app(cfg_path: &str) -> Config {
    match Config::load_from_file(&cfg_path){
        Ok(cfg) => cfg,
        Err(e) => {
            println!("failed to process the config file.\n{e:?}");
            println!("attempting to save a dummy config at the config path...");
            write_cfg(Config::new(), cfg_path);
            println!("consider generating a config from your helmdall account :)");
            process::exit(2);
        }
    }
}

fn write_cfg(cfg: Config, cfg_path: &str) {
    if let Err(e) = cfg.save_to_file(cfg_path) {
        println!("failed to save the config:\n{e:?}");
        process::exit(3);
    }
}

#[tokio::main]
async fn main() {
    let cfg_path = "config.yaml";
    let cfg = initialize_app(cfg_path);

    drive(cfg).await;
}
