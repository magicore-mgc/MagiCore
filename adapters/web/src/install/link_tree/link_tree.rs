//! Strict package-link module facade and owner exports.
//! Facade module link package strict và export các owner.

#[path = "backing_file/mod.rs"]
mod backing_file;
#[path = "entry/mod.rs"]
mod entry;
#[path = "error/mod.rs"]
mod error;
#[path = "mode/mod.rs"]
mod mode;
#[path = "pool/mod.rs"]
mod pool;
#[path = "tree/mod.rs"]
mod tree;

pub use backing_file::backing_link_file;
pub use entry::{link_package_tree, link_package_tree_with_profile};
pub use mode::{StrictTreeLinkMode, strict_tree_link_mode};
pub use pool::{default_hardlink_threads, hardlink_pool, hardlink_thread_count};
pub use tree::{hardlink_tree, hardlink_tree_with_profile};
