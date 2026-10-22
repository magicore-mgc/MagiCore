//! Lockfile v4 public type owner wiring.
//! File này chỉ nối type document, package, payload và metadata.

mod document;
mod metadata;
mod package;
mod payload;

pub use document::LockfileV4;
pub use metadata::LockfileV4Metadata;
pub use package::PackageV4;
pub use payload::LockfilePayload;
