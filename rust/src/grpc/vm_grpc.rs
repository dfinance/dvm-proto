/// An `AbortLocation` specifies where a Move program `abort` occurred, either in a function in
/// a module, or in a script.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbortLocation {
    /// Indicates `abort` occurred in the specified module.
    #[prost(bytes, tag = "1")]
    pub address: std::vec::Vec<u8>,
    /// Indicates the `abort` occurred in a script.
    #[prost(string, tag = "2")]
    pub module: std::string::String,
}
/// Function location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionLoc {
    /// Function index.
    #[prost(uint64, tag = "1")]
    pub function: u64,
    /// Code offset.
    #[prost(uint64, tag = "2")]
    pub code_offset: u64,
}
/// VmStatus `Error` case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveError {
    /// Status code.
    #[prost(uint64, tag = "2")]
    pub status_code: u64,
}
/// VmStatus `MoveAbort` case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Abort {
    /// Abort location. (optional). Null if abort occurred in the script.
    #[prost(message, optional, tag = "1")]
    pub abort_location: ::std::option::Option<AbortLocation>,
    /// Abort code.
    #[prost(uint64, tag = "2")]
    pub abort_code: u64,
}
/// VmStatus `ExecutionFailure` case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Failure {
    /// Status code.
    #[prost(uint64, tag = "1")]
    pub status_code: u64,
    /// Abort location. (optional). Null if abort occurred in the script.
    #[prost(message, optional, tag = "2")]
    pub abort_location: ::std::option::Option<AbortLocation>,
    /// Function location.
    #[prost(message, optional, tag = "3")]
    pub function_loc: ::std::option::Option<FunctionLoc>,
}
//// Message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// Message with error details if needed.
    #[prost(string, tag = "1")]
    pub text: std::string::String,
}
/// A `VMStatus` is represented as either
/// - `Null` indicating successful execution.
/// - `Error` indicating an error from the VM itself.
/// - `MoveAbort` indicating an `abort` ocurred inside of a Move program
/// - `ExecutionFailure` indicating an runtime error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmStatus {
    /// Message with error details if needed (optional).
    #[prost(message, optional, tag = "4")]
    pub message: ::std::option::Option<Message>,
    #[prost(oneof = "vm_status::Error", tags = "1, 2, 3")]
    pub error: ::std::option::Option<vm_status::Error>,
}
pub mod vm_status {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Error {
        /// Indicates an error from the VM, e.g. OUT_OF_GAS, INVALID_AUTH_KEY, RET_TYPE_MISMATCH_ERROR
        /// etc.
        /// The code will neither EXECUTED nor ABORTED
        #[prost(message, tag = "1")]
        MoveError(super::MoveError),
        /// Indicates an error from the VM, e.g. OUT_OF_GAS, INVALID_AUTH_KEY, RET_TYPE_MISMATCH_ERROR
        /// etc.
        /// The code will neither EXECUTED nor ABORTED
        #[prost(message, tag = "2")]
        Abort(super::Abort),
        /// Indicates an failure from inside Move code, where the VM could not continue exection, e.g.
        /// dividing by zero or a missing resource
        #[prost(message, tag = "3")]
        ExecutionFailure(super::Failure),
    }
}
//// Full name of the structure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructIdent {
    /// address of module owner
    #[prost(bytes, tag = "1")]
    pub address: std::vec::Vec<u8>,
    /// module name.
    #[prost(string, tag = "2")]
    pub module: std::string::String,
    /// name of structure.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// Structure type parameters.
    #[prost(message, repeated, tag = "4")]
    pub type_params: ::std::vec::Vec<LcsTag>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LcsTag {
    /// type tag.
    #[prost(enumeration = "LcsType", tag = "1")]
    pub type_tag: i32,
    /// vector type. Has a non-null value if the type_tag is equal to a LcsVector.
    #[prost(message, optional, boxed, tag = "2")]
    pub vector_type: ::std::option::Option<::std::boxed::Box<LcsTag>>,
    /// struct identifier. Has a non-null value if the type_tag is equal to a LcsStruct.
    #[prost(message, optional, tag = "3")]
    pub struct_ident: ::std::option::Option<StructIdent>,
}
//// Module identifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleIdent {
    /// module address.
    #[prost(bytes, tag = "1")]
    pub address: std::vec::Vec<u8>,
    /// module name.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
