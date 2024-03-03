use async_stream::stream;
use futures::{FutureExt, Stream};
use tokio::sync::{mpsc, oneshot, watch};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use super::{AsyncCommand, Command, CommandResult, IndexServer};
use crate::gen::*;

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

// TODO: channel may not be necessary, could be a ref to the IndexServer itself (if IndexServer becomes Sync?)
pub struct IndexInterface {
    command_tx: mpsc::Sender<Command>,
    async_command_tx: mpsc::Sender<AsyncCommand>,
    status_rx: watch::Receiver<StatusResponse>,
}

impl IndexInterface {
    pub async fn get_database_details(&self) -> CommandResult<DatabaseDetailsResponse> {
        do_oneshot(&self.command_tx, |s| s.state.get_database_details()).await
    }
    pub async fn set_configuration(&self, request: ConfigurationRequest) -> CommandResult<()> {
        do_oneshot(&self.command_tx, |s| s.state.set_configuration(request)).await
    }
    pub async fn register_folder(
        &self,
        request: FolderRegisterRequest,
    ) -> CommandResult<impl Stream<Item = IndexUpdate>> {
        let (tx, mut rx) = mpsc::channel(16);
        self.async_command_tx
            .send(Box::new(|s: &mut IndexServer| {
                async move {
                    s.state.register_folder(tx, request).await.ok();
                }
                .boxed()
            }))
            .await?;
        Ok(stream! {
            while let Some(update) = rx.recv().await {
                yield update;
            }
        })
    }
    pub async fn unregister_folder(&self, request: FolderUnregisterRequest) -> CommandResult<()> {
        do_oneshot(&self.command_tx, |s| s.state.unregister_folder(request)).await
    }
    pub async fn search_file_paths<RequestStream: Stream<Item = FilePathSearchRequest>>(
        &self,
        request: RequestStream,
    ) -> CommandResult<impl Stream<Item = CommandResult<FilePathSearchResponse>>> {
        // TODO: Consider refactoring so tx doesn't have to be cloned as much - e.g. explicitly create output stream from channel that's captured by command?
        // Benefit of current path is it ought to allow search requests to interleave with other commands
        let tx = self.command_tx.clone();
        Ok(request.then(move |request| {
            let tx = tx.clone();
            async move {
                do_oneshot(&tx, |s: &mut IndexServer| {
                    s.state.search_file_paths(request)
                })
                .await
            }
        }))
    }
    pub async fn search_file_contents(
        &self,
        request: FileContentsSearchRequest,
    ) -> CommandResult<FileContentsSearchResponse> {
        let token = CancellationToken::new();
        do_oneshot(&self.command_tx, |s: &mut IndexServer| {
            s.state.search_file_contents(request, token)
        })
        .await
    }
    pub async fn get_file_extracts(
        &self,
        request: FileExtractsRequest,
    ) -> CommandResult<FileExtractsResponse> {
        do_oneshot(&self.command_tx, |s: &mut IndexServer| {
            s.state.get_file_extracts(request)
        })
        .await
    }

    pub fn get_status_stream(&self) -> impl Stream<Item = StatusResponse> {
        tokio_stream::wrappers::WatchStream::new(self.status_rx.clone())
    }
}

// Helpers

// Execute an operation on the index server and return a single result
// TODO: Work out best way to allow f to be an async function - needs boxing and pinning of returned future?
// Or can command be put in a trait object instead to wrap up both types?
async fn do_oneshot<R, F>(channel: &mpsc::Sender<Command>, f: F) -> CommandResult<R>
where
    F: FnOnce(&mut IndexServer) -> CommandResult<R> + Send + 'static,
    R: Send + 'static,
{
    let (tx, rx) = oneshot::channel();
    let wrapper = |s: &mut IndexServer| {
        let value = f(s);
        // TODO: error handling here? how would it be handled anyway
        tx.send(value).ok();
    };
    channel.send(Box::new(wrapper)).await?;
    rx.await?
}
