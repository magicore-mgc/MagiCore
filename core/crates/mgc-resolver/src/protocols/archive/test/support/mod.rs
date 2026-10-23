//! Wires test-only archive fixture builders.
//! Nối các builder fixture archive chỉ dùng trong test.

mod test_support;
pub(super) use self::test_support::{tar_gz_entries, tar_gz_file, tar_gz_file_with_raw_path};
