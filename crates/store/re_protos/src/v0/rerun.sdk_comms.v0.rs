// This file is @generated by prost-build.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WriteMessagesResponse {}
impl ::prost::Name for WriteMessagesResponse {
    const NAME: &'static str = "WriteMessagesResponse";
    const PACKAGE: &'static str = "rerun.sdk_comms.v0";
    fn full_name() -> ::prost::alloc::string::String {
        "rerun.sdk_comms.v0.WriteMessagesResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/rerun.sdk_comms.v0.WriteMessagesResponse".into()
    }
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ReadMessagesRequest {}
impl ::prost::Name for ReadMessagesRequest {
    const NAME: &'static str = "ReadMessagesRequest";
    const PACKAGE: &'static str = "rerun.sdk_comms.v0";
    fn full_name() -> ::prost::alloc::string::String {
        "rerun.sdk_comms.v0.ReadMessagesRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/rerun.sdk_comms.v0.ReadMessagesRequest".into()
    }
}
/// Generated client implementations.
pub mod message_proxy_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Simple buffer for messages between SDKs and viewers.
    ///
    /// - SDKs produce messages by calling `WriteMessages`
    /// - Viewers consume messages by calling `ReadMessages`
    ///
    /// The buffer is bounded by a memory limit, and will drop the oldest messages when the limit is reached.
    ///
    /// Whenever `ReadMessages` is called, all buffered messages are sent in the order they were received.
    /// The stream will then also yield any new messages passed to `WriteMessages` from any client.
    #[derive(Debug, Clone)]
    pub struct MessageProxyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MessageProxyClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> MessageProxyClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            MessageProxyClient::new(InterceptedService::new(inner, interceptor))
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
        /// TODO(jan): Would it be more efficient to send a "message batch" instead of individual messages?
        ///            It may allow us to amortize the overhead of the gRPC protocol.
        pub async fn write_messages(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::log_msg::v0::LogMsg,
            >,
        ) -> std::result::Result<tonic::Response<super::WriteMessagesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.sdk_comms.v0.MessageProxy/WriteMessages",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.sdk_comms.v0.MessageProxy",
                "WriteMessages",
            ));
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn read_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadMessagesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::super::super::log_msg::v0::LogMsg>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.sdk_comms.v0.MessageProxy/ReadMessages",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.sdk_comms.v0.MessageProxy",
                "ReadMessages",
            ));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod message_proxy_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MessageProxyServer.
    #[async_trait]
    pub trait MessageProxy: std::marker::Send + std::marker::Sync + 'static {
        /// TODO(jan): Would it be more efficient to send a "message batch" instead of individual messages?
        ///            It may allow us to amortize the overhead of the gRPC protocol.
        async fn write_messages(
            &self,
            request: tonic::Request<tonic::Streaming<super::super::super::log_msg::v0::LogMsg>>,
        ) -> std::result::Result<tonic::Response<super::WriteMessagesResponse>, tonic::Status>;
        /// Server streaming response type for the ReadMessages method.
        type ReadMessagesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::super::super::log_msg::v0::LogMsg, tonic::Status>,
            > + std::marker::Send
            + 'static;
        async fn read_messages(
            &self,
            request: tonic::Request<super::ReadMessagesRequest>,
        ) -> std::result::Result<tonic::Response<Self::ReadMessagesStream>, tonic::Status>;
    }
    /// Simple buffer for messages between SDKs and viewers.
    ///
    /// - SDKs produce messages by calling `WriteMessages`
    /// - Viewers consume messages by calling `ReadMessages`
    ///
    /// The buffer is bounded by a memory limit, and will drop the oldest messages when the limit is reached.
    ///
    /// Whenever `ReadMessages` is called, all buffered messages are sent in the order they were received.
    /// The stream will then also yield any new messages passed to `WriteMessages` from any client.
    #[derive(Debug)]
    pub struct MessageProxyServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> MessageProxyServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MessageProxyServer<T>
    where
        T: MessageProxy,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
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
            match req.uri().path() {
                "/rerun.sdk_comms.v0.MessageProxy/WriteMessages" => {
                    #[allow(non_camel_case_types)]
                    struct WriteMessagesSvc<T: MessageProxy>(pub Arc<T>);
                    impl<T: MessageProxy>
                        tonic::server::ClientStreamingService<
                            super::super::super::log_msg::v0::LogMsg,
                        > for WriteMessagesSvc<T>
                    {
                        type Response = super::WriteMessagesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::log_msg::v0::LogMsg>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageProxy>::write_messages(&inner, request).await
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
                        let method = WriteMessagesSvc(inner);
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
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rerun.sdk_comms.v0.MessageProxy/ReadMessages" => {
                    #[allow(non_camel_case_types)]
                    struct ReadMessagesSvc<T: MessageProxy>(pub Arc<T>);
                    impl<T: MessageProxy>
                        tonic::server::ServerStreamingService<super::ReadMessagesRequest>
                        for ReadMessagesSvc<T>
                    {
                        type Response = super::super::super::log_msg::v0::LogMsg;
                        type ResponseStream = T::ReadMessagesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadMessagesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageProxy>::read_messages(&inner, request).await
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
                        let method = ReadMessagesSvc(inner);
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
                _ => Box::pin(async move {
                    let mut response = http::Response::new(empty_body());
                    let headers = response.headers_mut();
                    headers.insert(
                        tonic::Status::GRPC_STATUS,
                        (tonic::Code::Unimplemented as i32).into(),
                    );
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        tonic::metadata::GRPC_CONTENT_TYPE,
                    );
                    Ok(response)
                }),
            }
        }
    }
    impl<T> Clone for MessageProxyServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "rerun.sdk_comms.v0.MessageProxy";
    impl<T> tonic::server::NamedService for MessageProxyServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
