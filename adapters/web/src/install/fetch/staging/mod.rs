//! Staging and publication owner wiring.
//! Wiring cho owner staging và publication.

mod error;
mod staging;
pub(crate) use self::error::StagingError;
pub(crate) use staging::{
    DownloadStagingGuard, publish_downloaded_tarball, unique_tarball_staging_path,
};
