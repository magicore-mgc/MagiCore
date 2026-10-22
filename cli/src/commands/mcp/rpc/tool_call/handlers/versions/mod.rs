//! versions owner wiring.
//! File này chỉ nối module versions và export handler tương ứng.

mod versions;
pub(in crate::commands::mcp::rpc::tool_call) use versions::handle_versions;
