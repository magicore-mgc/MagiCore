//! Single Core help owner tests — test owner help single core.
//! Pin the nearest behavior contract without embedding test bodies in production files.
//! Giữ contract behavior gần owner, không đặt test body trong source production.

use super::append_single_core_section;

#[test]
fn single_core_section_contains_the_four_shortcuts() {
    let mut output = String::new();
    append_single_core_section(&mut output);

    assert!(output.contains("Add a dependency"));
    assert!(output.contains("Remove a dependency"));
    assert!(output.contains("Update packages"));
    assert!(output.contains("List installed packages"));
}
