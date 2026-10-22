//! Public API routing assertions.
//! Kiểm tra điều phối API public.

use crate::reflink::{ReflinkError, reflink_clone};
#[cfg(not(unix))]
use std::path::Path;

#[cfg(unix)]
#[test]
fn public_dispatch_reports_missing_source_as_operation_error() {
    let temp = tempfile::tempdir().expect("create temporary directory");
    let missing_source = temp.path().join("missing-source");
    let target = temp.path().join("target");

    assert!(matches!(
        reflink_clone(&missing_source, &target),
        Err(ReflinkError::Other(_))
    ));
}

#[cfg(not(unix))]
#[test]
fn public_dispatch_reports_unsupported_platform() {
    let error = reflink_clone(Path::new("source"), Path::new("target"))
        .expect_err("non-Unix target must reject reflink");

    assert!(matches!(
        error,
        ReflinkError::NotSupported(ref cause)
            if cause.kind() == std::io::ErrorKind::Unsupported
    ));
}
