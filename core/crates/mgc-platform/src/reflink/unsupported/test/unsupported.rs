//! Unsupported-platform result assertions.
//! Kiểm tra kết quả cho nền tảng không hỗ trợ.

use super::super::reflink_clone;
use crate::reflink::ReflinkError;
use std::io;
use std::path::Path;

#[test]
fn unsupported_target_returns_typed_unsupported_error() {
    let error = reflink_clone(Path::new("source"), Path::new("target"))
        .expect_err("unsupported target must reject reflink");

    assert!(matches!(
        error,
        ReflinkError::NotSupported(ref cause)
            if cause.kind() == io::ErrorKind::Unsupported
    ));
}
