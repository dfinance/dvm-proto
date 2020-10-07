/// Compilation unit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompilationUnit {
    /// utf8 encoded source code with libra/bech32 addresses
    #[prost(string, tag = "1")]
    pub text: std::string::String,
    /// name of the unit.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
/// Compiler API
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceFiles {
    /// Compilation units.
    #[prost(message, repeated, tag = "1")]
    pub units: ::std::vec::Vec<CompilationUnit>,
    /// address of the sender, in bech32 form
    #[prost(bytes, tag = "2")]
    pub address: std::vec::Vec<u8>,
}
/// Compiled source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompiledUnit {
    /// name of the module/script.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// bytecode of the compiled module/script
    #[prost(bytes, tag = "2")]
    pub bytecode: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompilationResult {
    #[prost(message, repeated, tag = "1")]
    pub units: ::std::vec::Vec<CompiledUnit>,
    /// list of error messages, empty if successful
    #[prost(string, repeated, tag = "2")]
    pub errors: ::std::vec::Vec<std::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod dvm_compiler_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct DvmCompilerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DvmCompilerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DvmCompilerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn compile(
            &mut self,
            request: impl tonic::IntoRequest<super::SourceFiles>,
        ) -> Result<tonic::Response<super::CompilationResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/compiler_grpc.DvmCompiler/Compile");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DvmCompilerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DvmCompilerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DvmCompilerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod dvm_compiler_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DvmCompilerServer."]
    #[async_trait]
    pub trait DvmCompiler: Send + Sync + 'static {
        async fn compile(
            &self,
            request: tonic::Request<super::SourceFiles>,
        ) -> Result<tonic::Response<super::CompilationResult>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DvmCompilerServer<T: DvmCompiler> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DvmCompiler> DvmCompilerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for DvmCompilerServer<T>
    where
        T: DvmCompiler,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/compiler_grpc.DvmCompiler/Compile" => {
                    #[allow(non_camel_case_types)]
                    struct CompileSvc<T: DvmCompiler>(pub Arc<T>);
                    impl<T: DvmCompiler> tonic::server::UnaryService<super::SourceFiles> for CompileSvc<T> {
                        type Response = super::CompilationResult;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SourceFiles>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).compile(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CompileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DvmCompiler> Clone for DvmCompilerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DvmCompiler> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DvmCompiler> tonic::transport::NamedService for DvmCompilerServer<T> {
        const NAME: &'static str = "compiler_grpc.DvmCompiler";
    }
}
