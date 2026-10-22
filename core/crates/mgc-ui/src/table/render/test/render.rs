//! Table render tests — kiểm thử render table.
//! Pure line checks do not redirect process-wide stdout.
//! Kiểm tra dòng thuần mà không chuyển hướng stdout toàn tiến trình.

use super::render::{header_line, row_line, separator_line};

#[test]
fn header_keeps_styled_column_text_and_spacing() {
    let headers = vec!["Package".to_string(), "Version".to_string()];
    let line = header_line(&headers);
    assert!(line.contains("Package"));
    assert!(line.contains("Version"));
    assert!(line.contains("  "));
}

#[test]
fn separator_is_eighty_horizontal_rules() {
    let line = separator_line();
    assert_eq!(line.chars().count(), 80);
    assert!(line.chars().all(|character| character == '─'));
}

#[test]
fn rows_keep_two_space_delimiters_and_order() {
    let row = vec!["alpha".to_string(), "1.0".to_string()];
    assert_eq!(row_line(&row), "alpha  1.0");
}
