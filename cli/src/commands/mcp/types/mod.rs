//! JSON-RPC request and response model wiring.
//! File này chỉ export model request và response cho các owner cùng server.

mod types;
pub(super) use types::{JsonRpcRequest, JsonRpcResponse};
