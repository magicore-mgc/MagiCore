//! Export dist-tag read handlers.
//! Xuất handler đọc dist-tag.

mod read;
pub(in crate::npm) use read::{get_dist_tags, get_dist_tags_scoped};
