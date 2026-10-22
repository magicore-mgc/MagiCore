use crate::policy_manifest::ManifestEntry;

#[test]
fn entry_keeps_every_exact_match_dimension_and_explanation() {
    let entry = ManifestEntry {
        program: "npm".into(),
        args_exact: vec!["--version".into()],
        framework: "next".into(),
        framework_version: "16.3.8".into(),
        os: "darwin".into(),
        operation: "dev".into(),
        reason: "probe".into(),
        expected: "0.0.0".into(),
    };
    assert_eq!(entry.program, "npm");
    assert_eq!(entry.args_exact[0], "--version");
    assert_eq!(entry.framework, "next");
    assert_eq!(entry.framework_version, "16.3.8");
    assert_eq!(entry.os, "darwin");
    assert_eq!(entry.operation, "dev");
    assert_eq!(entry.reason, "probe");
    assert_eq!(entry.expected, "0.0.0");
}
