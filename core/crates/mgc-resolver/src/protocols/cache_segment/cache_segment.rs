//! Validate one registry-controlled path segment before joining it into a cache path.
//! Xác minh một segment path do registry kiểm soát trước khi ghép vào path cache.
//!
//! SEC-N1: NuGet builds `{root}/{id}/{version}`, Swift builds
//! `{root}/checkouts/{name}-{version}`. Those identity fields come from a
//! manifest or remote registration JSON; a value such as `..` or `a/b` would
//! move the package directory outside its root and redirect every subsequent
//! write. Path separators and traversal markers are refused outright.
//! (SEC-N1: NuGet dựng `{root}/{id}/{version}`, Swift dựng
//! `{root}/checkouts/{name}-{version}`. Các trường định danh này đến từ manifest
//! hoặc JSON registration từ xa; giá trị như `..` hay `a/b` sẽ đẩy thư mục
//! package ra ngoài root và chuyển hướng mọi lần ghi sau. Dấu phân cách path và
//! dấu traversal bị từ chối dứt khoát.)

use mgc_types::{MgError, MgResult};

/// Verify that `segment` is exactly one safe path component.
/// Xác minh `segment` đúng là một thành phần path an toàn.
pub(crate) fn validate_cache_segment(segment: &str, label: &str) -> MgResult<String> {
    // Surrounding whitespace is a Windows directory-name trick (`name ` maps to
    // `name`), so it is rejected rather than silently trimmed.
    // (Khoảng trắng đầu/cuối là mẹo tên thư mục của Windows (`name ` thành
    // `name`), nên bị từ chối thay vì âm thầm cắt.)
    let trimmed = segment.trim();
    let unsafe_value = trimmed.is_empty()
        || trimmed == "."
        || trimmed == ".."
        || trimmed.contains('/')
        || trimmed.contains('\\')
        || trimmed.contains('\0')
        // Windows drive-relative and alternate data stream syntax.
        // (Cú pháp ổ đĩa tương đối và alternate data stream của Windows.)
        || trimmed.contains(':')
        // Windows silently strips trailing dots/spaces, so `name.` and `name`
        // would collide on one directory.
        // (Windows âm thầm bỏ dấu chấm/khoảng trắng cuối nên `name.` và `name`
        // trùng một thư mục.)
        || trimmed.ends_with('.')
        || trimmed.ends_with(' ');
    if unsafe_value {
        return Err(MgError::Integrity(format!(
            "refusing unsafe {label} '{segment}' as a cache path segment"
        )));
    }
    Ok(trimmed.to_owned())
}

#[cfg(test)]
#[path = "test/cache_segment.rs"]
mod tests;