/// VM event returns after contract execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmEvent {
    /// Event sender address.
    #[prost(bytes, tag = "1")]
    pub sender_address: std::vec::Vec<u8>,
    /// sender module.
    #[prost(message, optional, tag = "2")]
    pub sender_module: ::std::option::Option<ModuleIdent>,
    /// Type of value inside event.
    #[prost(message, optional, tag = "3")]
    pub event_type: ::std::option::Option<LcsTag>,
    /// Event data in bytes to parse.
    #[prost(bytes, tag = "4")]
    pub event_data: std::vec::Vec<u8>,
}
/// Storage path
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmAccessPath {
    /// account address.
    #[prost(bytes, tag = "1")]
    pub address: std::vec::Vec<u8>,
    /// storage path.
    #[prost(bytes, tag = "2")]
    pub path: std::vec::Vec<u8>,
}
/// VM value should be passed before execution and return after execution (with opcodes), write_set in nutshell.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmValue {
    /// Type of operation
    #[prost(enumeration = "VmWriteOp", tag = "2")]
    pub r#type: i32,
    /// Value returns from vm.
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
    /// Access path.
    #[prost(message, optional, tag = "3")]
    pub path: ::std::option::Option<VmAccessPath>,
}
/// Contract arguments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmArgs {
    /// Argument type.
    #[prost(enumeration = "super::types::VmTypeTag", tag = "1")]
    pub r#type: i32,
    /// Argument value.
    #[prost(bytes, tag = "2")]
    pub value: std::vec::Vec<u8>,
}
/// Publish module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmPublishModule {
    /// owner of contract.
    #[prost(bytes, tag = "1")]
    pub sender: std::vec::Vec<u8>,
    /// maximal total gas specified by wallet to spend for this transaction.
    #[prost(uint64, tag = "2")]
    pub max_gas_amount: u64,
    /// maximal price can be paid per gas.
    #[prost(uint64, tag = "3")]
    pub gas_unit_price: u64,
    /// compiled contract code.
    #[prost(bytes, tag = "4")]
    pub code: std::vec::Vec<u8>,
}
/// VM contract object to process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmExecuteScript {
    /// owners of contract.
    #[prost(bytes, repeated, tag = "1")]
    pub senders: ::std::vec::Vec<std::vec::Vec<u8>>,
    /// maximal total gas specified by wallet to spend for this transaction.
    #[prost(uint64, tag = "2")]
    pub max_gas_amount: u64,
    /// maximal price can be paid per gas.
    #[prost(uint64, tag = "3")]
    pub gas_unit_price: u64,
    /// block.
    #[prost(uint64, tag = "4")]
    pub block: u64,
    /// timestamp.
    #[prost(uint64, tag = "5")]
    pub timestamp: u64,
    /// compiled contract code.
    #[prost(bytes, tag = "6")]
    pub code: std::vec::Vec<u8>,
    /// type parameters.
    #[prost(message, repeated, tag = "7")]
    pub type_params: ::std::vec::Vec<StructIdent>,
    /// Contract arguments.
    #[prost(message, repeated, tag = "8")]
    pub args: ::std::vec::Vec<VmArgs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmBalanceChange {
    #[prost(oneof = "vm_balance_change::Op", tags = "1, 2")]
    pub op: ::std::option::Option<vm_balance_change::Op>,
}
pub mod vm_balance_change {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Op {
        /// Little-endian unsigned 128.
        #[prost(bytes, tag = "1")]
        Deposit(std::vec::Vec<u8>),
        /// Little-endian unsigned 128.
        #[prost(bytes, tag = "2")]
        Withdraw(std::vec::Vec<u8>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmBalanceChangeSet {
    #[prost(message, repeated, tag = "1")]
    pub change_set: ::std::vec::Vec<VmBalanceChange>,
}
/// Response from VM contains write_set, events, gas used and status for specific contract.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmExecuteResponse {
    /// using string instead of bytes for now, as map support only ints and strings as keys
    #[prost(message, repeated, tag = "1")]
    pub write_set: ::std::vec::Vec<VmValue>,
    /// list of events executed during contract execution
    #[prost(message, repeated, tag = "2")]
    pub events: ::std::vec::Vec<VmEvent>,
    /// list of native balance updates.
    #[prost(message, repeated, tag = "3")]
    pub balance_change_set: ::std::vec::Vec<VmBalanceChange>,
    /// Gas used during execution.
    #[prost(uint64, tag = "4")]
    pub gas_used: u64,
    /// Main status of execution, might contain an error.
    #[prost(message, optional, tag = "5")]
    pub status: ::std::option::Option<VmStatus>,
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
pub struct MultipleCompilationResult {
    #[prost(message, repeated, tag = "1")]
    pub units: ::std::vec::Vec<CompiledUnit>,
    /// list of error messages, empty if successful
    #[prost(string, repeated, tag = "2")]
    pub errors: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LcsType {
    /// Bool
    LcsBool = 0,
    /// Uint64
    LcsU64 = 1,
    /// Vector of bytes.
    LcsVector = 2,
    /// Address, in bech32 form
    LcsAddress = 3,
    /// U8
    LcsU8 = 4,
    /// U128
    LcsU128 = 5,
    /// Signer.
    LcsSigner = 6,
    /// Struct.
    LcsStruct = 7,
}
/// Write set operation type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VmWriteOp {
    /// Insert or update value
    Value = 0,
    /// Delete.
    Deletion = 1,
}
#[doc = r" Generated client implementations."]
pub mod vm_module_publisher_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " GRPC service"]
    pub struct VmModulePublisherClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VmModulePublisherClient<tonic::transport::Channel> {
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
    impl<T> VmModulePublisherClient<T>
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
        pub async fn publish_module(
            &mut self,
            request: impl tonic::IntoRequest<super::VmPublishModule>,
        ) -> Result<tonic::Response<super::VmExecuteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/vm_grpc.VMModulePublisher/PublishModule");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VmModulePublisherClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VmModulePublisherClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VmModulePublisherClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod vm_script_executor_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct VmScriptExecutorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VmScriptExecutorClient<tonic::transport::Channel> {
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
    impl<T> VmScriptExecutorClient<T>
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
        pub async fn execute_script(
            &mut self,
            request: impl tonic::IntoRequest<super::VmExecuteScript>,
        ) -> Result<tonic::Response<super::VmExecuteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/vm_grpc.VMScriptExecutor/ExecuteScript");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VmScriptExecutorClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VmScriptExecutorClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VmScriptExecutorClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod vm_module_publisher_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with VmModulePublisherServer."]
    #[async_trait]
    pub trait VmModulePublisher: Send + Sync + 'static {
        async fn publish_module(
            &self,
            request: tonic::Request<super::VmPublishModule>,
        ) -> Result<tonic::Response<super::VmExecuteResponse>, tonic::Status>;
    }
    #[doc = " GRPC service"]
    #[derive(Debug)]
    pub struct VmModulePublisherServer<T: VmModulePublisher> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: VmModulePublisher> VmModulePublisherServer<T> {
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
    impl<T, B> Service<http::Request<B>> for VmModulePublisherServer<T>
    where
        T: VmModulePublisher,
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
                "/vm_grpc.VMModulePublisher/PublishModule" => {
                    #[allow(non_camel_case_types)]
                    struct PublishModuleSvc<T: VmModulePublisher>(pub Arc<T>);
                    impl<T: VmModulePublisher> tonic::server::UnaryService<super::VmPublishModule>
                        for PublishModuleSvc<T>
                    {
                        type Response = super::VmExecuteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VmPublishModule>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).publish_module(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PublishModuleSvc(inner);
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
    impl<T: VmModulePublisher> Clone for VmModulePublisherServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: VmModulePublisher> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VmModulePublisher> tonic::transport::NamedService for VmModulePublisherServer<T> {
        const NAME: &'static str = "vm_grpc.VMModulePublisher";
    }
}
#[doc = r" Generated server implementations."]
pub mod vm_script_executor_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with VmScriptExecutorServer."]
    #[async_trait]
    pub trait VmScriptExecutor: Send + Sync + 'static {
        async fn execute_script(
            &self,
            request: tonic::Request<super::VmExecuteScript>,
        ) -> Result<tonic::Response<super::VmExecuteResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct VmScriptExecutorServer<T: VmScriptExecutor> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: VmScriptExecutor> VmScriptExecutorServer<T> {
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
    impl<T, B> Service<http::Request<B>> for VmScriptExecutorServer<T>
    where
        T: VmScriptExecutor,
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
                "/vm_grpc.VMScriptExecutor/ExecuteScript" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteScriptSvc<T: VmScriptExecutor>(pub Arc<T>);
                    impl<T: VmScriptExecutor> tonic::server::UnaryService<super::VmExecuteScript>
                        for ExecuteScriptSvc<T>
                    {
                        type Response = super::VmExecuteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VmExecuteScript>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).execute_script(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ExecuteScriptSvc(inner);
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
    impl<T: VmScriptExecutor> Clone for VmScriptExecutorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: VmScriptExecutor> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VmScriptExecutor> tonic::transport::NamedService for VmScriptExecutorServer<T> {
        const NAME: &'static str = "vm_grpc.VMScriptExecutor";
    }
}
