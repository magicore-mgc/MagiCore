//! Agent Smoke Test output owner wiring.
//! Wiring owner ghi output agent smoke test.

#[path = "agent_smoke_output.rs"]
mod implementation;
pub(super) use implementation::write;
