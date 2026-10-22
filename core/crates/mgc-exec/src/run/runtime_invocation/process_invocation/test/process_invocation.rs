// Focused tests for normalized manager script names.
// Test tập trung cho tên script manager sau chuẩn hóa.

use super::*;

#[test]
fn strips_script_extensions_after_token_normalization() {
    assert_eq!(process_invocation_program("/tools/npm.sh"), "npm");
    assert_eq!(process_invocation_program("/tools/pnpm-cli.js"), "pnpm");
}
