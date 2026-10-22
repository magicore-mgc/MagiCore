//! Export dist-tag delete handlers.
//! Xuất handler xóa dist-tag.

mod delete;
pub(in crate::npm) use delete::{delete_dist_tag, delete_dist_tag_scoped};
