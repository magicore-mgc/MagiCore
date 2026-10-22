//! Wires bounded-memory tree and link copying.
//! Nối owner copy tree và link với memory có giới hạn.

mod error;
mod tree_copy;

pub(super) use tree_copy::{copy_directory, copy_links};
