//! Verify quoted fields and precise malformed-row classifications.
//! Kiểm tra field có quote và phân loại chính xác dòng lỗi.

use super::super::{RecordCsvError, parse_record_row};

#[test]
fn record_csv_preserves_quoted_comma_and_escaped_quote() {
    let row = parse_record_row("\"pkg/with,comma\"\"name.py\",sha256=YQ,7")
        .expect("test operation must succeed");
    assert_eq!(row, ["pkg/with,comma\"name.py", "sha256=YQ", "7"]);
}

#[test]
fn record_csv_returns_owner_errors_for_malformed_rows() {
    assert_eq!(
        parse_record_row("a,sha256=YQ"),
        Err(RecordCsvError::ExpectedThreeFields)
    );
    assert_eq!(
        parse_record_row("\"unterminated,sha256=YQ,1"),
        Err(RecordCsvError::UnterminatedQuotedField)
    );
    assert_eq!(
        parse_record_row("\"closed\"tail,sha256=YQ,1"),
        Err(RecordCsvError::UnexpectedDataAfterQuotedField)
    );
}
