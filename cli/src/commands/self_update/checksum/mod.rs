//! Hash archives incrementally and verify the SHA-256 sidecar.
//! Hash archive theo khối và xác minh file SHA-256 đi kèm.

mod checksum;
pub(in crate::commands) use checksum::{sha256_file_streaming, verify_checksum};
