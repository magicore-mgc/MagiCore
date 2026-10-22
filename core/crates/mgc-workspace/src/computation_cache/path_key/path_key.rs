// Native source-path key — khóa đường dẫn source gốc

use std::path::Path;

/// Encode native path units so distinct source names cannot collapse to one cache key.
/// Mã hóa đơn vị path gốc để các tên nguồn khác nhau không bị gộp thành cùng cache key.
#[cfg(unix)]
pub(in crate::computation_cache) fn source_path_key(path: &Path) -> String {
    use std::os::unix::ffi::OsStrExt;
    encode_path_bytes(path.as_os_str().as_bytes().iter().copied())
}

#[cfg(windows)]
pub(in crate::computation_cache) fn source_path_key(path: &Path) -> String {
    use std::os::windows::ffi::OsStrExt;
    encode_path_bytes(path.as_os_str().encode_wide().flat_map(u16::to_le_bytes))
}

#[cfg(not(any(unix, windows)))]
pub(in crate::computation_cache) fn source_path_key(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn encode_path_bytes(bytes: impl IntoIterator<Item = u8>) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::new();
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}
