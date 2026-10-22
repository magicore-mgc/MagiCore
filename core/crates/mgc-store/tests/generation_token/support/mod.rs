//! Shared fixtures for generation-token tests.
//! Fixture dùng chung cho test generation-token.

#[path = "support.rs"]
mod support;
pub(crate) use support::{PROJ_A, PROJ_B, TestDb, assert_orphan_claim_rejected};
