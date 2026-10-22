//! add owner wiring.
//! File này chỉ nối module add và export handler tương ứng.

mod add;
pub(in crate::commands::mcp::rpc::tool_call) use add::handle_add;
