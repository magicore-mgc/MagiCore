//! config owner wiring.
//! File này chỉ nối module config và export handler tương ứng.

mod config;
pub(in crate::commands::mcp::rpc::tool_call) use config::handle_config;
