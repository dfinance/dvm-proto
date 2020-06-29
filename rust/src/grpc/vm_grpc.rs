/// Status of code contract execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmStatus {
    /// Major status.
    #[prost(uint64, tag = "1")]
    pub major_status: u64,
    /// Sub status if needed (optional).
    #[prost(uint64, tag = "2")]
    pub sub_status: u64,
    /// Message with error details if needed (optional).
    #[prost(string, tag = "3")]
    pub message: std::string::String,
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
    #[prost(enumeration = "VmTypeTag", tag = "1")]
    pub r#type: i32,
    /// Argument value.
    #[prost(bytes, tag = "2")]
    pub value: std::vec::Vec<u8>,
}
/// Publish module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmPublishModule {
    /// owner of contract (module) or script executor.
    #[prost(bytes, tag = "1")]
    pub address: std::vec::Vec<u8>,
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
    /// owner of contract (module) or script executor.
    #[prost(bytes, tag = "1")]
    pub address: std::vec::Vec<u8>,
    /// maximal total gas specified by wallet to spend for this transaction.
    #[prost(uint64, tag = "2")]
    pub max_gas_amount: u64,
    /// maximal price can be paid per gas.
    #[prost(uint64, tag = "3")]
    pub gas_unit_price: u64,
    /// compiled contract code.
    #[prost(bytes, tag = "4")]
    pub code: std::vec::Vec<u8>,
    /// type parameters.
    #[prost(message, repeated, tag = "7")]
    pub type_params: ::std::vec::Vec<StructIdent>,
    /// Contract arguments.
    #[prost(message, repeated, tag = "8")]
    pub args: ::std::vec::Vec<VmArgs>,
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
    /// Gas used during execution.
    #[prost(uint64, tag = "3")]
    pub gas_used: u64,
    /// Status of contract execution.
    #[prost(enumeration = "ContractStatus", tag = "4")]
    pub status: i32,
    /// Main status of execution, might contain an error.
    #[prost(message, optional, tag = "5")]
    pub status_struct: ::std::option::Option<VmStatus>,
}
/// Compiler API
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceFile {
    /// utf8 encoded source code with libra/bech32 addresses
    #[prost(string, tag = "1")]
    pub text: std::string::String,
    /// address of the sender, in bech32 form
    #[prost(bytes, tag = "2")]
    pub address: std::vec::Vec<u8>,
}
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
pub struct MultipleCompilationResult {
    #[prost(message, repeated, tag = "1")]
    pub units: ::std::vec::Vec<CompiledUnit>,
    /// list of error messages, empty if successful
    #[prost(string, repeated, tag = "2")]
    pub errors: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompilationResult {
    /// bytecode of the compiled module/script
    #[prost(bytes, tag = "1")]
    pub bytecode: std::vec::Vec<u8>,
    /// list of error messages, empty if successful
    #[prost(string, repeated, tag = "2")]
    pub errors: ::std::vec::Vec<std::string::String>,
}
/// Script Arguments API
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmScript {
    /// bytecode of script
    #[prost(bytes, tag = "1")]
    pub code: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(enumeration = "VmTypeTag", repeated, tag = "1")]
    pub arguments: ::std::vec::Vec<i32>,
}
//// Resource access path.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessVector {
    #[prost(bytes, tag = "1")]
    pub access_vector: std::vec::Vec<u8>,
}
/// Status of contract execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractStatus {
    /// If transaction should be ignored, because of error.
    Discard = 0,
    /// If we keep transaction and write write_set.
    Keep = 1,
}
/// Type of contract argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VmTypeTag {
    /// Bool 0x0 - false, 0x1 - true.
    Bool = 0,
    /// Uint64. Little-endian unsigned 64 bits integer.
    U64 = 1,
    /// Vector of bytes.
    Vector = 2,
    /// Address, in bech32 form. 20 bytes.
    Address = 3,
    /// U8
    U8 = 4,
    /// U128 Little-endian unsigned 128 bits integer.
    U128 = 5,
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
#[doc = r" Generated client implementations."]
pub mod vm_compiler_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct VmCompilerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VmCompilerClient<tonic::transport::Channel> {
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
    impl<T> VmCompilerClient<T>
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
            request: impl tonic::IntoRequest<super::SourceFile>,
        ) -> Result<tonic::Response<super::CompilationResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vm_grpc.VMCompiler/Compile");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VmCompilerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VmCompilerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VmCompilerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod vm_multiple_sources_compiler_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct VmMultipleSourcesCompilerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VmMultipleSourcesCompilerClient<tonic::transport::Channel> {
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
    impl<T> VmMultipleSourcesCompilerClient<T>
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
        ) -> Result<tonic::Response<super::MultipleCompilationResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/vm_grpc.VMMultipleSourcesCompiler/Compile");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VmMultipleSourcesCompilerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VmMultipleSourcesCompilerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VmMultipleSourcesCompilerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod vm_script_metadata_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Returns argument names/types for script."]
    pub struct VmScriptMetadataClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VmScriptMetadataClient<tonic::transport::Channel> {
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
    impl<T> VmScriptMetadataClient<T>
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
        pub async fn get_signature(
            &mut self,
            request: impl tonic::IntoRequest<super::VmScript>,
        ) -> Result<tonic::Response<super::Signature>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/vm_grpc.VMScriptMetadata/GetSignature");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VmScriptMetadataClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VmScriptMetadataClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VmScriptMetadataClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod vm_access_vector_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "/ Returns the structure access vector."]
    pub struct VmAccessVectorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VmAccessVectorClient<tonic::transport::Channel> {
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
    impl<T> VmAccessVectorClient<T>
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
        pub async fn get_access_vector(
            &mut self,
            request: impl tonic::IntoRequest<super::StructIdent>,
        ) -> Result<tonic::Response<super::AccessVector>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/vm_grpc.VMAccessVector/GetAccessVector");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VmAccessVectorClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VmAccessVectorClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VmAccessVectorClient {{ ... }}")
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
    #[doc(hidden)]
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
                            let fut = async move { inner.publish_module(request).await };
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
    #[doc(hidden)]
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
                            let fut = async move { inner.execute_script(request).await };
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
#[doc = r" Generated server implementations."]
pub mod vm_compiler_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with VmCompilerServer."]
    #[async_trait]
    pub trait VmCompiler: Send + Sync + 'static {
        async fn compile(
            &self,
            request: tonic::Request<super::SourceFile>,
        ) -> Result<tonic::Response<super::CompilationResult>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct VmCompilerServer<T: VmCompiler> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: VmCompiler> VmCompilerServer<T> {
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
    impl<T, B> Service<http::Request<B>> for VmCompilerServer<T>
    where
        T: VmCompiler,
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
                "/vm_grpc.VMCompiler/Compile" => {
                    #[allow(non_camel_case_types)]
                    struct CompileSvc<T: VmCompiler>(pub Arc<T>);
                    impl<T: VmCompiler> tonic::server::UnaryService<super::SourceFile> for CompileSvc<T> {
                        type Response = super::CompilationResult;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SourceFile>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.compile(request).await };
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
    impl<T: VmCompiler> Clone for VmCompilerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: VmCompiler> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VmCompiler> tonic::transport::NamedService for VmCompilerServer<T> {
        const NAME: &'static str = "vm_grpc.VMCompiler";
    }
}
#[doc = r" Generated server implementations."]
pub mod vm_multiple_sources_compiler_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with VmMultipleSourcesCompilerServer."]
    #[async_trait]
    pub trait VmMultipleSourcesCompiler: Send + Sync + 'static {
        async fn compile(
            &self,
            request: tonic::Request<super::SourceFiles>,
        ) -> Result<tonic::Response<super::MultipleCompilationResult>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct VmMultipleSourcesCompilerServer<T: VmMultipleSourcesCompiler> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: VmMultipleSourcesCompiler> VmMultipleSourcesCompilerServer<T> {
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
    impl<T, B> Service<http::Request<B>> for VmMultipleSourcesCompilerServer<T>
    where
        T: VmMultipleSourcesCompiler,
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
                "/vm_grpc.VMMultipleSourcesCompiler/Compile" => {
                    #[allow(non_camel_case_types)]
                    struct CompileSvc<T: VmMultipleSourcesCompiler>(pub Arc<T>);
                    impl<T: VmMultipleSourcesCompiler>
                        tonic::server::UnaryService<super::SourceFiles> for CompileSvc<T>
                    {
                        type Response = super::MultipleCompilationResult;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SourceFiles>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.compile(request).await };
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
    impl<T: VmMultipleSourcesCompiler> Clone for VmMultipleSourcesCompilerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: VmMultipleSourcesCompiler> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VmMultipleSourcesCompiler> tonic::transport::NamedService
        for VmMultipleSourcesCompilerServer<T>
    {
        const NAME: &'static str = "vm_grpc.VMMultipleSourcesCompiler";
    }
}
#[doc = r" Generated server implementations."]
pub mod vm_script_metadata_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with VmScriptMetadataServer."]
    #[async_trait]
    pub trait VmScriptMetadata: Send + Sync + 'static {
        async fn get_signature(
            &self,
            request: tonic::Request<super::VmScript>,
        ) -> Result<tonic::Response<super::Signature>, tonic::Status>;
    }
    #[doc = " Returns argument names/types for script."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct VmScriptMetadataServer<T: VmScriptMetadata> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: VmScriptMetadata> VmScriptMetadataServer<T> {
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
    impl<T, B> Service<http::Request<B>> for VmScriptMetadataServer<T>
    where
        T: VmScriptMetadata,
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
                "/vm_grpc.VMScriptMetadata/GetSignature" => {
                    #[allow(non_camel_case_types)]
                    struct GetSignatureSvc<T: VmScriptMetadata>(pub Arc<T>);
                    impl<T: VmScriptMetadata> tonic::server::UnaryService<super::VmScript> for GetSignatureSvc<T> {
                        type Response = super::Signature;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VmScript>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_signature(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSignatureSvc(inner);
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
    impl<T: VmScriptMetadata> Clone for VmScriptMetadataServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: VmScriptMetadata> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VmScriptMetadata> tonic::transport::NamedService for VmScriptMetadataServer<T> {
        const NAME: &'static str = "vm_grpc.VMScriptMetadata";
    }
}
#[doc = r" Generated server implementations."]
pub mod vm_access_vector_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with VmAccessVectorServer."]
    #[async_trait]
    pub trait VmAccessVector: Send + Sync + 'static {
        async fn get_access_vector(
            &self,
            request: tonic::Request<super::StructIdent>,
        ) -> Result<tonic::Response<super::AccessVector>, tonic::Status>;
    }
    #[doc = "/ Returns the structure access vector."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct VmAccessVectorServer<T: VmAccessVector> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: VmAccessVector> VmAccessVectorServer<T> {
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
    impl<T, B> Service<http::Request<B>> for VmAccessVectorServer<T>
    where
        T: VmAccessVector,
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
                "/vm_grpc.VMAccessVector/GetAccessVector" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccessVectorSvc<T: VmAccessVector>(pub Arc<T>);
                    impl<T: VmAccessVector> tonic::server::UnaryService<super::StructIdent> for GetAccessVectorSvc<T> {
                        type Response = super::AccessVector;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StructIdent>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_access_vector(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetAccessVectorSvc(inner);
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
    impl<T: VmAccessVector> Clone for VmAccessVectorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: VmAccessVector> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VmAccessVector> tonic::transport::NamedService for VmAccessVectorServer<T> {
        const NAME: &'static str = "vm_grpc.VMAccessVector";
    }
}
