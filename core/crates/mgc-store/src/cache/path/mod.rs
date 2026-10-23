//! Cache path construction owner.
//! Owner tạo đường dẫn cache.

mod naming;
mod path;

pub(super) use path::{metadata_path, tarball_path};
