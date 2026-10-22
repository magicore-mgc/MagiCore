//! MCP protocol output handle owner wiring.
//! File này chỉ nối owner handle giao thức và export kiểu cho vòng stdio.

mod protocol_output;
pub(in crate::commands::mcp) use protocol_output::McpProtocolOutput;
