use futures_core::stream::BoxStream;
use memory_stats::memory_stats;
use tokio_stream::StreamExt;
use tonic::{Response, Status};
use tracing::info;
use tracing::instrument;

use crate::gen::*;
use crate::index;
use crate::index::*;

type TonicResult<T> = Result<T, tonic::Status>;

pub fn new() -> Service {
    let interface = index::new();
    Service { interface }
}

pub struct Service {
    interface: index::Interface,
}

// Required for instrumentation macros
impl std::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}

#[tonic::async_trait]
impl searchium_service_server::SearchiumService for Service {
    async fn hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> TonicResult<Response<HelloResponse>> {
        info!("Got hello from {}", request.into_inner().id);
        let reply = HelloResponse {
            id: "rust searchium server".to_string(),
        };
        Ok(Response::new(reply))
    }

    #[instrument(err)]
    async fn set_configuration(
        &self,
        request: tonic::Request<ConfigurationRequest>,
    ) -> TonicResult<tonic::Response<ConfigurationResponse>> {
        self.interface
            .set_configuration(request.into_inner())
            .await?;
        Ok(Response::new(ConfigurationResponse {}))
    }

    type RegisterFolderStream = BoxStream<'static, TonicResult<IndexUpdate>>;

    #[instrument(err)]
    async fn register_folder(
        &self,
        request: tonic::Request<FolderRegisterRequest>,
    ) -> TonicResult<Response<Self::RegisterFolderStream>> {
        let stream = self.interface.register_folder(request.into_inner()).await?;
        let stream = stream.map(|update| -> TonicResult<IndexUpdate> { Ok(update) });
        Ok(Response::new(Box::pin(stream)))
    }

    #[instrument(err)]
    async fn unregister_folder(
        &self,
        request: tonic::Request<FolderUnregisterRequest>,
    ) -> TonicResult<Response<GenericResponse>> {
        match self.interface.unregister_folder(request.into_inner()).await {
            Err(e) => Err(Status::internal(format!("{:?}", e))),
            Ok(_) => Ok(Response::new(GenericResponse {
                error: 0,
                log_message: "".to_owned(),
            })),
        }
    }

    type SearchFilePathsStream = BoxStream<'static, TonicResult<FilePathSearchResponse>>;

    async fn search_file_paths(
        &self,
        request: tonic::Request<tonic::Streaming<FilePathSearchRequest>>,
    ) -> TonicResult<tonic::Response<Self::SearchFilePathsStream>> {
        let request_stream = request.into_inner().map_while(|s| s.ok());
        let mapped = self
            .interface
            .search_file_paths(request_stream)
            .await?
            .map(|r| r.map_err(|e| e.into()));
        return Ok(Response::new(
            Box::pin(mapped) as Self::SearchFilePathsStream
        ));
    }

    async fn search_file_contents(
        &self,
        request: tonic::Request<FileContentsSearchRequest>,
    ) -> TonicResult<tonic::Response<FileContentsSearchResponse>> {
        Ok(Response::new(
            self.interface
                .search_file_contents(request.into_inner())
                .await?,
        ))
    }

    async fn get_file_extracts(
        &self,
        request: tonic::Request<FileExtractsRequest>,
    ) -> TonicResult<tonic::Response<FileExtractsResponse>> {
        Ok(Response::new(
            self.interface
                .get_file_extracts(request.into_inner())
                .await?,
        ))
    }

    async fn get_process_info(
        &self,
        _request: tonic::Request<ProcessInfoRequest>,
    ) -> TonicResult<tonic::Response<ProcessInfoResponse>> {
        let stats = memory_stats().ok_or_else(|| Status::internal(""))?;
        Ok(Response::new(ProcessInfoResponse {
            physical_memory: stats.physical_mem as u64,
            virtual_memory: stats.virtual_mem as u64,
        }))
    }

    #[instrument(err)]
    async fn get_database_details(
        &self,
        _request: tonic::Request<DatabaseDetailsRequest>,
    ) -> TonicResult<tonic::Response<DatabaseDetailsResponse>> {
        let details = self.interface.get_database_details().await?;
        Ok(Response::new(details))
    }

    type GetStatusStream = BoxStream<'static, TonicResult<StatusResponse>>;

    #[instrument(err)]
    async fn get_status(
        &self,
        _request: tonic::Request<StatusRequest>,
    ) -> TonicResult<tonic::Response<Self::GetStatusStream>> {
        Ok(Response::new(Box::pin(
            self.interface.get_status_stream().map(Ok),
        )))
    }
}

impl From<CommandError> for tonic::Status {
    fn from(command_error: CommandError) -> Self {
        match command_error {
            CommandError::InvalidArgument(s) => tonic::Status::invalid_argument(s),
            CommandError::NotFound(s) => tonic::Status::not_found(s),
            CommandError::InternalError(s) => tonic::Status::internal(s),
        }
    }
}
