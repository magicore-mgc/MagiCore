//! Stable AI model-download API and owner wiring.
//! API download model AI ổn định và wiring các owner.

mod checksum;
mod destination;
mod run_download;
mod source;
mod staging;
mod transfer;
mod validation;

pub use checksum::verify_file_checksum;
pub use run_download::download_model;
pub use source::ModelSource;
#[cfg(test)]
pub(in crate::install) use transfer::{
    http_download_to_staging_with_limit, next_model_download_total,
};
#[cfg(test)]
pub(in crate::install) use validation::validate_remote_integrity;

pub(super) use staging::filename_from_url;
