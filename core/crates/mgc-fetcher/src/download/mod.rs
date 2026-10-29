//! Wire the download entry point and its nearest tests.
//! Nối entry point tải file và test gần nhất.

mod download;
pub use download::{
    DEFAULT_MAX_DOWNLOAD_BYTES, MAX_DOWNLOAD_BYTES_ENV, ProgressCallback, download_with_progress,
    max_download_bytes,
};
#[cfg(test)]
mod test;
