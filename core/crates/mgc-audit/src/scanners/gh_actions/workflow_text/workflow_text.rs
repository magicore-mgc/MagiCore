//! Scan one workflow while preserving the existing policy order.
//! Quét một workflow và giữ nguyên thứ tự policy hiện tại.

use super::permissions;
use super::pull_request_target;
use super::uses_reference;
use crate::scanners::gh_actions::types::WorkflowFinding;
use mgc_types::MgResult;

/// One workflow text to policy findings.
/// Văn bản một workflow thành các policy finding.
pub fn scan_workflow_text(file: &str, raw: &str) -> MgResult<Vec<WorkflowFinding>> {
    let mut findings = Vec::new();
    let mut has_top_permissions = false;
    let mut saw_pull_request_target = false;
    let mut event_block_indent: Option<usize> = None;

    for (idx, line) in raw.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        // Step bullets are common; strip one prefix for key checks.
        // Step bullet thường gặp; bỏ một prefix cho các kiểm tra key.
        let key_line = trimmed.strip_prefix("- ").unwrap_or(trimmed);

        if let Some(finding) = uses_reference::finding(file, line_no, key_line) {
            findings.push(finding);
        }

        permissions::observe_top_level(indent, trimmed, &mut has_top_permissions);
        pull_request_target::observe_trigger(
            indent,
            trimmed,
            &mut event_block_indent,
            &mut saw_pull_request_target,
        );

        if let Some(finding) =
            pull_request_target::injection_finding(file, line_no, key_line, saw_pull_request_target)
        {
            findings.push(finding);
        }
    }

    if let Some(finding) = permissions::missing_finding(file, has_top_permissions) {
        findings.push(finding);
    }
    Ok(findings)
}

#[cfg(test)]
#[path = "test/workflow_text.rs"]
mod tests;
