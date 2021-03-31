#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsAccessPath {
    /// AccountAddress
    #[prost(bytes, tag = "1")]
    pub address: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub path: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsRawResponse {
    #[prost(bytes, tag = "1")]
    pub blob: std::vec::Vec<u8>,
    #[prost(enumeration = "ErrorCode", tag = "2")]
    pub error_code: i32,
    /// error message from libra, empty if ErrorCode::None
    #[prost(string, tag = "3")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsAccessPaths {
    #[prost(message, repeated, tag = "1")]
    pub paths: ::std::vec::Vec<DsAccessPath>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsRawResponses {
    #[prost(bytes, repeated, tag = "1")]
    pub blobs: ::std::vec::Vec<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OraclePriceRequest {
    #[prost(string, tag = "1")]
    pub currency_1: std::string::String,
    #[prost(string, tag = "2")]
    pub currency_2: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OraclePriceResponse {
    #[prost(message, optional, tag = "1")]
    pub price: ::std::option::Option<super::types::U128>,
    #[prost(enumeration = "ErrorCode", tag = "2")]
    pub error_code: i32,
    /// error message from libra, empty if ErrorCode::None
    #[prost(string, tag = "3")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeBalanceRequest {
    #[prost(bytes, tag = "1")]
    pub address: std::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub ticker: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeBalanceResponse {
    #[prost(message, optional, tag = "1")]
    pub price: ::std::option::Option<super::types::U128>,
    #[prost(enumeration = "ErrorCode", tag = "2")]
    pub error_code: i32,
    /// error message from libra, empty if ErrorCode::None
    #[prost(string, tag = "3")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyInfoRequest {
    #[prost(string, tag = "2")]
    pub ticker: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyInfo {
    #[prost(bytes, tag = "1")]
    pub denom: std::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub decimals: u32,
    #[prost(bool, tag = "3")]
    pub is_token: bool,
    #[prost(bytes, tag = "4")]
    pub address: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub total_supply: ::std::option::Option<super::types::U128>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub info: ::std::option::Option<CurrencyInfo>,
    #[prost(enumeration = "ErrorCode", tag = "2")]
    pub error_code: i32,
    /// error message from libra, empty if ErrorCode::None
    #[prost(string, tag = "3")]
    pub error_message: std::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorCode {
    /// no error
    None = 0,
    /// crash of compilation, logs will show stacktrace
    BadRequest = 1,
    /// no such module
    NoData = 2,
}
#[doc = r" Generated client implementations."]
pub mod ds_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " GRPC service"]
    pub struct DsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsServiceClient<tonic::transport::Channel> {
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
    impl<T> DsServiceClient<T>
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
        pub async fn get_raw(
            &mut self,
            request: impl tonic::IntoRequest<super::DsAccessPath>,
        ) -> Result<tonic::Response<super::DsRawResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_grpc.DSService/GetRaw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn multi_get_raw(
            &mut self,
            request: impl tonic::IntoRequest<super::DsAccessPaths>,
        ) -> Result<tonic::Response<super::DsRawResponses>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_grpc.DSService/MultiGetRaw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_oracle_price(
            &mut self,
            request: impl tonic::IntoRequest<super::OraclePriceRequest>,
        ) -> Result<tonic::Response<super::OraclePriceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_grpc.DSService/GetOraclePrice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_native_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::NativeBalanceRequest>,
        ) -> Result<tonic::Response<super::NativeBalanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_grpc.DSService/GetNativeBalance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_currency_info(
            &mut self,
            request: impl tonic::IntoRequest<super::CurrencyInfoRequest>,
        ) -> Result<tonic::Response<super::CurrencyInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_grpc.DSService/GetCurrencyInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DsServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod ds_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DsServiceServer."]
    #[async_trait]
    pub trait DsService: Send + Sync + 'static {
        async fn get_raw(
            &self,
            request: tonic::Request<super::DsAccessPath>,
        ) -> Result<tonic::Response<super::DsRawResponse>, tonic::Status>;
        async fn multi_get_raw(
            &self,
            request: tonic::Request<super::DsAccessPaths>,
        ) -> Result<tonic::Response<super::DsRawResponses>, tonic::Status>;
        async fn get_oracle_price(
            &self,
            request: tonic::Request<super::OraclePriceRequest>,
        ) -> Result<tonic::Response<super::OraclePriceResponse>, tonic::Status>;
        async fn get_native_balance(
            &self,
            request: tonic::Request<super::NativeBalanceRequest>,
        ) -> Result<tonic::Response<super::NativeBalanceResponse>, tonic::Status>;
        async fn get_currency_info(
            &self,
            request: tonic::Request<super::CurrencyInfoRequest>,
        ) -> Result<tonic::Response<super::CurrencyInfoResponse>, tonic::Status>;
    }
    #[doc = " GRPC service"]
    #[derive(Debug)]
    pub struct DsServiceServer<T: DsService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DsService> DsServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for DsServiceServer<T>
    where
        T: DsService,
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
                "/ds_grpc.DSService/GetRaw" => {
                    #[allow(non_camel_case_types)]
                    struct GetRawSvc<T: DsService>(pub Arc<T>);
                    impl<T: DsService> tonic::server::UnaryService<super::DsAccessPath> for GetRawSvc<T> {
                        type Response = super::DsRawResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DsAccessPath>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_raw(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetRawSvc(inner);
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
                "/ds_grpc.DSService/MultiGetRaw" => {
                    #[allow(non_camel_case_types)]
                    struct MultiGetRawSvc<T: DsService>(pub Arc<T>);
                    impl<T: DsService> tonic::server::UnaryService<super::DsAccessPaths> for MultiGetRawSvc<T> {
                        type Response = super::DsRawResponses;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DsAccessPaths>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_get_raw(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MultiGetRawSvc(inner);
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
                "/ds_grpc.DSService/GetOraclePrice" => {
                    #[allow(non_camel_case_types)]
                    struct GetOraclePriceSvc<T: DsService>(pub Arc<T>);
                    impl<T: DsService> tonic::server::UnaryService<super::OraclePriceRequest> for GetOraclePriceSvc<T> {
                        type Response = super::OraclePriceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OraclePriceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_oracle_price(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetOraclePriceSvc(inner);
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
                "/ds_grpc.DSService/GetNativeBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GetNativeBalanceSvc<T: DsService>(pub Arc<T>);
                    impl<T: DsService> tonic::server::UnaryService<super::NativeBalanceRequest>
                        for GetNativeBalanceSvc<T>
                    {
                        type Response = super::NativeBalanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NativeBalanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_native_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetNativeBalanceSvc(inner);
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
                "/ds_grpc.DSService/GetCurrencyInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetCurrencyInfoSvc<T: DsService>(pub Arc<T>);
                    impl<T: DsService> tonic::server::UnaryService<super::CurrencyInfoRequest>
                        for GetCurrencyInfoSvc<T>
                    {
                        type Response = super::CurrencyInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CurrencyInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_currency_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetCurrencyInfoSvc(inner);
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
    impl<T: DsService> Clone for DsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsService> tonic::transport::NamedService for DsServiceServer<T> {
        const NAME: &'static str = "ds_grpc.DSService";
    }
}
