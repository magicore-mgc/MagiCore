//! uninstall owner wiring.
//! File này chỉ nối module uninstall và export handler tương ứng.

mod uninstall;
pub(in crate::commands::mcp::rpc::tool_call) use uninstall::handle_uninstall;
