use futures::channel::mpsc::UnboundedSender;
use log::{error, info};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    managers::tasks::Task,
    relay::ws_extensions::{socket_frame::Data, SocketFrame, TaskRequest},
};

pub(crate) async fn handle_task_request(req: TaskRequest, tx: UnboundedSender<Message>) {
    info!("received task request: {}", req.id);
    // execute the task
    let response = Task::from(req).execute().await;

    info!("sending task response: {}", response.id);

    // create return message
    let return_frame = SocketFrame {
        data: Some(Data::TaskResponse(response)),
    };

    if let Err(e) = tx.unbounded_send(Message::Binary(prost::Message::encode_to_vec(
        &return_frame,
    ))) {
        error!("failed to send task response: {}", e);
    }
}
