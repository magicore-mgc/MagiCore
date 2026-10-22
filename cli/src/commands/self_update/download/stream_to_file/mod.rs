//! Export the archive streaming writer.
//! Công bố writer tải archive theo stream.

mod stream_to_file;
pub(in crate::commands::self_update) use stream_to_file::download_stream;
