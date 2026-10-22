// Focused test for /k and first-operand-only inspection.
// Test tập trung cho /k và chỉ xem operand đầu tiên.

use super::*;
use std::ffi::OsString;

#[test]
fn reads_only_the_first_command_operand() {
    let argv = vec![
        OsString::from("cmd.exe"),
        OsString::from("/k"),
        OsString::from("echo npm install"),
    ];
    let invocation = windows_cmd_invocation(&argv).expect("cmd /k has a first command");
    assert_eq!(invocation.1, "echo");
    assert_eq!(invocation.2, vec!["npm".to_string(), "install".to_string()]);
}
