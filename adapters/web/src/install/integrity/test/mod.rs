//! Register integrity-reader cases. / Đăng ký ca kiểm tra integrity reader.

use super::verify_tarball_reader_integrity;

include!("streaming_sri_accepts_expected_sha512/mod.rs");
include!("streaming_sri_rejects_mismatched_sha512/mod.rs");
