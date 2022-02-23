use std::{collections::HashMap, sync::Arc};

use tokio::sync::{oneshot::Sender, Mutex};
use tracing::{error, instrument};

use crate::relay::{
    core::{CheckStyleRequest, CheckStyleResponse},
    ws_extensions::{socket_frame::Opcode, task, SocketFrame, Task},
};

#[derive(Debug, Clone)]
pub(crate) enum CoreMessage {
    CheckStyleRequest(CheckStyleRequest),
    CheckStyleResponse(CheckStyleResponse),
}

impl CoreMessage {
    /// Converts a `CoreMessage` into a `SocketFrame`.
    pub(crate) fn into_socket_frame(self, id: String) -> SocketFrame {
        match self {
            Self::CheckStyleRequest(csr) => SocketFrame {
                opcode: Opcode::CheckStyleRequest as i32,
                task:   Some(Task {
                    id,
                    data: Some(task::Data::CheckStyleRequest(csr)),
                }),
            },
            Self::CheckStyleResponse(csr) => SocketFrame {
                opcode: Opcode::CheckStyleResponse as i32,
                task:   Some(Task {
                    id,
                    data: Some(task::Data::CheckStyleResponse(csr)),
                }),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TaskList {
    pub(crate) tasks: Arc<Mutex<HashMap<String, Sender<CoreMessage>>>>,
}

impl TaskList {
    pub(crate) fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) async fn add_task(&self, id: String, channel: Sender<CoreMessage>) {
        self.tasks.lock().await.insert(id, channel);
    }

    /// Removes a task from the list and sends the result to the task's channel.
    #[instrument]
    pub(crate) async fn complete_task(&self, id: String, message: CoreMessage) {
        let chan = self.tasks.lock().await.remove(&id).unwrap_or_else(|| {
            error!("attempted to complete a task that doesn't exist");
            panic!();
        });

        if let Err(e) = chan.send(message) {
            error!("failed to send message to task: {:?}", e);
        }
    }
}
