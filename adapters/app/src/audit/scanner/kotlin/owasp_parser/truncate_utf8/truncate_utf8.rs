//! Truncates external advisory text at UTF-8 character boundaries.
//! Cắt text advisory bên ngoài tại ranh giới ký tự UTF-8.

/// Truncate to at most `max_chars` chars, respecting char (not byte)
/// boundaries — advisory text is external data and may contain any UTF-8.
/// Cắt tối đa `max_chars` ký tự, đúng ranh giới ký tự (không phải byte) —
/// text advisory là dữ liệu ngoài, có thể chứa UTF-8 bất kỳ.
#[cfg(test)]
pub(in crate::audit::scanner::kotlin::owasp_parser) fn truncate_utf8(
    text: &str,
    max_chars: usize,
) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    text.chars().take(max_chars).collect()
}
