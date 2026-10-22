//! Typed failures for the three-column wheel RECORD CSV grammar.
//! Lỗi typed cho cú pháp CSV ba cột của wheel RECORD.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::protocols::pypi::wheel_record) enum RecordCsvError {
    ExpectedThreeFields,
    UnterminatedQuotedField,
    UnexpectedDataAfterQuotedField,
}

impl fmt::Display for RecordCsvError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::ExpectedThreeFields => "expected exactly three CSV fields",
            Self::UnterminatedQuotedField => "unterminated quoted CSV field",
            Self::UnexpectedDataAfterQuotedField => "unexpected data after quoted CSV field",
        };
        formatter.write_str(message)
    }
}
