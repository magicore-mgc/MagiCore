//! Wiring for the filesystem owner.
//! Wiring cho owner filesystem.

mod filesystem;
pub use filesystem::{
    prune_root_install_dirs, remove_fs_entry, repair_dangling_symlinks, reset_nested_node_modules,
};
