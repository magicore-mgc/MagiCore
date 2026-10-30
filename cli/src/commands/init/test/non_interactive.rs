//! Tests for non-interactive `mgc init` name resolution.
//! Kiểm tra giải tên dự án không tương tác cho `mgc init`.

use super::resolve_non_interactive_name;

/// A bad explicit `--name` must error (fail fast) and never fall back to a
/// prompt; the failure names the offending value.
/// Tên `--name` sai phải trả lỗi (fail fast), không rơi về prompt; thông báo
/// nêu đúng giá trị sai.
#[test]
fn explicit_name_is_validated_and_rejects_path_traversal() {
    let err = resolve_non_interactive_name(Some("../escape".to_string()), false)
        .expect_err("a traversal name must be rejected, not prompted");
    assert!(
        err.to_string().contains("../escape"),
        "error must name the rejected value: {err}"
    );
}

/// An explicit, valid `--name` wins verbatim and never consults the cwd.
/// `--name` hợp lệ được dùng nguyên văn và không phụ thuộc cwd.
#[test]
fn explicit_valid_name_is_used_verbatim() {
    let resolved = resolve_non_interactive_name(Some("ci-project".to_string()), false)
        .expect("valid name must resolve");
    assert_eq!(resolved.as_deref(), Some("ci-project"));
}

/// `--yes` without `--name` derives a deterministic default from the current
/// directory basename and validates it successfully.
/// `--yes` không có `--name` suy mặc định tất định từ basename thư mục hiện tại
/// và xác thực thành công.
#[test]
fn yes_derives_name_from_current_directory_basename() {
    let cwd = std::env::current_dir().expect("current directory");
    let basename = cwd
        .file_name()
        .and_then(|segment| segment.to_str())
        .expect("current directory has a basename");

    let resolved = resolve_non_interactive_name(None, true).expect("--yes must derive a name");
    assert_eq!(resolved.as_deref(), Some(basename));
}

/// No flags preserves the interactive contract: nothing is resolved here.
/// Không cờ nào giữ hợp đồng tương tác: ở đây không giải tên.
#[test]
fn neither_flag_yields_interactive_path() {
    let resolved = resolve_non_interactive_name(None, false).expect("no flags is not an error");
    assert!(
        resolved.is_none(),
        "interactive path must not pre-resolve a name"
    );
}
