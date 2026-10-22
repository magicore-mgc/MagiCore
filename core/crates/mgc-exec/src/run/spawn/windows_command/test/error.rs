//! Typed Windows command error contracts.
//! Hợp đồng lỗi typed của lệnh Windows.

use crate::run::implementation::spawn::windows_command::error::WindowsCommandError;

#[test]
fn windows_command_errors_are_stable_and_do_not_echo_untrusted_input() {
    let cases = [
        (
            WindowsCommandError::UnsupportedBatchSyntax,
            "batch-script command contains unsupported shell metacharacters",
        ),
        (
            WindowsCommandError::SystemRootUnavailable,
            "SystemRoot is unavailable",
        ),
        (
            WindowsCommandError::SystemToolUnavailable,
            "Windows system tool is unavailable",
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
        assert!(!error.to_string().contains("private-value"));
    }
}
