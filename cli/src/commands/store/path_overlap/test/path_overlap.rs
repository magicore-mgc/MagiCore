//! Tests for component-based Store path overlap — test overlap path theo component.

use super::super::paths_overlap;

#[test]
fn detects_ancestor_and_descendant_but_not_string_prefix_siblings() {
    let directory = tempfile::tempdir().unwrap();
    let project = directory.path().join("project");
    let child = project.join("backup");
    let sibling = directory.path().join("project-backup");

    assert!(paths_overlap(&project, &child));
    assert!(paths_overlap(&child, &project));
    assert!(!paths_overlap(&project, &sibling));
}
