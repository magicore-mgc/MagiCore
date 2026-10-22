//! audit owner wiring.
//! File này chỉ nối module audit và export handler tương ứng.

mod audit;
pub(in crate::commands::mcp::rpc::tool_call) use audit::handle_audit;
