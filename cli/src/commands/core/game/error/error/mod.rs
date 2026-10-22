//! Error variant and constructor owner.
//! Owner variant lỗi và constructor.

mod error;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use error::GameOptimizerError;
