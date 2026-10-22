//! Represent an explicit core selector conflicting with project ownership.
//! Biểu diễn lỗi `--core` xung đột với chủ sở hữu project đã ghi nhận.

use std::fmt;

/// A stable, downcastable error for attempts to reassign a project core.
/// Lỗi typed ổn định, cho phép downcast khi cố đổi core của project.
#[derive(Debug)]
pub(in crate::context::config_resolution) struct CoreOverrideConflictError {
    requested: String,
    owned: String,
}

impl CoreOverrideConflictError {
    pub(in crate::context::config_resolution) fn new(requested: &str, owned: &str) -> Self {
        Self {
            requested: requested.to_string(),
            owned: owned.to_string(),
        }
    }
}

impl fmt::Display for CoreOverrideConflictError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "core override '{}' conflicts with existing project core '{}'; core reassignment is not supported by this version",
            self.requested, self.owned,
        )
    }
}

impl std::error::Error for CoreOverrideConflictError {}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
