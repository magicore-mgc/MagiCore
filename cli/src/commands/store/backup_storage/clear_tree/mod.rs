//! Safe directory clearing owner wiring.
//! Wiring owner xóa nội dung directory an toàn.

mod clear_tree;
mod error;
pub(in crate::commands::store) use clear_tree::clear_dir;
