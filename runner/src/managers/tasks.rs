use std::{
    fs::DirBuilder,
    path::{Path, PathBuf},
    process::Stdio,
};

use colored::Colorize;
use log::{error, info};

use crate::relay::{
    core::{CommandRequest, CommandResponse, Directory},
    ws_extensions::{TaskRequest, TaskResponse},
};

#[derive(Debug)]
pub(crate) struct Task {
    pub(crate) id:      String,
    pub(crate) request: CommandRequest,
}

macro_rules! return_error_response {
    ($id:expr, $msg:expr) => {{
        error!($msg);
        return TaskResponse {
            id:       $id,
            response: Some(CommandResponse {
                output:    format!("operation failed: {}", $msg.red().to_string()),
                exit_code: -1,
            }),
        }}
    };
    ($id:expr, $msg:expr, $($arg:expr),*) => {{
        error!($msg, $($arg),*);
        return TaskResponse {
            id:       $id,
            response: Some(CommandResponse {
                output:    format!("operation failed: {}", format!($msg, $($arg),*)).red().to_string(),
                exit_code: -1,
            }),
        }}
    };
}

impl Task {
    pub(crate) async fn execute(self) -> TaskResponse {
        info!("executing task: {}", self.id);
        // create a new temporary folder and cd into it
        let folder_name = format!("runner-tmp-{}", self.id);

        // create all of the relevant files in this directory
        let root_dir = match self.request.directory {
            Some(mut d) => {
                // the root directory will contain everything else, so we will name it the temp
                // folder
                d.name = folder_name.clone();
                d
            },
            None => return_error_response!(self.id, "no directory specified"),
        };

        // realise the directory
        match root_dir.realise(Path::new(".")) {
            Ok(_) => {},
            Err(e) => return_error_response!(self.id, "failed to create working directory: {}", e),
        }

        // all files have been created; now we can execute the command
        let child = match tokio::process::Command::new(self.request.command)
            .args(self.request.arguments)
            .current_dir(Path::new(&folder_name))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(r) => r,
            Err(e) => return_error_response!(self.id, "failed to execute command: {}", e),
        };

        // wait for the child to finish
        match child.wait_with_output().await {
            Ok(r) => {
                info!(
                    "task {} finished with exit code {}",
                    self.id,
                    r.status.code().unwrap_or(2)
                );
                // delete the temp folder
                std::fs::remove_dir_all(folder_name.clone()).unwrap_or_else(|e| {
                    error!(
                        "failed to delete temporary directory {}/: {}",
                        folder_name, e
                    );
                });

                let status = r.status.code().unwrap_or(1);

                TaskResponse {
                    id:       self.id,
                    response: Some(CommandResponse {
                        // TODO: decide how to handle stdout/stderr together
                        output:    String::from_utf8_lossy(&r.stdout).to_string()
                            + &String::from_utf8_lossy(&r.stderr).to_string(),
                        exit_code: status as i64,
                    }),
                }
            },
            Err(e) => {
                return_error_response!(self.id, "failed to wait for command to finish: {}", e)
            },
        }
    }
}

impl Directory {
    /// Creates the directory and all files and directories in it.
    pub(crate) fn realise(self, root: impl Into<PathBuf>) -> Result<(), std::io::Error> {
        // create each file in this directory
        // create this intial directory
        let path = Path::join(&root.into(), self.name);
        DirBuilder::new().recursive(true).create(&path)?;

        // create each file in this directory
        for file in self.files.into_iter() {
            std::fs::write(Path::join(&path, file.file_name), file.data)?;
        }

        // create each directory in this directory and realise it
        for dir in self.directories.into_iter() {
            dir.realise(&path)?;
        }

        Ok(())
    }
}

impl From<TaskRequest> for Task {
    fn from(cr: TaskRequest) -> Self {
        Self {
            id:      cr.id,
            request: cr.command.unwrap_or_else(|| {
                error!("received a task request without a command");
                panic!()
            }),
        }
    }
}
