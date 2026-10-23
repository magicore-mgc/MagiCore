//! Python Web backend starter owner.
//! Owner starter backend Web Python.

pub mod django;
mod python_backend;

pub(crate) use python_backend::{write_fastapi, write_flask};
