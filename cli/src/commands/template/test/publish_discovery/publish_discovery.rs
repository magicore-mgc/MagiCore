use tempfile;

use super::super::*;

#[test]
fn publish_all_core_summary_lists_only_cores_with_template_layers() {
    let root = tempfile::tempdir().unwrap();
    let layers = vec![
        root.path().join("web/frontend/vanilla"),
        root.path().join("ai/agents/python-agent"),
        root.path().join("web/shared/partials/base"),
    ];

    assert_eq!(
        published_template_cores(root.path(), &layers).unwrap(),
        ["ai", "web"]
    );
}

#[test]
fn publish_all_rejects_a_layer_outside_the_template_root() {
    let root = tempfile::tempdir().unwrap();
    let outside = root.path().parent().unwrap().join("outside-template-layer");

    assert!(published_template_cores(root.path(), &[outside]).is_err());
}
