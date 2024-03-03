use crate::fs_state::*;
use crate::gen::*;

use futures::future::BoxFuture;
use std::path::Path;
use std::{fmt::Debug, time::SystemTime};
use thiserror::Error;
use tokio::sync::watch;
use tokio::sync::{mpsc, oneshot};
use tracing::{event, Level};

pub(crate) mod interface;
mod state;

// Re-exports
pub type Interface = interface::IndexInterface;

struct Configuration {
    concurrent_file_reads: u32,
    max_file_size: u64,
    large_file_threshold: u64,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            concurrent_file_reads: 64,
            max_file_size: 10 * 1024 * 1024,
            large_file_threshold: 1024 * 1024,
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum CommandError {
    #[error("Invalid argument {0}")]
    InvalidArgument(String),
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

type CommandResult<T> = Result<T, CommandError>;

// TODO: If refactoring to this generic style, work out how to do it with borrow - does this need Pin?
// TODO: Consider Result return type?
type Command = Box<dyn FnOnce(&mut IndexServer) + Send>;
type AsyncCommand = Box<dyn Send + for<'a> FnOnce(&'a mut IndexServer) -> BoxFuture<'a, ()>>;

pub struct IndexServer {
    command_rx: mpsc::Receiver<Command>,
    async_command_rx: mpsc::Receiver<AsyncCommand>,
    state: state::IndexState,
}

#[allow(dead_code)]
pub fn new() -> interface::IndexInterface {
    let (status_tx, status_rx) = watch::channel(StatusResponse::default());
    let (command_tx, command_rx) = mpsc::channel(8);
    let (async_command_tx, async_command_rx) = mpsc::channel(8);
    let server = IndexServer {
        command_rx,
        async_command_rx,
        state: state::new(status_tx),
    };
    tokio::spawn(async move { server.run_internal().await });
    interface::new(command_tx, async_command_tx, status_rx)
}

impl std::fmt::Debug for IndexServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("IndexServer")
    }
}

impl IndexUpdate {
    fn scan_start() -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(index_update::Type::FilesystemScanStart(
                index_update::FilesystemScanStart::default(),
            )),
        }
    }
    fn scan_end() -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(index_update::Type::FilesystemScanEnd(
                index_update::FilesystemScanEnd::default(),
            )),
        }
    }
    fn loaded() -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(index_update::Type::FileContentsLoaded(
                index_update::FileContentsLoaded::default(),
            )),
        }
    }
    fn files_loaded(count: usize, total: usize, path: &Path) -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(index_update::Type::FileContentsLoaded(
                index_update::FileContentsLoaded {
                    count: count as u32,
                    total: total as u32,
                    path: path.to_string_lossy().to_string(),
                },
            )),
        }
    }
}

// TODO: Setup directory watcher to feed update commands back into index state
impl IndexServer {
    async fn run_internal(mut self) {
        event!(Level::INFO, "Starting index state command loop");
        loop {
            tokio::select! {
                Some(command) = self.command_rx.recv() => { command(&mut self); }
                Some(async_command) = self.async_command_rx.recv() => {Box::pin(async_command(&mut self)).await; }
                else => {
                    break;
                }
            }
        }
        event!(Level::INFO, "Leaving index state command loop");
    }
}

fn match_file_path(file: &Path, fragments: &[&str]) -> Option<String> {
    file.to_str().and_then(|s| {
        fragments
            .iter()
            .all(|f| s.contains(f))
            .then(|| s.to_owned())
    })
}

impl<T> From<mpsc::error::SendError<T>> for CommandError {
    fn from(value: mpsc::error::SendError<T>) -> Self {
        CommandError::InternalError(value.to_string())
    }
}

impl From<oneshot::error::RecvError> for CommandError {
    fn from(value: oneshot::error::RecvError) -> Self {
        CommandError::InternalError(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_single_fragment_match() {
        let file = "C:\\code\\projects\\foo\\src\\module.cpp";
        let path = PathBuf::from(file);
        let fragments: Vec<_> = "foo".split_whitespace().collect();
        let m = match_file_path(&path, &fragments);
        assert_eq!(m.as_ref().map(|s| s.as_str()), Some(file));
    }

    #[test]
    fn test_two_fragment_match() {
        let file = "C:\\code\\projects\\foo\\src\\module.cpp";
        let path = PathBuf::from(file);
        let fragments: Vec<_> = "foo cpp".split_whitespace().collect();
        let m = match_file_path(&path, &fragments);
        assert_eq!(m.as_ref().map(|s| s.as_str()), Some(file));
    }
}
