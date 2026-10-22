//! Shared manifest-read helper exports and nearest safety tests.
mod common;
pub(in crate::commands::optimizer) use common::project_text;

#[cfg(test)]
#[path = "test/common.rs"]
mod tests;
