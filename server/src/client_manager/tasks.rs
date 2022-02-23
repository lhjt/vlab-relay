use std::{collections::HashMap, sync::Arc};

use tokio::sync::{oneshot::Sender, Mutex};
use tracing::{error, instrument};

use crate::relay::{
    core::{
        AutoTestSubmissionRequest,
        AutoTestSubmissionResponse,
        CheckStyleRequest,
        CheckStyleResponse,
        SubmissionRequest,
        SubmissionResponse,
    },
    ws_extensions::{socket_frame::Opcode, task, SocketFrame, Task},
};

#[derive(Debug, Clone)]
pub(crate) enum CoreMessage {
    AutoTestSubmissionRequest(AutoTestSubmissionRequest),
    AutoTestSubmissionResponse(AutoTestSubmissionResponse),
    CheckStyleRequest(CheckStyleRequest),
    CheckStyleResponse(CheckStyleResponse),
    SubmissionRequest(SubmissionRequest),
    SubmissionResponse(SubmissionResponse),
}

impl CoreMessage {
    /// Converts a `CoreMessage` into a `SocketFrame`.
    pub(crate) fn into_socket_frame(self, id: String) -> SocketFrame {
        match self {
            Self::AutoTestSubmissionRequest(data) => SocketFrame {
                opcode: Opcode::CheckStyleRequest as i32,
                task:   Some(Task {
                    id,
                    data: Some(task::Data::AutotestSubmissionRequest(data)),
                }),
            },
            Self::AutoTestSubmissionResponse(data) => SocketFrame {
                opcode: Opcode::CheckStyleRequest as i32,
                task:   Some(Task {
                    id,
                    data: Some(task::Data::AutotestSubmissionResponse(data)),
                }),
            },
            Self::CheckStyleRequest(data) => SocketFrame {
                opcode: Opcode::CheckStyleRequest as i32,
                task:   Some(Task {
                    id,
                    data: Some(task::Data::CheckStyleRequest(data)),
                }),
            },
            Self::CheckStyleResponse(data) => SocketFrame {
                opcode: Opcode::CheckStyleResponse as i32,
                task:   Some(Task {
                    id,
                    data: Some(task::Data::CheckStyleResponse(data)),
                }),
            },
            Self::SubmissionRequest(data) => SocketFrame {
                opcode: Opcode::SubmissionRequest as i32,
                task:   Some(Task {
                    id,
                    data: Some(task::Data::SubmissionRequest(data)),
                }),
            },
            Self::SubmissionResponse(data) => SocketFrame {
                opcode: Opcode::SubmissionResponse as i32,
                task:   Some(Task {
                    id,
                    data: Some(task::Data::SubmissionResponse(data)),
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
