//! Unix path-validation assertions.
//! Kiểm tra xác thực path trên Unix.

use super::super::reflink_clone;
use crate::reflink::ReflinkError;
use std::io;
use std::path::Path;

#[test]
fn embedded_nul_path_is_rejected_before_native_file_operations() {
    let error = reflink_clone(Path::new("bad\0source"), Path::new("target"))
        .expect_err("embedded NUL must be rejected");

    assert!(matches!(
        error,
        ReflinkError::Other(ref cause)
            if cause.kind() == io::ErrorKind::InvalidInput
    ));
}
