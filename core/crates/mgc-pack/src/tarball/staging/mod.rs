//! Staged output lifecycle exports — export vòng đời output staging.

mod staging;

pub(super) use staging::{create_staged_output, publish_staged_output};

#[cfg(test)]
#[path = "test/staging.rs"]
mod tests;
