//! Export safe tar.gz binary extraction.
//! Công bố trích binary tar.gz an toàn.

mod tar_gz;
pub(in crate::commands::self_update) use tar_gz::stage_from_tar_gz;
