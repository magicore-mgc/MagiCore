//! Match a byte sequence without converting binary model data to text.
//! So khớp chuỗi byte mà không chuyển dữ liệu model nhị phân thành văn bản.

pub(in crate::audit::scanner) fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
