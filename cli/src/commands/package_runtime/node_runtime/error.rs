//! Typed failures for host Node runtime discovery.
//! Lỗi có kiểu khi tìm Node runtime trên máy host.

use anyhow::Error as AnyhowError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum NodeRuntimeError {
    HostNodeRuntimeMissing,
}

impl Display for NodeRuntimeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HostNodeRuntimeMissing => write!(
                formatter,
                "a host Node.js runtime is required to execute this JavaScript package binary"
            ),
        }
    }
}

impl std::error::Error for NodeRuntimeError {}

pub fn registry_package_missing_host_node() -> AnyhowError {
    AnyhowError::new(NodeRuntimeError::HostNodeRuntimeMissing)
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
