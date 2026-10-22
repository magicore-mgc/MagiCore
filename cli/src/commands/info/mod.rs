//! Package-info command facade.
//! Facade lệnh xem thông tin package.

#[cfg(feature = "web")]
mod core_label;
mod error;
mod info;
#[cfg(feature = "web")]
mod local_version;
#[cfg(feature = "web")]
mod output;

pub use info::{run, versions_json};
