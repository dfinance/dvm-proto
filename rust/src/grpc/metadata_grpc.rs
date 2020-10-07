/// Bytecode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bytecode {
    /// bytecode of script
    #[prost(bytes, tag = "1")]
    pub code: std::vec::Vec<u8>,
}
/// Struct field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(string, tag = "2")]
    pub r#type: std::string::String,
}
//// Struct representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(bool, tag = "2")]
    pub is_resource: bool,
    #[prost(string, repeated, tag = "3")]
    pub type_parameters: ::std::vec::Vec<std::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub field: ::std::vec::Vec<Field>,
}
//// Function representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Function {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(bool, tag = "2")]
    pub is_public: bool,
    #[prost(bool, tag = "3")]
    pub is_native: bool,
    #[prost(string, repeated, tag = "4")]
    pub type_parameters: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub arguments: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub returns: ::std::vec::Vec<std::string::String>,
}
/// Script metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptMeta {
    /// Script type parameters.
    #[prost(string, repeated, tag = "1")]
    pub type_parameters: ::std::vec::Vec<std::string::String>,
    /// Script arguments.
    #[prost(enumeration = "super::types::VmTypeTag", repeated, tag = "2")]
    pub arguments: ::std::vec::Vec<i32>,
}
/// Module metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleMeta {
    /// module name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Types defined in a module.
    #[prost(message, repeated, tag = "2")]
    pub types: ::std::vec::Vec<Struct>,
    /// Functions defined in a module.
    #[prost(message, repeated, tag = "3")]
    pub functions: ::std::vec::Vec<Function>,
}
/// Bytecode metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(oneof = "metadata::Meta", tags = "1, 2")]
    pub meta: ::std::option::Option<metadata::Meta>,
}
pub mod metadata {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Meta {
        /// In case the provided bytecode is a script.
        #[prost(message, tag = "1")]
        Script(super::ScriptMeta),
        /// In case the provided bytecode is a module.
        #[prost(message, tag = "2")]
        Module(super::ModuleMeta),
    }
}
#[doc = r" Generated client implementations."]
pub mod dvm_bytecode_metadata_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Returns bytecode metadata."]
    pub struct DvmBytecodeMetadataClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DvmBytecodeMetadataClient<tonic::transport::Channel> {
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
    impl<T> DvmBytecodeMetadataClient<T>
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
        pub async fn get_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::Bytecode>,
        ) -> Result<tonic::Response<super::Metadata>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/metadata_grpc.DVMBytecodeMetadata/GetMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DvmBytecodeMetadataClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DvmBytecodeMetadataClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DvmBytecodeMetadataClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod dvm_bytecode_metadata_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DvmBytecodeMetadataServer."]
    #[async_trait]
    pub trait DvmBytecodeMetadata: Send + Sync + 'static {
        async fn get_metadata(
            &self,
            request: tonic::Request<super::Bytecode>,
        ) -> Result<tonic::Response<super::Metadata>, tonic::Status>;
    }
    #[doc = " Returns bytecode metadata."]
    #[derive(Debug)]
    pub struct DvmBytecodeMetadataServer<T: DvmBytecodeMetadata> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DvmBytecodeMetadata> DvmBytecodeMetadataServer<T> {
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
    impl<T, B> Service<http::Request<B>> for DvmBytecodeMetadataServer<T>
    where
        T: DvmBytecodeMetadata,
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
                "/metadata_grpc.DVMBytecodeMetadata/GetMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct GetMetadataSvc<T: DvmBytecodeMetadata>(pub Arc<T>);
                    impl<T: DvmBytecodeMetadata> tonic::server::UnaryService<super::Bytecode> for GetMetadataSvc<T> {
                        type Response = super::Metadata;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Bytecode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_metadata(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetMetadataSvc(inner);
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
    impl<T: DvmBytecodeMetadata> Clone for DvmBytecodeMetadataServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DvmBytecodeMetadata> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DvmBytecodeMetadata> tonic::transport::NamedService for DvmBytecodeMetadataServer<T> {
        const NAME: &'static str = "metadata_grpc.DVMBytecodeMetadata";
    }
}
