//! Typed denial raised when a command may resolve external dependencies.
//! Lỗi từ chối typed khi lệnh có thể resolve dependency bên ngoài.

use std::fmt;

#[derive(Debug)]
pub(in crate::run) struct DependencyResolutionError {
    reason: String,
}

impl DependencyResolutionError {
    pub(super) fn new(reason: String) -> Self {
        Self { reason }
    }
}

impl fmt::Display for DependencyResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "external dependency resolution blocked: {}",
            self.reason
        )
    }
}

impl std::error::Error for DependencyResolutionError {}

#[cfg(test)]
#[path = "test/error.rs"]
mod test;
