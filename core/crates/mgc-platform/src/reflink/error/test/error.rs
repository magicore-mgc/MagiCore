//! Reflink error and source-chain assertions.
//! Kiểm tra lỗi reflink và chuỗi nguyên nhân.

use crate::reflink::ReflinkError;
use std::error::Error;
use std::io;

#[test]
fn error_categories_preserve_display_and_source() {
    let unsupported = ReflinkError::NotSupported(io::Error::new(
        io::ErrorKind::Unsupported,
        "filesystem cannot clone",
    ));
    assert_eq!(
        unsupported.to_string(),
        "reflink not supported: filesystem cannot clone"
    );
    assert_eq!(
        unsupported
            .source()
            .and_then(|cause| cause.downcast_ref::<io::Error>())
            .map(|cause| cause.kind()),
        Some(io::ErrorKind::Unsupported)
    );

    let other = ReflinkError::Other(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "target denied",
    ));
    assert_eq!(other.to_string(), "reflink failed: target denied");
    assert_eq!(
        other
            .source()
            .and_then(|cause| cause.downcast_ref::<io::Error>())
            .map(|cause| cause.kind()),
        Some(io::ErrorKind::PermissionDenied)
    );
}
