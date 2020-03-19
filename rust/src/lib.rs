pub extern crate tonic;

pub mod grpc {
    pub mod ds_grpc;
    pub mod vm_grpc;
    pub(crate) use ds_grpc as ds;
    pub(crate) use vm_grpc as vm;

    pub use super::vm_grpc_ext::*;
    pub use super::ds_grpc_ext::*;
}

mod vm_grpc_ext {
    use super::grpc::vm::CompilationResult;
    use super::grpc::vm::{Signature, VmScript, VmTypeTag};

    impl CompilationResult {
        pub fn with_bytecode(bytecode: Vec<u8>) -> Self {
            CompilationResult {
                bytecode,
                errors: vec![],
            }
        }

        pub fn with_errors(errors: Vec<String>) -> Self {
            CompilationResult {
                bytecode: vec![],
                errors,
            }
        }
    }

    impl VmScript {
        pub fn new(bytecode: Vec<u8>) -> Self {
            Self { code: bytecode }
        }
    }

    impl Signature {
        pub fn new(args: &[VmTypeTag]) -> Self {
            let arguments = args.iter().map(|&tag| tag as i32).collect();
            Self { arguments }
        }
    }
}

mod ds_grpc_ext {
    use super::grpc::ds::DsAccessPath;
    use super::grpc::ds::{DsRawResponse, ds_raw_response};

    impl DsAccessPath {
        pub fn new(address: Vec<u8>, path: Vec<u8>) -> Self {
            Self { address, path }
        }
    }

    impl DsRawResponse {
        pub fn with_blob(blob: &[u8]) -> DsRawResponse {
            DsRawResponse {
                blob: blob.to_vec(),
                error_code: ds_raw_response::ErrorCode::None as i32,
                error_message: "".to_string(),
            }
        }

        pub fn with_error(
            error_code: ds_raw_response::ErrorCode,
            error_message: String,
        ) -> DsRawResponse {
            DsRawResponse {
                blob: vec![],
                error_code: error_code as i32,
                error_message,
            }
        }
    }
}
