// Recursive error conversion — preserve the exact user-facing messages.
// Chuyển lỗi recursive — giữ nguyên nội dung thông báo người dùng.
use super::super::EnginePolicyError;

#[test]
fn recursive_not_available_keeps_its_diagnostic() {
    assert_eq!(
        EnginePolicyError::RecursiveNotAvailable
            .into_error()
            .to_string(),
        "--recursive is declared on the CLI surface but workspace recursion is not implemented yet. Refusing to pretend it ran."
    );
}

#[test]
fn unsupported_recursive_command_keeps_its_label_and_diagnostic() {
    assert_eq!(
        EnginePolicyError::RecursiveUnsupported { command: "init" }
            .into_error()
            .to_string(),
        "--recursive is not implemented for 'init' yet. Beta safety rule: refusing a silent no-op on workspace commands."
    );
}
