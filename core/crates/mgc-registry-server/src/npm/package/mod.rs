//! Declare package metadata read/publish owners.
//! Khai báo owner đọc/publish metadata package.

mod fetch;
mod package;
mod publish;
#[cfg(test)]
pub(super) use fetch::rewrite_tarball_host;
pub(super) use fetch::{get_package, get_package_scoped};
pub(super) use package::routes;
pub(super) use publish::{publish_package, publish_package_scoped};
