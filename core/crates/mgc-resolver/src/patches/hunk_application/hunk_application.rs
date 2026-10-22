//! Apply already-parsed hunks while preserving current newline behavior.
//! Áp dụng hunk đã phân tích, giữ nguyên cách xử lý newline hiện tại.

use crate::patches::{hunk_application::error::HunkApplicationError, types::Hunk};

pub(in crate::patches) fn apply_hunks(content: &str, hunks: Vec<Hunk>) -> anyhow::Result<String> {
    let trailing_newline = content.ends_with('\n');
    let line_ending = if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    for hunk in hunks {
        let start = hunk.old_start.saturating_sub(1);
        let end = start
            .checked_add(hunk.old_len)
            .ok_or_else(|| anyhow::Error::new(HunkApplicationError::RangeOverflow))?;
        if end > lines.len() {
            return Err(HunkApplicationError::RangeExceeds {
                end,
                actual: lines.len(),
            }
            .into());
        }
        let mut expected_old = Vec::new();
        for (op, content) in &hunk.lines {
            if matches!(op, ' ' | '-') {
                expected_old.push(content.clone());
            }
        }
        let actual_old: Vec<String> = lines[start..end].to_vec();
        if actual_old != expected_old {
            return Err(HunkApplicationError::ContextMismatch {
                line: start + 1,
                expected: expected_old,
                actual: actual_old,
            }
            .into());
        }
        let mut new_lines = Vec::new();
        for (op, content) in &hunk.lines {
            match op {
                ' ' | '+' => new_lines.push(content.clone()),
                '-' => {}
                _ => return Err(HunkApplicationError::InvalidOperation(*op).into()),
            }
        }
        lines.splice(start..end, new_lines);
    }
    let mut updated = lines.join(line_ending);
    if trailing_newline {
        updated.push_str(line_ending);
    }
    Ok(updated)
}

#[cfg(test)]
#[path = "test/hunk_application.rs"]
mod tests;
