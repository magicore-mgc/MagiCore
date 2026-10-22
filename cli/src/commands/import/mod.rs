//! Legacy lockfile import facade; public paths and crate-visible merge API stay stable.
//! Facade import lockfile cũ; giữ nguyên API public và API merge nội bộ crate.

mod artifact_reader;
mod error;
mod import;
mod merge;
mod project_owner;
mod signing_policy;

use error::ImportError;
pub use import::run;
