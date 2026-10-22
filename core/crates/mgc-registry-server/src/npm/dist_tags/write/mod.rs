//! Export dist-tag write handlers.
//! Xuất handler ghi dist-tag.

mod write;
pub(in crate::npm) use write::{set_dist_tag, set_dist_tag_scoped};
