//! Wires archive path normalization and its tests.
//! Nối chuẩn hóa path archive và test tương ứng.

mod archive_path;
pub(super) use self::archive_path::sanitize_archive_path;
#[cfg(test)]
mod test;
