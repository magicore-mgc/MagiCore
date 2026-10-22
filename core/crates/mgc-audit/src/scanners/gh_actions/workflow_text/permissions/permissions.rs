//! Detect the top-level permissions declaration and missing-policy finding.
//! Nhận diện khai báo permissions cấp gốc và finding thiếu policy.

use crate::scanners::gh_actions::types::WorkflowFinding;

pub(crate) fn observe_top_level(indent: usize, trimmed: &str, found: &mut bool) {
    if indent == 0 && trimmed.starts_with("permissions:") {
        *found = true;
    }
}

pub(crate) fn missing_finding(file: &str, has_top_permissions: bool) -> Option<WorkflowFinding> {
    if has_top_permissions {
        return None;
    }
    Some(WorkflowFinding {
        file: file.to_string(),
        line: 0,
        rule: "permissions-undeclared",
        detail: "workflow has no top-level `permissions:` block — defaults may grant write"
            .to_string(),
    })
}

#[cfg(test)]
#[path = "test/permissions.rs"]
mod tests;
