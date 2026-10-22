//! Stable shared package.json API and module wiring.
//! API package.json dùng chung ổn định và wiring các module.

mod atomic_write;
mod error;
mod manifest;
mod package_json;
mod workspace_protocol;

pub use atomic_write::{atomic_write, atomic_write_if_changed};
pub use manifest::{parse_manifest, write_manifest};
pub use package_json::PackageJson;
pub use workspace_protocol::is_workspace_protocol_range;
