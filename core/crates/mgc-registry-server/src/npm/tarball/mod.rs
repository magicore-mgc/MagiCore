//! Declare tarball endpoint owners.
//! Khai báo owner endpoint tarball.

mod delete;
mod download;
mod tarball;
mod upload;
pub(super) use delete::{delete_package_version_route, delete_package_version_scoped};
pub(super) use download::{download_tarball, download_tarball_scoped};
pub(super) use tarball::routes;
pub(super) use upload::{upload_tarball, upload_tarball_scoped};
