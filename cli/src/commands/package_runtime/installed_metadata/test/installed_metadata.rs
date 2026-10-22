//! Focused installed metadata regression tests.
//! Test hồi quy tập trung cho installed metadata.

use super::*;
use std::path::PathBuf;

#[test]
fn requested_package_bin_metadata_is_owned_and_typed() {
    let root = tempfile::tempdir().expect("isolated installed package");
    for (manifest, expected) in [
        (
            serde_json::json!({"name":"@scope/pkg","bin":"cli.js"}),
            vec![("pkg".to_string(), PathBuf::from("cli.js"))],
        ),
        (
            serde_json::json!({"name":"@scope/pkg","bin":{"tool":"bin/tool.js"}}),
            vec![("tool".to_string(), PathBuf::from("bin/tool.js"))],
        ),
        (serde_json::json!({"name":"@scope/pkg"}), vec![]),
    ] {
        std::fs::write(root.path().join("package.json"), manifest.to_string())
            .expect("write installed manifest");
        assert_eq!(
            requested_package_bins(root.path(), "@scope/pkg").expect("owned typed manifest"),
            expected
        );
    }
    for manifest in [
        serde_json::json!({"name":"dependency","bin":"cli.js"}),
        serde_json::json!({"name":"@scope/pkg","bin":{"tool":true}}),
        serde_json::json!({"name":"@scope/pkg","bin":["tool"]}),
    ] {
        std::fs::write(root.path().join("package.json"), manifest.to_string())
            .expect("write invalid manifest");
        assert!(requested_package_bins(root.path(), "@scope/pkg").is_err());
    }
}

#[test]
fn requested_package_version_requires_an_owned_semver_identity() {
    let root = tempfile::tempdir().expect("isolated installed package");
    std::fs::write(
        root.path().join("package.json"),
        r#"{"name":"create-vite","version":"9.2.1"}"#,
    )
    .expect("write package identity");
    assert_eq!(
        requested_package_version(root.path(), "create-vite").unwrap(),
        "9.2.1"
    );
    assert!(requested_package_version(root.path(), "other-package").is_err());

    std::fs::write(
        root.path().join("package.json"),
        r#"{"name":"create-vite","version":"latest"}"#,
    )
    .expect("write invalid package version");
    assert!(requested_package_version(root.path(), "create-vite").is_err());
}

#[cfg(unix)]
#[test]
fn requested_package_metadata_symlink_is_rejected() {
    let root = tempfile::tempdir().expect("isolated installed package");
    let foreign = tempfile::NamedTempFile::new().expect("foreign metadata");
    std::fs::write(foreign.path(), r#"{"name":"pkg","bin":"cli.js"}"#)
        .expect("write foreign metadata");
    std::os::unix::fs::symlink(foreign.path(), root.path().join("package.json"))
        .expect("redirect installed metadata");
    assert!(requested_package_bins(root.path(), "pkg").is_err());
}
