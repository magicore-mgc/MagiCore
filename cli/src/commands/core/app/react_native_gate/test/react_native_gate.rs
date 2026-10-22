//! React Native gate branch tests.
//! Test các nhánh gate React Native.

use super::super::gate_react_native;
use crate::commands::compat::CompatMode;
use crate::commands::dep_gate::DepOp;

#[test]
fn gate_rejects_react_native_dependency_mutation() {
    let project = tempfile::tempdir().unwrap();
    let error = gate_react_native(
        project.path(),
        mgc_app_adapter::AppLanguage::ReactNative,
        DepOp::Add,
        &CompatMode::Native,
    )
    .unwrap_err();

    assert!(error.to_string().contains("unsupported"), "{error:#}");
}

#[test]
fn gate_is_noop_for_non_react_native_language() {
    let project = tempfile::tempdir().unwrap();
    gate_react_native(
        project.path(),
        mgc_app_adapter::AppLanguage::Flutter,
        DepOp::Add,
        &CompatMode::Native,
    )
    .unwrap();
    assert!(!project.path().join(".magicore").exists());
}
