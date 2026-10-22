//! Mcp Server output owner wiring.
//! Wiring owner ghi output mcp server.

#[path = "mcp_server.rs"]
mod implementation;
pub(super) use implementation::write;
