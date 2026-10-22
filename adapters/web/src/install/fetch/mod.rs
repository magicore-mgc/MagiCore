//! Web install tarball-fetch owners and stable public routes.
//! Các owner tải tarball của Web install và các route public ổn định.

mod error;
mod fetch;
mod staging;
mod tarball_url;

pub use self::fetch::{get_tarball_bytes, get_tarball_bytes_with_registry};
pub(crate) use self::staging::{
    DownloadStagingGuard, StagingError, publish_downloaded_tarball, unique_tarball_staging_path,
};
pub use self::tarball_url::package_tarball_url;
