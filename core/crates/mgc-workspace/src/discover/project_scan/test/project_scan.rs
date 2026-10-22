//! Workspace project target scanning contract tests.
//! Test hợp đồng cho việc quét target project workspace.

#![allow(clippy::unwrap_used)]

use super::super::has_backend_manifest;
use crate::discover_workspace_targets;

#[test]
fn discover_apps_and_packages() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    std::fs::create_dir_all(root.join("apps/app1")).unwrap();
    std::fs::create_dir_all(root.join("apps/app2")).unwrap();
    std::fs::create_dir_all(root.join("packages/shared")).unwrap();
    std::fs::create_dir_all(root.join("packages/deep/nested/pkg")).unwrap();
    for dir in [
        "apps/app1",
        "apps/app2",
        "packages/shared",
        "packages/deep/nested/pkg",
    ] {
        std::fs::write(root.join(dir).join("package.json"), "{}").unwrap();
    }
    std::fs::create_dir_all(root.join("packages/empty")).unwrap();

    let targets = discover_workspace_targets(root).unwrap();
    let names: Vec<String> = targets
        .iter()
        .map(|t| t.strip_prefix(root).unwrap().to_string_lossy().to_string())
        .collect();
    assert_eq!(
        names,
        [
            "apps/app1",
            "apps/app2",
            "packages/deep/nested/pkg",
            "packages/shared"
        ]
    );
}

#[test]
fn backend_manifest_names_are_recognized() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let manifest_names = [
        "mgc.toml",
        "pyproject.toml",
        "west.yml",
        "platformio.ini",
        "project.godot",
        "go.mod",
        "Cargo.toml",
        "manage.py",
        "main.py",
        "src/main.py",
        "pom.xml",
        "artisan",
        "composer.json",
    ];

    for (index, &name) in manifest_names.iter().enumerate() {
        let candidate = root.join(format!("candidate-{index}"));
        let manifest = candidate.join(name);
        std::fs::create_dir_all(manifest.parent().unwrap()).unwrap();
        std::fs::write(manifest, "").unwrap();
        assert!(
            has_backend_manifest(&candidate),
            "manifest {name} should be recognized"
        );
    }

    assert!(!has_backend_manifest(&root.join("empty")));
}
