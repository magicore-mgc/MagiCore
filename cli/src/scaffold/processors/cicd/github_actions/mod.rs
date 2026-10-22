//! GitHub Actions output owner wiring.
//! Wiring owner output GitHub Actions.

#[path = "github_actions.rs"]
mod implementation;
pub(super) use implementation::write;
