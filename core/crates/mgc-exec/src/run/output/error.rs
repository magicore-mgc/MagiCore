//! Typed failures while draining captured child output.
//! Lỗi typed khi đọc output đã capture từ process con.

use std::fmt;
use std::io;

#[derive(Debug)]
pub(in crate::run::implementation) struct StreamReadError {
    source: io::Error,
}

impl StreamReadError {
    pub(super) fn new(source: io::Error) -> Self {
        Self { source }
    }
}

impl fmt::Display for StreamReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failed to read child output stream: {}",
            self.source
        )
    }
}

impl std::error::Error for StreamReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod test;
