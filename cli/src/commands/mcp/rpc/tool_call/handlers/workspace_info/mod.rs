//! workspace_info owner wiring.
//! File này chỉ nối module workspace_info và export handler tương ứng.

mod workspace_info;
pub(in crate::commands::mcp::rpc::tool_call) use workspace_info::handle_workspace_info;
