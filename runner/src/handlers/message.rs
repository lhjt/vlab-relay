use futures::channel::mpsc::UnboundedSender;
use log::{error, warn};
use prost::bytes::Bytes;
use tokio_tungstenite::tungstenite::Message;

use super::task::handle_task_request;
use crate::relay::ws_extensions::{socket_frame::Data, SocketFrame};

pub(crate) async fn handle_message(msg: Message, tx: UnboundedSender<Message>) {
    match msg {
        Message::Binary(data) => {
            // attempt to parse the message as a task request
            match <SocketFrame as prost::Message>::decode(Bytes::from(data)) {
                Ok(frame) => {
                    // if we successfully parsed the message, then we can
                    // process it
                    if let Some(data) = frame.data {
                        if let Data::TaskRequest(req) = data {
                            handle_task_request(req, tx).await;
                        }
                    } else {
                        error!("received invalid message from relay");
                    }
                },
                Err(e) => {
                    error!("failed to parse message: {}", e);
                },
            }
        },
        Message::Close(_) => {
            warn!("received close message from relay");
        },
        _ => {
            // should not be receiving any other message types
            warn!("received unexpected message type");
        },
    }
}
