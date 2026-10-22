//! Mcp Smoke Test output owner wiring.
//! Wiring owner ghi output mcp smoke test.

#[path = "mcp_smoke_test.rs"]
mod implementation;
pub(super) use implementation::write;
