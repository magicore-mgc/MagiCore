//! Explicit non-Unix reflink unsupported result.
//! Kết quả không hỗ trợ reflink tường minh trên nền tảng non-Unix.

use crate::reflink::ReflinkError;
use std::io;
use std::path::Path;

#[cfg(not(unix))]
pub(in crate::reflink) fn reflink_clone(
    _source: &Path,
    _target: &Path,
) -> Result<(), ReflinkError> {
    Err(ReflinkError::NotSupported(io::Error::new(
        io::ErrorKind::Unsupported,
        "reflink not supported on this platform",
    )))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
