//! Preserve the existing flutter_sdk_disjunctive_constraints_fail_closed_when_intersection_is_needed Flutter SDK behavior assertion.
//! Giữ nguyên assertion hành vi Flutter SDK flutter_sdk_disjunctive_constraints_fail_closed_when_intersection_is_needed.

#[test]
fn flutter_sdk_disjunctive_constraints_fail_closed_when_intersection_is_needed() {
    let project = tempfile::tempdir().unwrap();
    let sdk = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        "name: sample\ndependencies:\n  flutter:\n    sdk: flutter\n  meta: '>=1.17.0 <2.0.0 || >=3.0.0 <4.0.0'\n",
    )
    .unwrap();
    std::fs::create_dir_all(sdk.path().join("packages/flutter/lib")).unwrap();
    std::fs::write(
        sdk.path().join("packages/flutter/pubspec.yaml"),
        "name: flutter\ndependencies:\n  meta: '>=1.18.0 <2.0.0'\n",
    )
    .unwrap();

    let base =
        crate::manifest::parse_manifest(crate::AppLanguage::Flutter, project.path()).unwrap();
    let error = crate::manifest::flutter::expand_flutter_sdk_dependencies_with_root(
        project.path(),
        &base,
        sdk.path(),
    )
    .expect_err("constraint intersection must not weaken an OR expression");
    assert!(error.to_string().contains("disjunctive constraints"));
}
