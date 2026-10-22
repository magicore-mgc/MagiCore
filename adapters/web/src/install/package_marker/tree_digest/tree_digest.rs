//! Hash a normalized path, size, executable bit, and content digest.
//! Hash path chuẩn hóa, kích thước, cờ executable và digest nội dung.

use sha2::{Digest, Sha256};

/// Hash path, byte length, and file content digest into the package-tree digest.
/// Hash path, kích thước và digest nội dung vào digest cây package.
pub(crate) fn update_tree_hasher(
    hasher: &mut Sha256,
    path: &str,
    size: u64,
    executable: bool,
    content_sha256: &str,
) {
    hasher.update(path.as_bytes());
    hasher.update([0]);
    hasher.update(size.to_string().as_bytes());
    hasher.update([0]);
    hasher.update([u8::from(executable)]);
    hasher.update([0]);
    hasher.update(content_sha256.as_bytes());
    hasher.update(*b"\n");
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
