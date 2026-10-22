//! Shared PyPI wheel fixture support for unit cases.
//! Fixture dùng chung cho unit test wheel PyPI.

#[path = "test_support.rs"]
mod implementation;

pub(in crate::protocols::pypi) use implementation::{entry_for, stored_zip};
