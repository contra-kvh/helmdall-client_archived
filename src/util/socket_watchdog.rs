use log::{debug, info};
use serde_json::Value;
use std::error::Error;
use ws::{connect, CloseCode, Message};

use crate::models::api_comms::ConnectionRequest;
use crate::models::config::Config;

struct SocketClient {
    ws: ws::Sender,
}

impl ws::Handler for SocketClient {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let msg_text = msg.as_text().unwrap();
        if msg_text.eq("invalid token") {
            self.ws
                .close_with_reason(CloseCode::Error, "authentication error")
                .unwrap();
        }
        let json: Value = serde_json::from_str(msg_text).unwrap();
        debug!("got message: {:#?}", json);

        let opcode = json["opcode"].as_str().unwrap();
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
    connection_response: ConnectionRequest,
    cfg: Config,
) -> Result<(), Box<dyn Error>> {
    let (provisioned_socket, connection_token) = (
        connection_response.provisioned_socket,
        connection_response.connection_token,
    );

    info!("connecting to socket: {}...", provisioned_socket);
    info!("using connection token: {}...", connection_token);

    connect(provisioned_socket, |out| {
        out.send(format!("{connection_token}")).unwrap();
        info!("token: {}", out.connection_id());

        SocketClient { ws: out }
    })
    .unwrap();

    Ok(())
}
