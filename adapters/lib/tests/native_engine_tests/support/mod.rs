//! Shared integration test support facade.
//! Facade support dùng chung cho integration test.

#[path = "support.rs"]
mod implementation;

pub(crate) use implementation::{crate_line, mock_server, rust_manifest};

#[path = "environment_override/mod.rs"]
pub(crate) mod environment_override;
