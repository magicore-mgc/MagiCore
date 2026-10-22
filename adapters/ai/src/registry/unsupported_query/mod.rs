//! Export fail-closed handlers for providers without query clients.
//! Export handler fail-closed cho provider chưa có client truy vấn.

mod unsupported_query;
pub(super) use unsupported_query::{query_onnx, query_pytorch, query_tfhub};
