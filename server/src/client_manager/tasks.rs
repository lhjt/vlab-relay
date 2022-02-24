use std::{collections::HashMap, sync::Arc};

use tokio::sync::{oneshot::Sender, Mutex};
use tracing::{error, instrument};

use crate::relay::{core::CommandResponse, ws_extensions::TaskResponse};

#[derive(Debug, Clone)]
pub(crate) struct TaskList {
    pub(crate) tasks: Arc<Mutex<HashMap<String, Sender<Option<CommandResponse>>>>>,
}

impl TaskList {
    pub(crate) fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) async fn add_task(&self, id: String, channel: Sender<Option<CommandResponse>>) {
        self.tasks.lock().await.insert(id, channel);
    }

    /// Removes a task from the list and sends the result to the task's channel.
    #[instrument]
    pub(crate) async fn complete_task(&self, result: TaskResponse) {
        let chan = self
            .tasks
            .lock()
            .await
            .remove(&result.id)
            .unwrap_or_else(|| {
                error!("attempted to complete a task that doesn't exist");
                panic!();
            });

        if result.response.is_none() {
            // this shouldn't happen if the runner is valid
            error!("task response was None");
        }

        if let Err(e) = chan.send(result.response) {
            error!("failed to send message to task: {:?}", e);
        }
    }
}
