//! Exercise PEP 508 marker behavior for the selected Python environment.
//! Kiểm tra marker PEP 508 theo môi trường Python được chọn.

use super::super::super::*;

#[test]
fn sys_platform_markers_evaluate() {
    // This machine's platform must match itself and reject others.
    // Platform máy hiện tại phải khớp chính nó và từ chối platform khác.
    #[cfg(target_os = "linux")]
    {
        assert_eq!(marker_applies("sys_platform == 'linux'", None), Some(true));
        assert_eq!(
            marker_applies("sys_platform == 'darwin'", None),
            Some(false)
        );
        assert_eq!(marker_applies("sys_platform == 'win32'", None), Some(false));
        assert_eq!(marker_applies("os_name == 'posix'", None), Some(true));
    }
    #[cfg(target_os = "macos")]
    {
        assert_eq!(marker_applies("sys_platform == 'darwin'", None), Some(true));
        assert_eq!(marker_applies("sys_platform == 'linux'", None), Some(false));
    }
    #[cfg(target_os = "windows")]
    {
        assert_eq!(marker_applies("sys_platform == 'win32'", None), Some(true));
        assert_eq!(marker_applies("os_name == 'nt'", None), Some(true));
    }
    assert_eq!(marker_applies("extra == 'foo'", None), None);
    // No consumer configured: version grounds never exclude.
    // Chưa cấu hình consumer thì không loại theo điều kiện version.
    assert_eq!(marker_applies("python_version < '3.0'", None), None);
    assert_eq!(marker_applies("python_version > '3.99'", None), None);
    // Consumer 3.9: version grounds decide.
    // Với consumer 3.9, điều kiện version quyết định kết quả.
    assert_eq!(
        marker_applies("python_version < '3.10'", Some((3, 9))),
        Some(true)
    );
    assert_eq!(
        marker_applies("python_version >= '3.10'", Some((3, 9))),
        Some(false)
    );
    assert_eq!(
        marker_applies("python_version == '3.9.*'", Some((3, 9))),
        Some(true)
    );
}

#[test]
fn python_version_markers_need_consumer() {
    // Without MGC_PYTHON_VERSION the version is unknown — never
    // Khi thiếu MGC_PYTHON_VERSION, version chưa biết — không được
    // exclude on version grounds alone.
    // loại package chỉ dựa vào điều kiện version.
    assert_eq!(marker_applies("python_version < '3.0'", None), None);
}

#[test]
fn compound_markers_compose() {
    assert_eq!(
        marker_applies("extra == 'docs' and python_version < '3.10'", Some((3, 9))),
        None,
        "unknown AND true remains unknown"
    );
    assert_eq!(
        marker_applies("extra == 'docs' and python_version > '3.99'", Some((3, 9))),
        Some(false),
        "false AND unknown is determinately false"
    );
    assert_eq!(
        marker_applies("extra == 'docs' or python_version > '3.99'", Some((3, 9))),
        None,
        "unknown OR false remains unknown"
    );
    assert_eq!(
        marker_applies(
            "platform_release == '版本' and python_version < '3.10'",
            Some((3, 9))
        ),
        None,
        "Unicode literals must not panic and unknown AND true remains unknown"
    );
    #[cfg(target_os = "linux")]
    {
        assert_eq!(
            marker_applies(
                "sys_platform == 'darwin' and python_version > '3.99'",
                Some((3, 9))
            ),
            Some(false)
        );
        assert_eq!(
            marker_applies(
                "sys_platform == 'linux' and python_version > '3.99'",
                Some((3, 9))
            ),
            Some(false)
        );
        assert_eq!(
            marker_applies("sys_platform == 'darwin' or sys_platform == 'linux'", None),
            Some(true)
        );
    }
}
