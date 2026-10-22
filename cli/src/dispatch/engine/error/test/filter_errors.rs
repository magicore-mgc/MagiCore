// Filter error conversion — preserve the exact user-facing messages.
// Chuyển lỗi filter — giữ nguyên nội dung thông báo người dùng.
use super::super::EnginePolicyError;

#[test]
fn unsupported_filter_command_keeps_its_label_and_diagnostic() {
    assert_eq!(
        EnginePolicyError::FilterUnsupported { command: "publish" }
            .into_error()
            .to_string(),
        "--filter is not wired into 'publish' yet (the publish recursive pipeline does not accept filter). Refusing a silent no-op."
    );
}

#[test]
fn filter_without_recursive_keeps_its_diagnostic() {
    assert_eq!(
        EnginePolicyError::FilterRequiresRecursive
            .into_error()
            .to_string(),
        "--filter requires --recursive (it filters workspace targets). Use `mgc <cmd> --recursive --filter <glob>`."
    );
}
