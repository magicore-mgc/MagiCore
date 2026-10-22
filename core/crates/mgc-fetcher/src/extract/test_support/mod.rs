//! Exports gzip-tar fixture builders for nearest unit tests.
//! Export helper fixture gzip-tar cho unit test gần nhất.

mod test_support;
#[cfg(windows)]
pub(super) use self::test_support::gzip_tar_file_with_mode;
pub(super) use self::test_support::{gzip_tar_file, gzip_tar_files, gzip_tar_link};
