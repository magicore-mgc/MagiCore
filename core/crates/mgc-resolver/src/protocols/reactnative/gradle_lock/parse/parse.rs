//! Parse pinned Gradle coordinates without silently dropping malformed rows.
//! Parse tọa độ Gradle đã ghim, không âm thầm bỏ dòng lỗi.

use super::super::{GradleLockEntry, error::GradleLockError};

pub(in crate::protocols::reactnative::gradle_lock) fn parse_entries(
    text: &str,
) -> Result<Vec<GradleLockEntry>, GradleLockError> {
    let mut out = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 3 {
            return Err(GradleLockError::MalformedLine {
                line: idx + 1,
                input: line.to_string(),
            });
        }
        out.push(GradleLockEntry {
            group: parts[0].to_string(),
            artifact: parts[1].to_string(),
            version: parts[2].to_string(),
            configuration: parts.get(3).unwrap_or(&"").to_string(),
        });
    }
    Ok(out)
}

#[cfg(test)]
#[path = "test/parse.rs"]
mod tests;
