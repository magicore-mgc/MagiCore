//! install owner wiring.
//! File này chỉ nối module install và export handler tương ứng.

mod install;
pub(in crate::commands::mcp::rpc::tool_call) use install::handle_install;
