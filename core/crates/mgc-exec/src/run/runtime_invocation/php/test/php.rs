// Focused tests for PHP's equals-form file option.
// Test tập trung cho file option dạng có dấu bằng của PHP.

use super::*;
use std::ffi::OsString;

#[test]
fn recognizes_equals_form_file_and_keeps_inline_code_opaque() {
    let file = vec![
        OsString::from("php"),
        OsString::from("--file=/project/composer.phar"),
    ];
    assert_eq!(
        php_script_argument(&file),
        Some((1, "/project/composer.phar".to_string()))
    );

    let inline = vec![OsString::from("php"), OsString::from("-r"), OsString::from("echo 1;")];
    assert_eq!(php_script_argument(&inline), None);
}
