//! outdated owner wiring.
//! File này chỉ nối module outdated và export handler tương ứng.

mod outdated;
pub(in crate::commands::mcp::rpc::tool_call) use outdated::handle_outdated;
