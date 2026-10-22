//! Section-header tests — kiểm thử heading từng bước.
//! Pure line checks avoid redirecting process-wide stdout.
//! Kiểm tra dòng thuần, không chuyển hướng stdout toàn tiến trình.

use super::section::section_lines;

#[test]
fn section_lines_keep_step_title_and_divider() {
    let lines = section_lines("Resolve packages", 2, 5);
    assert!(lines[0].contains("Step 2/5"));
    assert!(lines[1].contains("Resolve packages"));
    assert!(lines[2].contains("─"));
}
