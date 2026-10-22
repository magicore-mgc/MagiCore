//! Workspace-discovery coordinator contract tests.
//! Test hợp đồng cho coordinator tìm workspace.

#![allow(clippy::unwrap_used)]

use crate::discover_workspace_targets;

#[test]
fn discover_returns_empty_when_no_dirs() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let targets = discover_workspace_targets(root).unwrap();
    assert!(targets.is_empty());
}
