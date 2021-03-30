pub extern crate tonic;

pub mod grpc {
    pub(crate) use ds_grpc as ds;

    pub use super::ds_grpc_ext::*;

    pub mod compiler_grpc;
    pub mod ds_grpc;
    pub mod metadata_grpc;
    pub mod types;
    pub mod vm_grpc;
}

mod ds_grpc_ext {
    use crate::grpc::ds::ErrorCode;

    use super::grpc::ds::DsAccessPath;
    use super::grpc::ds::DsRawResponse;

    impl DsAccessPath {
        pub fn new(address: Vec<u8>, path: Vec<u8>) -> Self {
            Self { address, path }
        }
    }

    impl DsRawResponse {
        pub fn with_blob(blob: &[u8]) -> DsRawResponse {
            DsRawResponse {
                blob: blob.to_vec(),
                error_code: ErrorCode::None as i32,
                error_message: "".to_string(),
            }
        }

        pub fn with_error(error_code: ErrorCode, error_message: String) -> DsRawResponse {
            DsRawResponse {
                blob: vec![],
                error_code: error_code as i32,
                error_message,
            }
        }
    }
}

pub mod types {
    pub use crate::grpc::types::U128;

    impl From<u128> for U128 {
        fn from(val: u128) -> Self {
            U128 {
                buf: val.to_le_bytes().to_vec(),
            }
        }
    }

    impl From<U128> for u128 {
        fn from(val: U128) -> Self {
            let mut buf = [0; 16];
            buf.copy_from_slice(&val.buf);
            u128::from_le_bytes(buf)
        }
    }

    #[cfg(test)]
    mod test {
        use super::U128;
        #[test]
        fn test_u128() {
            assert_eq!(u128::from(U128::from(1134)), 1134);
        }
    }
}
