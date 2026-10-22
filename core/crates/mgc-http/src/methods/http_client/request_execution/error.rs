//! Typed request-execution failures owned by this module.
//! Lỗi typed thực thi request do module này sở hữu.

use std::fmt;

#[derive(Debug)]
pub(super) enum RequestExecutionError {
    RequestNotCloneable,
}

impl fmt::Display for RequestExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestNotCloneable => formatter.write_str("request not cloneable"),
        }
    }
}

impl std::error::Error for RequestExecutionError {}
