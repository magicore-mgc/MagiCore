//! Test environment override selection without mutating process-global state.
//! Kiểm tra chọn biến môi trường mà không sửa trạng thái toàn tiến trình.

#[cfg(any(unix, windows))]
use std::ffi::OsString;

use super::super::configured_web_registry_url;

#[test]
fn missing_override_uses_the_default_registry() {
    assert_eq!(
        configured_web_registry_url(None).unwrap(),
        super::super::DEFAULT_WEB_REGISTRY_URL
    );
}

#[cfg(any(unix, windows))]
#[test]
fn non_unicode_override_is_rejected_instead_of_using_the_default() {
    #[cfg(unix)]
    let value = {
        use std::os::unix::ffi::OsStringExt;
        OsString::from_vec(vec![0xff])
    };

    #[cfg(windows)]
    let value = {
        use std::os::windows::ffi::OsStringExt;
        OsString::from_wide(&[0xd800])
    };

    let error = configured_web_registry_url(Some(value)).unwrap_err();

    assert_eq!(
        error.to_string(),
        "MAGICORE_WEB_REGISTRY_URL must contain a valid UTF-8 URL"
    );
}
