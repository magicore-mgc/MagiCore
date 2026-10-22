//! Parse file headers, hunk headers, ranges and line operations.
//! Phân tích header file, header hunk, phạm vi và thao tác từng dòng.

use std::iter::Peekable;

use super::error::HunkParseError;
use crate::patches::{path_validation::validate_patch_relative_path, types::Hunk};

pub(in crate::patches) fn parse_file_header<I>(
    lines: &mut Peekable<I>,
    prefix: &str,
) -> Result<String, HunkParseError>
where
    I: Iterator<Item = String>,
{
    let line = lines
        .next()
        .ok_or_else(|| HunkParseError::ExpectedHeader(prefix.to_string()))?;
    if !line.starts_with(prefix) {
        return Err(HunkParseError::ExpectedPrefix(prefix.to_string()));
    }
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(HunkParseError::InvalidHeader(prefix.to_string()));
    }
    let path = parts[1].trim_start_matches("a/").trim_start_matches("b/");
    validate_patch_relative_path(path)?;
    Ok(path.to_string())
}

pub(in crate::patches) fn parse_hunk<I>(
    header: String,
    lines: &mut Peekable<I>,
) -> Result<Hunk, HunkParseError>
where
    I: Iterator<Item = String>,
{
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(HunkParseError::InvalidHunkHeader(header));
    }
    let old_range = parse_range(parts[1])?;
    let _new_range = parse_range(parts[2])?;
    let mut hunk_lines = Vec::new();
    while let Some(line) = lines.peek() {
        if line.starts_with("@@") || line.starts_with("---") || line.starts_with("+++") {
            break;
        }
        let Some(line) = lines.next() else {
            return Err(HunkParseError::PatchLineStreamEnded);
        };
        if line.is_empty() {
            hunk_lines.push((' ', String::new()));
            continue;
        }
        let op = line.chars().next().unwrap_or(' ');
        if !matches!(op, ' ' | '+' | '-') {
            return Err(HunkParseError::InvalidOperation(op));
        }
        let content = line
            .get(op.len_utf8()..)
            .ok_or(HunkParseError::InvalidUtf8Boundary)?;
        hunk_lines.push((op, content.to_string()));
    }
    Ok(Hunk {
        old_start: old_range.0,
        old_len: old_range.1,
        lines: hunk_lines,
    })
}

fn parse_range(s: &str) -> Result<(usize, usize), HunkParseError> {
    let s = s.trim_start_matches('-').trim_start_matches('+');
    let parts: Vec<&str> = s.split(',').collect();
    let start = parts[0].parse::<usize>()?;
    let len = if parts.len() > 1 {
        parts[1].parse::<usize>()?
    } else {
        1
    };
    Ok((start, len))
}

#[cfg(test)]
#[path = "test/hunk_parser.rs"]
mod tests;
