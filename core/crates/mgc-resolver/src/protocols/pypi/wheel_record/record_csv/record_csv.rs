//! Parse one three-field CSV row, including quoted commas and escaped quotes.
//! Parse một dòng CSV ba trường, gồm dấu phẩy trong quote và quote escape.

use super::error::RecordCsvError;

pub(in crate::protocols::pypi::wheel_record) fn parse_record_row(
    line: &str,
) -> Result<Vec<String>, RecordCsvError> {
    let mut fields = Vec::with_capacity(3);
    let mut field = String::new();
    let mut chars = line.chars().peekable();
    let mut quoted = false;
    let mut closed_quote = false;
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted => {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    quoted = false;
                    closed_quote = true;
                }
            }
            '"' if field.is_empty() && !closed_quote => quoted = true,
            ',' if !quoted => {
                fields.push(std::mem::take(&mut field));
                closed_quote = false;
                if fields.len() >= 3 {
                    return Err(RecordCsvError::ExpectedThreeFields);
                }
            }
            _ if quoted => field.push(ch),
            _ if closed_quote => return Err(RecordCsvError::UnexpectedDataAfterQuotedField),
            _ => field.push(ch),
        }
    }
    if quoted {
        return Err(RecordCsvError::UnterminatedQuotedField);
    }
    fields.push(field);
    if fields.len() != 3 {
        return Err(RecordCsvError::ExpectedThreeFields);
    }
    Ok(fields)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
