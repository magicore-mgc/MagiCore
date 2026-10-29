//! Wires test-only archive fixture builders.
//! Nối các builder fixture archive chỉ dùng trong test.

mod test_support;
#[cfg(test)]
pub(crate) use self::test_support::tar_gz_entries;
pub(in crate::protocols::archive) use self::test_support::{
    tar_gz_file, tar_gz_file_with_raw_path,
};
