//! Declare quota, bounded and streamed transfer owners.
//! Khai báo owner quota, tải giới hạn và tải stream.

mod bounded_response;
mod download;
mod limits;
mod stream_to_file;
pub(in crate::commands) use bounded_response::download_capped;
pub(in crate::commands) use download::{download_file, download_text};
pub(in crate::commands) use limits::MAX_DOC_BYTES;
