pub extern crate tonic;

pub mod grpc {
    pub mod ds_grpc;
    pub mod vm_grpc;
    pub mod compiler_grpc;
    pub mod metadata_grpc;
    pub mod types;

    pub(crate) use ds_grpc as ds;
    pub use super::ds_grpc_ext::*;
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
