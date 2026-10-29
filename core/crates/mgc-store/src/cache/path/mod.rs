//! Cache path construction owner.
//! Owner tạo đường dẫn cache.

mod naming;
mod path;
mod registry_scope;

pub(super) use path::{metadata_path, tarball_path};
pub(in crate::cache) use registry_scope::registry_scope_directory;
