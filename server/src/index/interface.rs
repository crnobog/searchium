use futures::{FutureExt, Stream};
use tokio::sync::{mpsc, oneshot, watch};
use tokio_util::sync::CancellationToken;

use super::{state::State, AsyncCommand, Command, CommandResult};
use crate::gen::searchium::*;

pub fn new(
    command_tx: mpsc::Sender<Command>,
    async_command_tx: mpsc::Sender<AsyncCommand>,
    status_rx: watch::Receiver<StatusResponse>,
) -> IndexInterface {
    IndexInterface {
        command_tx,
        async_command_tx,
        status_rx,
    }
}

#[derive(Clone)]
pub struct IndexInterface {
    command_tx: mpsc::Sender<Command>,
    async_command_tx: mpsc::Sender<AsyncCommand>,
    status_rx: watch::Receiver<StatusResponse>,
}

impl IndexInterface {
    pub async fn get_database_details(&self) -> CommandResult<DatabaseDetailsResponse> {
        self.do_oneshot(|s| s.get_database_details()).await
    }
    pub async fn set_configuration(&self, request: ConfigurationRequest) -> CommandResult<()> {
        self.do_oneshot(|s| s.set_configuration(request)).await
    }
    pub async fn register_folder(
        &self,
        request: FolderRegisterRequest,
    ) -> CommandResult<impl Stream<Item = IndexUpdate>> {
        let (tx, rx) = mpsc::channel(16);
        self.async_command_tx
            .send(Box::new(|s: &mut State| {
                async move {
                    s.register_folder(tx, request).await.ok();
                }
                .boxed()
            }))
            .await?;
        Ok(tokio_stream::wrappers::ReceiverStream::new(rx))
    }
    pub async fn unregister_folder(&self, request: FolderUnregisterRequest) -> CommandResult<()> {
        self.do_oneshot(|s| s.unregister_folder(request)).await
    }
    pub async fn search_file_paths(
        &self,
        request: FilePathSearchRequest,
    ) -> CommandResult<FilePathSearchResponse> {
        self.do_oneshot(|s| s.search_file_paths(request)).await
    }
    pub async fn search_file_contents(
        &self,
        request: FileContentsSearchRequest,
    ) -> CommandResult<FileContentsSearchResponse> {
        let token = CancellationToken::new();
        self.do_oneshot(|s| s.search_file_contents(request, token))
            .await
    }
    pub async fn get_file_extracts(
        &self,
        request: FileExtractsRequest,
    ) -> CommandResult<FileExtractsResponse> {
        self.do_oneshot(|s| s.get_file_extracts(request)).await
    }

    pub fn get_status_stream(&self) -> impl Stream<Item = StatusResponse> {
        tokio_stream::wrappers::WatchStream::new(self.status_rx.clone())
    }

    // Execute an operation on the index server and return a single result
    async fn do_oneshot<R, F>(&self, f: F) -> CommandResult<R>
    where
        F: FnOnce(&mut State) -> CommandResult<R> + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let wrapper = |s: &mut State| {
            let value = f(s);
            if tx.send(value).is_err() {
                panic!("Oneshot receiver unexpectedly dropped in do_oneshot");
            }
        };
        self.command_tx.send(Box::new(wrapper)).await?;
        rx.await?
    }
}
