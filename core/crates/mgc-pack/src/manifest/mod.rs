//! Public manifest API facade — preserves mgc-pack manifest routes.
//! Facade API manifest công khai — giữ ổn định các route của mgc-pack.

mod dependency_fields;
mod error;
mod metadata_identity;
mod sanitize;
mod types;

pub use dependency_fields::dep_fields;
pub use sanitize::sanitize;
pub use types::Sanitized;

pub(super) use error::ManifestSanitizeError;
pub(super) use metadata_identity::{name_or, version_or};
