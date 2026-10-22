//! Track the trigger block and flag untrusted PR expressions.
//! Theo dõi trigger block và gắn cờ expression PR không tin cậy.

use crate::scanners::gh_actions::types::WorkflowFinding;

pub(crate) fn observe_trigger(
    indent: usize,
    trimmed: &str,
    event_block_indent: &mut Option<usize>,
    saw_pull_request_target: &mut bool,
) {
    if indent == 0 && trimmed.starts_with("on:") {
        *event_block_indent = Some(indent);
    }
    if let Some(block_indent) = *event_block_indent
        && indent == block_indent + 2
        && trimmed == "pull_request_target:"
    {
        *saw_pull_request_target = true;
    }
    // A later non-empty top-level key closes the event block.
    // Key cấp gốc không rỗng xuất hiện sau đó sẽ đóng event block.
    if let Some(block_indent) = *event_block_indent
        && indent <= block_indent
        && !trimmed.is_empty()
        && !trimmed.starts_with("on:")
        && !trimmed.starts_with("pull_request_target")
    {
        *event_block_indent = None;
    }
}

pub(crate) fn injection_finding(
    file: &str,
    line: usize,
    key_line: &str,
    saw_pull_request_target: bool,
) -> Option<WorkflowFinding> {
    if saw_pull_request_target
        && (key_line.starts_with("run:") || key_line.starts_with("ref:"))
        && key_line.contains("github.event.pull_request")
    {
        return Some(WorkflowFinding {
            file: file.to_string(),
            line,
            rule: "pull_request_target-injection",
            detail: format!(
                "pull_request_target workflow evaluates untrusted PR expressions ({}) — runs with base secrets",
                key_line.trim()
            ),
        });
    }
    None
}

#[cfg(test)]
#[path = "test/pull_request_target.rs"]
mod tests;
