//! Wire registry search access and its nearest tests.
//! Nối truy cập registry search với test gần nhất.

#[path = "registry_request.rs"]
mod registry_request;

pub(super) use registry_request::fetch_registry_search;
