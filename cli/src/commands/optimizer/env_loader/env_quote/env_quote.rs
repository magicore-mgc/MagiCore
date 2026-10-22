//! Validate and strip balanced outer environment quotes.
//! (Kiểm tra và bỏ quote ngoài của giá trị environment.)

use anyhow::Result;

/// Strip balanced outer quotes; report only the line number on malformed input.
/// Bỏ quote ngoài cân bằng; khi lỗi chỉ báo số dòng, không lộ giá trị.
pub(in crate::commands::optimizer::env_loader) fn strip_env_quotes(
    value: &str,
    line_number: usize,
) -> Result<String> {
    let bytes = value.as_bytes();
    let first = bytes.first().copied();
    let last = bytes.last().copied();

    match (first, last) {
        (Some(b'"'), Some(b'"')) if value.len() >= 2 => Ok(value[1..value.len() - 1].to_string()),
        (Some(b'\''), Some(b'\'')) if value.len() >= 2 => Ok(value[1..value.len() - 1].to_string()),
        (Some(b'"' | b'\''), _) => {
            Err(crate::commands::optimizer::env_loader::error::unmatched_quote(line_number))
        }
        _ => Ok(value.to_string()),
    }
}
