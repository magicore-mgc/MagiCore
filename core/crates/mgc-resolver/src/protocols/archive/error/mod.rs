//! Wires archive-local typed errors and their contract tests.
//! Nối lỗi typed cục bộ của archive và test hợp đồng lỗi.

mod error;
#[cfg(test)]
mod test;
pub(super) use self::error::{ArchiveError, ArchiveResult};
