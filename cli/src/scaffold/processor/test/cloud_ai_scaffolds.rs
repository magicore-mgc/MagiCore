// Cloud and AI scaffold contract tests.
// Kiểm thử hợp đồng scaffold Cloud và AI.

#[test]
fn cdk_scaffold_contains_a_synthesizable_app_and_real_smoke_test() {
    let root = tempfile::tempdir().unwrap();

    crate::scaffold::processors::clo::CloProcessor::files(root.path(), "sample", "cdk").unwrap();

    let package: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.path().join("package.json")).unwrap()).unwrap();
    assert_eq!(
        package["scripts"]["test"],
        "node --test test/scaffold.test.cjs"
    );
    assert!(package["dependencies"]["aws-cdk-lib"].is_string());
    assert!(package["devDependencies"]["aws-cdk"].is_string());
    assert!(root.path().join("cdk.json").is_file());
    assert!(root.path().join("bin/app.js").is_file());
    assert!(root.path().join("test/scaffold.test.cjs").is_file());
}
#[test]
fn pulumi_scaffold_contains_a_runnable_program_and_real_smoke_test() {
    let root = tempfile::tempdir().unwrap();

    crate::scaffold::processors::clo::CloProcessor::files(root.path(), "sample", "pulumi").unwrap();

    let package: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.path().join("package.json")).unwrap()).unwrap();
    assert_eq!(
        package["scripts"]["test"],
        "node --test test/scaffold.test.cjs"
    );
    assert!(package["dependencies"]["@pulumi/pulumi"].is_string());
    assert!(root.path().join("Pulumi.yaml").is_file());
    assert!(root.path().join("index.js").is_file());
    assert!(root.path().join("test/scaffold.test.cjs").is_file());
}
