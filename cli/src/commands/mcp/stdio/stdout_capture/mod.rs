//! MCP stdout capture owner wiring.
//! File này chỉ nối owner capture và export kiểu cho vòng stdio.

mod stdout_capture;
pub(in crate::commands::mcp) use stdout_capture::McpStdoutCapture;
