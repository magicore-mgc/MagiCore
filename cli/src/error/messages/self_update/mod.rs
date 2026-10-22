//! Self-update download and signature diagnostics.
//! Lỗi tải và xác minh self-update.

mod error;

pub use error::{
    self_update_checksum_mismatch, self_update_download_failed, self_update_invalid_version,
    self_update_manifest_invalid, self_update_no_binary, self_update_probe_failed,
    self_update_signature_invalid, self_update_unsigned_refused, self_update_unsupported_host,
    self_update_unsupported_variant,
};
