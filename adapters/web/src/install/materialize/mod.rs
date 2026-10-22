//! Web materialization compatibility facade and owner wiring.
//! Facade tương thích và wiring các owner materialize Web.

mod extracted_root;
mod filesystem;
mod graph;
mod materialize;
mod nested_dependencies;
mod package_root;
mod virtual_store;

pub use crate::install::link_tree::{
    StrictTreeLinkMode, backing_link_file, default_hardlink_threads, hardlink_pool,
    hardlink_thread_count, hardlink_tree, hardlink_tree_with_profile, link_package_tree,
    link_package_tree_with_profile,
};
pub use extracted_root::extracted_root_for;
pub use filesystem::{
    prune_root_install_dirs, remove_fs_entry, repair_dangling_symlinks, reset_nested_node_modules,
};
pub use graph::{graph_without_packages, select_root_packages};
pub use materialize::materialize_strict_layout;
pub use nested_dependencies::materialize_nested_dependencies;
pub use package_root::materialization_package_root;
pub use virtual_store::{strict_vstore_node_modules_dir, strict_vstore_package_dir};

#[cfg(test)]
pub(crate) mod test;
