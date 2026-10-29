//! Protocol limits that appear in more than one place.
//! Giới hạn protocol xuất hiện ở nhiều nơi.
//!
//! Each value below is a number some external specification fixes. Repeating
//! the literal is how two validators drift apart — one accepts a name the other
//! rejects, and the rejection surfaces far from the cause. The constant name also
//! documents WHY the number is that number.
//! (Mỗi giá trị là con số do spec ngoài quy định. Lặp literal là cách hai validator
//! lệch nhau — một bên nhận tên bên kia từ chối. Tên hằng cũng ghi rõ VÌ SAO.)

/// Maximum length of an npm package name (the registry rejects longer).
/// Độ dài tối đa tên package npm (registry từ chối tên dài hơn).
pub const NPM_PACKAGE_NAME_MAX: usize = 214;

/// Maximum length of one path segment on the common filesystems we target
/// (ext4, APFS, NTFS all cap a single segment at 255 bytes).
/// Độ dài tối đa một segment path trên các filesystem phổ biến.
pub const PATH_SEGMENT_MAX: usize = 255;

/// Maximum length of an OCI/Docker image tag (`[A-Za-z0-9_][A-Za-z0-9._-]{0,127}`).
/// Độ dài tối đa tag image OCI/Docker.
pub const OCI_TAG_MAX: usize = 128;

/// Maximum length of a Linux interface name (`IFNAMSIZ - 1`).
/// Độ dài tối đa tên interface Linux.
pub const LINUX_INTERFACE_NAME_MAX: usize = 256;

/// HTTP status that means "client sent too many requests".
/// Mã HTTP "client gửi quá nhiều request".
pub const HTTP_TOO_MANY_REQUESTS: u16 = 429;

/// Lowest HTTP status in the server-error range.
/// Mã HTTP thấp nhất trong nhóm lỗi server.
pub const HTTP_SERVER_ERROR_MIN: u16 = 500;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
