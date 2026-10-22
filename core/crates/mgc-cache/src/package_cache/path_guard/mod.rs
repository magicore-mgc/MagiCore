//! Cache-root path validation and no-follow cleanup exports.
//! Export kiểm tra path cache root và xóa không đi theo symlink.

mod path_guard;

pub(super) use path_guard::{
    ensure_cache_directory, ensure_parent_within_root, is_regular_cache_file, remove_path_no_follow,
};
