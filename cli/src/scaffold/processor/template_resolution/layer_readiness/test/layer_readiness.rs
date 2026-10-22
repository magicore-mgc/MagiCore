//! Tests for template contract readiness and its existing refusal path.
//! Kiểm thử readiness contract template và đường từ chối hiện có.

#![allow(clippy::unwrap_used)]

use crate::scaffold::processor::Scaffolder;
use crate::scaffold::template_root::TemplateRoot;

#[test]
fn template_contract_requires_manifest_and_sources_directory() {
    let root = tempfile::tempdir().unwrap();
    let layer = TemplateRoot::disk(root.path().to_path_buf());

    std::fs::write(root.path().join("template.toml"), "[files]\n").unwrap();
    assert!(!Scaffolder::layer_has_contract(&layer));

    std::fs::create_dir(root.path().join("sources")).unwrap();
    assert!(Scaffolder::layer_has_contract(&layer));
}

#[test]
fn readiness_keeps_success_and_not_implemented_error_behavior() {
    let ready = tempfile::tempdir().unwrap();
    std::fs::write(ready.path().join("template.toml"), "[files]\n").unwrap();
    std::fs::create_dir(ready.path().join("sources")).unwrap();
    let ready_layer = TemplateRoot::disk(ready.path().to_path_buf());
    assert!(Scaffolder::ensure_web_layer_ready(&ready_layer, "fixture").is_ok());

    let missing = tempfile::tempdir().unwrap();
    let missing_layer = TemplateRoot::disk(missing.path().to_path_buf());
    assert!(Scaffolder::ensure_web_layer_ready(&missing_layer, "fixture").is_err());
}
