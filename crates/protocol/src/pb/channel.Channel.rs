/// Generated client implementations.
pub mod channel_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ChannelClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChannelClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ChannelClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ChannelClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ChannelClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn download_media(
            &mut self,
            request: impl tonic::IntoRequest<crate::channel::DownloadMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<crate::DownloadProgressItem>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = crate::json_codec::JsonCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/channel.Channel/DownloadMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("channel.Channel", "DownloadMedia"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn get_media_metadata(
            &mut self,
            request: impl tonic::IntoRequest<crate::channel::GetMediaMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<crate::channel::MediaMetadata>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = crate::json_codec::JsonCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/channel.Channel/GetMediaMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("channel.Channel", "GetMediaMetadata"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn search_media(
            &mut self,
            request: impl tonic::IntoRequest<crate::channel::SearchMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<crate::channel::SearchMediaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = crate::json_codec::JsonCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/channel.Channel/SearchMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("channel.Channel", "SearchMedia"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_media_playlist(
            &mut self,
            request: impl tonic::IntoRequest<crate::channel::GetMediaPlaylistRequest>,
        ) -> std::result::Result<
            tonic::Response<crate::channel::MediaPlaylist>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = crate::json_codec::JsonCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/channel.Channel/GetMediaPlaylist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("channel.Channel", "GetMediaPlaylist"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod channel_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ChannelServer.
    #[async_trait]
    pub trait Channel: Send + Sync + 'static {
        /// Server streaming response type for the DownloadMedia method.
        type DownloadMediaStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<crate::DownloadProgressItem, tonic::Status>,
            >
            + Send
            + 'static;
        async fn download_media(
            &self,
            request: tonic::Request<crate::channel::DownloadMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::DownloadMediaStream>,
            tonic::Status,
        >;
        async fn get_media_metadata(
            &self,
            request: tonic::Request<crate::channel::GetMediaMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<crate::channel::MediaMetadata>,
            tonic::Status,
        >;
        async fn search_media(
            &self,
            request: tonic::Request<crate::channel::SearchMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<crate::channel::SearchMediaResponse>,
            tonic::Status,
        >;
        async fn get_media_playlist(
            &self,
            request: tonic::Request<crate::channel::GetMediaPlaylistRequest>,
        ) -> std::result::Result<
            tonic::Response<crate::channel::MediaPlaylist>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ChannelServer<T: Channel> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Channel> ChannelServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ChannelServer<T>
    where
        T: Channel,
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
                "/channel.Channel/DownloadMedia" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadMediaSvc<T: Channel>(pub Arc<T>);
                    impl<
                        T: Channel,
                    > tonic::server::ServerStreamingService<
                        crate::channel::DownloadMediaRequest,
                    > for DownloadMediaSvc<T> {
                        type Response = crate::DownloadProgressItem;
                        type ResponseStream = T::DownloadMediaStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<crate::channel::DownloadMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Channel>::download_media(&inner, request).await
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
                        let method = DownloadMediaSvc(inner);
                        let codec = crate::json_codec::JsonCodec::default();
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
                "/channel.Channel/GetMediaMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct GetMediaMetadataSvc<T: Channel>(pub Arc<T>);
                    impl<
                        T: Channel,
                    > tonic::server::UnaryService<
                        crate::channel::GetMediaMetadataRequest,
                    > for GetMediaMetadataSvc<T> {
                        type Response = crate::channel::MediaMetadata;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                crate::channel::GetMediaMetadataRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Channel>::get_media_metadata(&inner, request).await
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
                        let method = GetMediaMetadataSvc(inner);
                        let codec = crate::json_codec::JsonCodec::default();
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
                "/channel.Channel/SearchMedia" => {
                    #[allow(non_camel_case_types)]
                    struct SearchMediaSvc<T: Channel>(pub Arc<T>);
                    impl<
                        T: Channel,
                    > tonic::server::UnaryService<crate::channel::SearchMediaRequest>
                    for SearchMediaSvc<T> {
                        type Response = crate::channel::SearchMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<crate::channel::SearchMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Channel>::search_media(&inner, request).await
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
                        let method = SearchMediaSvc(inner);
                        let codec = crate::json_codec::JsonCodec::default();
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
                "/channel.Channel/GetMediaPlaylist" => {
                    #[allow(non_camel_case_types)]
                    struct GetMediaPlaylistSvc<T: Channel>(pub Arc<T>);
                    impl<
                        T: Channel,
                    > tonic::server::UnaryService<
                        crate::channel::GetMediaPlaylistRequest,
                    > for GetMediaPlaylistSvc<T> {
                        type Response = crate::channel::MediaPlaylist;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                crate::channel::GetMediaPlaylistRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Channel>::get_media_playlist(&inner, request).await
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
                        let method = GetMediaPlaylistSvc(inner);
                        let codec = crate::json_codec::JsonCodec::default();
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
    impl<T: Channel> Clone for ChannelServer<T> {
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
    impl<T: Channel> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Channel> tonic::server::NamedService for ChannelServer<T> {
        const NAME: &'static str = "channel.Channel";
    }
}
