//! Local tarball error exports — export lỗi tarball cục bộ.

mod error;

pub(super) use error::PackError;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
