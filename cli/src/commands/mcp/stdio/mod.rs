//! MCP stdio session and output-owner wiring.
//! Khai báo vòng stdio cùng owner handle, capture và response output.

mod append_captured_stdout;
mod protocol_output;
mod stdio;
mod stdout_capture;

pub(super) use append_captured_stdout::append_captured_stdout;
pub(super) use protocol_output::McpProtocolOutput;
pub(super) use stdio::serve;
pub(super) use stdout_capture::McpStdoutCapture;
