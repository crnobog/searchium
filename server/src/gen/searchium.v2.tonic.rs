// @generated
/// Generated server implementations.
pub mod searchium_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SearchiumServiceServer.
    #[async_trait]
    pub trait SearchiumService: Send + Sync + 'static {
        async fn hello(
            &self,
            request: tonic::Request<super::HelloRequest>,
        ) -> std::result::Result<tonic::Response<super::HelloResponse>, tonic::Status>;
        /// Server streaming response type for the RegisterFolder method.
        type RegisterFolderStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::IndexUpdate, tonic::Status>,
            >
            + Send
            + 'static;
        async fn register_folder(
            &self,
            request: tonic::Request<super::FolderRegisterRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::RegisterFolderStream>,
            tonic::Status,
        >;
        async fn unregister_folder(
            &self,
            request: tonic::Request<super::FolderUnregisterRequest>,
        ) -> std::result::Result<tonic::Response<super::GenericResponse>, tonic::Status>;
        /// Server streaming response type for the SearchFilePaths method.
        type SearchFilePathsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::FilePathSearchResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn search_file_paths(
            &self,
            request: tonic::Request<tonic::Streaming<super::FilePathSearchRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::SearchFilePathsStream>,
            tonic::Status,
        >;
        async fn search_file_contents(
            &self,
            request: tonic::Request<super::FileContentsSearchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FileContentsSearchResponse>,
            tonic::Status,
        >;
        async fn get_file_extracts(
            &self,
            request: tonic::Request<super::FileExtractsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FileExtractsResponse>,
            tonic::Status,
        >;
        async fn set_configuration(
            &self,
            request: tonic::Request<super::ConfigurationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigurationResponse>,
            tonic::Status,
        >;
        async fn get_process_info(
            &self,
            request: tonic::Request<super::ProcessInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProcessInfoResponse>,
            tonic::Status,
        >;
        async fn get_database_details(
            &self,
            request: tonic::Request<super::DatabaseDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DatabaseDetailsResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetStatus method.
        type GetStatusStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::StatusResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_status(
            &self,
            request: tonic::Request<super::StatusRequest>,
        ) -> std::result::Result<tonic::Response<Self::GetStatusStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SearchiumServiceServer<T: SearchiumService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SearchiumService> SearchiumServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SearchiumServiceServer<T>
    where
        T: SearchiumService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/searchium.v2.SearchiumService/Hello" => {
                    #[allow(non_camel_case_types)]
                    struct HelloSvc<T: SearchiumService>(pub Arc<T>);
                    impl<
                        T: SearchiumService,
                    > tonic::server::UnaryService<super::HelloRequest> for HelloSvc<T> {
                        type Response = super::HelloResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HelloRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SearchiumService>::hello(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HelloSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/searchium.v2.SearchiumService/RegisterFolder" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterFolderSvc<T: SearchiumService>(pub Arc<T>);
                    impl<
                        T: SearchiumService,
                    > tonic::server::ServerStreamingService<super::FolderRegisterRequest>
                    for RegisterFolderSvc<T> {
                        type Response = super::IndexUpdate;
                        type ResponseStream = T::RegisterFolderStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FolderRegisterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SearchiumService>::register_folder(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterFolderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/searchium.v2.SearchiumService/UnregisterFolder" => {
                    #[allow(non_camel_case_types)]
                    struct UnregisterFolderSvc<T: SearchiumService>(pub Arc<T>);
                    impl<
                        T: SearchiumService,
                    > tonic::server::UnaryService<super::FolderUnregisterRequest>
                    for UnregisterFolderSvc<T> {
                        type Response = super::GenericResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FolderUnregisterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SearchiumService>::unregister_folder(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnregisterFolderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/searchium.v2.SearchiumService/SearchFilePaths" => {
                    #[allow(non_camel_case_types)]
                    struct SearchFilePathsSvc<T: SearchiumService>(pub Arc<T>);
                    impl<
                        T: SearchiumService,
                    > tonic::server::StreamingService<super::FilePathSearchRequest>
                    for SearchFilePathsSvc<T> {
                        type Response = super::FilePathSearchResponse;
                        type ResponseStream = T::SearchFilePathsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::FilePathSearchRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SearchiumService>::search_file_paths(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchFilePathsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/searchium.v2.SearchiumService/SearchFileContents" => {
                    #[allow(non_camel_case_types)]
                    struct SearchFileContentsSvc<T: SearchiumService>(pub Arc<T>);
                    impl<
                        T: SearchiumService,
                    > tonic::server::UnaryService<super::FileContentsSearchRequest>
                    for SearchFileContentsSvc<T> {
                        type Response = super::FileContentsSearchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FileContentsSearchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SearchiumService>::search_file_contents(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchFileContentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/searchium.v2.SearchiumService/GetFileExtracts" => {
                    #[allow(non_camel_case_types)]
                    struct GetFileExtractsSvc<T: SearchiumService>(pub Arc<T>);
                    impl<
                        T: SearchiumService,
                    > tonic::server::UnaryService<super::FileExtractsRequest>
                    for GetFileExtractsSvc<T> {
                        type Response = super::FileExtractsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FileExtractsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SearchiumService>::get_file_extracts(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFileExtractsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/searchium.v2.SearchiumService/SetConfiguration" => {
                    #[allow(non_camel_case_types)]
                    struct SetConfigurationSvc<T: SearchiumService>(pub Arc<T>);
                    impl<
                        T: SearchiumService,
                    > tonic::server::UnaryService<super::ConfigurationRequest>
                    for SetConfigurationSvc<T> {
                        type Response = super::ConfigurationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigurationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SearchiumService>::set_configuration(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetConfigurationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/searchium.v2.SearchiumService/GetProcessInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetProcessInfoSvc<T: SearchiumService>(pub Arc<T>);
                    impl<
                        T: SearchiumService,
                    > tonic::server::UnaryService<super::ProcessInfoRequest>
                    for GetProcessInfoSvc<T> {
                        type Response = super::ProcessInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProcessInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SearchiumService>::get_process_info(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProcessInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/searchium.v2.SearchiumService/GetDatabaseDetails" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatabaseDetailsSvc<T: SearchiumService>(pub Arc<T>);
                    impl<
                        T: SearchiumService,
                    > tonic::server::UnaryService<super::DatabaseDetailsRequest>
                    for GetDatabaseDetailsSvc<T> {
                        type Response = super::DatabaseDetailsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DatabaseDetailsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SearchiumService>::get_database_details(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatabaseDetailsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/searchium.v2.SearchiumService/GetStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatusSvc<T: SearchiumService>(pub Arc<T>);
                    impl<
                        T: SearchiumService,
                    > tonic::server::ServerStreamingService<super::StatusRequest>
                    for GetStatusSvc<T> {
                        type Response = super::StatusResponse;
                        type ResponseStream = T::GetStatusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SearchiumService>::get_status(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: SearchiumService> Clone for SearchiumServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: SearchiumService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SearchiumService> tonic::server::NamedService for SearchiumServiceServer<T> {
        const NAME: &'static str = "searchium.v2.SearchiumService";
    }
}
