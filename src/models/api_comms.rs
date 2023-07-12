use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionRequest {
    pub connection_token: String,
    pub provisioned_socket: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRequest {
    pub status: i32,
    pub message: String
}