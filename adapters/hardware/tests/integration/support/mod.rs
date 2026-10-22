//! Shared integration-test support wiring.
//! Wiring cho tiện ích integration test dùng chung.

#[path = "support.rs"]
mod support;
pub use support::tmp;
