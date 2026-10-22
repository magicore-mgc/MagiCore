// Focused tests for stdin mode and option termination.
// Test tập trung cho chế độ stdin và dấu kết thúc option.

use super::*;
use std::ffi::OsString;

#[test]
fn leaves_stdin_source_opaque_and_accepts_file_after_terminator() {
    let stdin = vec![OsString::from("sh"), OsString::from("-s"), OsString::from("npm install")];
    assert_eq!(shell_script_argument(&stdin), None);

    let file = vec![OsString::from("sh"), OsString::from("--"), OsString::from("/project/npm")];
    assert_eq!(
        shell_script_argument(&file),
        Some((2, "/project/npm".to_string()))
    );
}
