//! Protocol-limit constants — hằng giới hạn protocol.

#[path = "limits.rs"]
mod implementation;

pub use implementation::{
    HTTP_SERVER_ERROR_MIN, HTTP_TOO_MANY_REQUESTS, LINUX_INTERFACE_NAME_MAX, NPM_PACKAGE_NAME_MAX,
    OCI_TAG_MAX, PATH_SEGMENT_MAX,
};
