//! MCP tool-call owner wiring.
//! Khai báo dispatcher tools/call và nhóm handler theo tool.

mod handlers;
mod tool_call;
pub(in crate::commands::mcp) use tool_call::tool_call;
