//! Detect action references that are not pinned to a full commit SHA.
//! Phát hiện action reference chưa ghim vào commit SHA đầy đủ.

use crate::scanners::gh_actions::types::WorkflowFinding;

pub(crate) fn finding(file: &str, line: usize, key_line: &str) -> Option<WorkflowFinding> {
    let uses = key_line.strip_prefix("uses:")?;
    // Ignore trailing documentation comments and surrounding quotes, as before.
    // Bỏ qua comment ghi chú cuối và dấu nháy bao ngoài như hành vi cũ.
    let action_ref = uses
        .split('#')
        .next()
        .unwrap_or("")
        .trim()
        .trim_matches('\'')
        .trim_matches('"');
    let ref_part = action_ref.rsplit('@').next().unwrap_or("");
    if !action_ref.contains('@')
        || action_ref.ends_with('@')
        || ref_part.len() != 40
        || !ref_part
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        return Some(WorkflowFinding {
            file: file.to_string(),
            line,
            rule: "actions-unpinned-ref",
            detail: format!(
                "'{action_ref}' is not pinned to a full 40-hex commit SHA — mutable refs can be hijacked"
            ),
        });
    }
    None
}

#[cfg(test)]
#[path = "test/uses_reference.rs"]
mod tests;
