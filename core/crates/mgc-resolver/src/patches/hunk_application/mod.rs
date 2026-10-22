//! Hunk application module exports.
//! Export module áp dụng hunk.

mod error;
mod hunk_application;

pub(super) use hunk_application::apply_hunks;
