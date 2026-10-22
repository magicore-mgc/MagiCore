use super::super::for_each_prefixed_line;

#[test]
fn prefixes_lines_in_order_and_handles_empty_output() {
    let mut lines = Vec::new();
    for_each_prefixed_line(4, b"first\nsecond\n", |line| lines.push(line));
    assert_eq!(
        lines,
        vec!["[4] first".to_string(), "[4] second".to_string()]
    );
    lines.clear();
    for_each_prefixed_line(4, b"", |line| lines.push(line));
    assert!(lines.is_empty());
}
