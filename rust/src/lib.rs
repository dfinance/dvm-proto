pub extern crate tonic;

pub mod grpc {
    #[path = "../grpc/dfinance.dvm.rs"]
    pub mod dfinance_dvm;
    pub use dfinance_dvm::*;
    pub use super::grpc_ext::*;
}

mod grpc_ext {
    use crate::grpc::{DsAccessPath, DsRawResponse, ErrorCode};

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
    use crate::grpc::U128;

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
