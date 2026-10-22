//! Export metadata read handlers.
//! Xuất handler đọc metadata.

mod fetch;
#[cfg(test)]
pub(in crate::npm) use fetch::rewrite_tarball_host;
pub(in crate::npm) use fetch::{get_package, get_package_scoped};
