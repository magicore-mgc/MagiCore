//! Export named byte quotas to the download owner.
//! Công bố quota byte có tên cho owner tải.

mod limits;
pub(in crate::commands) use limits::{MAX_ARCHIVE_BYTES, MAX_DOC_BYTES};
