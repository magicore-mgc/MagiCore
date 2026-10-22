//! PyPI client owner wiring and stable exports.
//! Wiring owner PyPI và giữ export ổn định.

mod error;
mod metadata;
mod metadata_request;
mod package_download;
mod pypi_client;
mod release_selection;
mod types;

pub use pypi_client::PyPiClient;
