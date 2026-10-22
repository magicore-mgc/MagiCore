//! Declare npm dist-tag owners.
//! Khai báo owner dist-tag npm.

mod delete;
mod dist_tags;
mod read;
mod write;
pub(super) use delete::{delete_dist_tag, delete_dist_tag_scoped};
pub(super) use dist_tags::routes;
pub(super) use read::{get_dist_tags, get_dist_tags_scoped};
pub(super) use write::{set_dist_tag, set_dist_tag_scoped};
