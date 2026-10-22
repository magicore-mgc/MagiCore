//! Cache filesystem operations owner wiring.
//! File này chỉ nối module filesystem và export API nội bộ.

mod filesystem;

pub(super) use filesystem::{canonical_or_original, cleanup_empty_dirs, path_size, remove_path};
