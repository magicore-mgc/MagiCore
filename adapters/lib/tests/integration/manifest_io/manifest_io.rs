//! Lib manifest I/O tests — kiểm thử I/O manifest Lib.
//! Owns Rust/Python round trips and TS workspace projection. — Sở hữu roundtrip Rust/Python và projection TS.

use mgc_lib_adapter::adapter_for;
use mgc_types::adapter::PackageAdapter;
use mgc_types::capabilities::LockfileProvider;
use mgc_types::{DependencySpec, PackageName, VersionRange};

use super::support::tmp;

pub(crate) async fn rust_manifest_roundtrip() {
    let dir = tmp("manifest-rt");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"roundtrip\"\nversion = \"0.1.0\"\n\n[package.metadata.magicore]\ncore = \"lib\"\n\n[dependencies]\nserde = \"1\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    let manifest = a.parse_manifest(&dir).await.unwrap();
    assert_eq!(manifest.name, "roundtrip");
    assert_eq!(manifest.dependencies.len(), 1);
    assert_eq!(manifest.dependencies[0].name.as_str(), "serde");
}

pub(crate) async fn rust_write_manifest_preserves_magicore_metadata() {
    let dir = tmp("rust-write");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[package.metadata.magicore]\ncore = \"lib\"\n\n[dependencies]\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    let mut manifest = a.parse_manifest(&dir).await.unwrap();
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("serde").unwrap(),
            VersionRange::parse("1").unwrap(),
        ),
        false,
        false,
        false,
    );
    a.write_manifest(&dir, &manifest).await.unwrap();
    let content = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(content.contains("magicore"));
    assert!(content.contains("serde"));
}

pub(crate) async fn python_manifest_roundtrip_preserves_project_table() {
    let dir = tmp("py-manifest");
    std::fs::write(
        dir.join("pyproject.toml"),
        "[project]\nname = \"py-lib\"\nversion = \"0.1.0\"\ndependencies = [\"requests>=2.32.3\"]\n\n[tool.magicore]\ncore = \"lib\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    let manifest = a.parse_manifest(&dir).await.unwrap();
    assert_eq!(manifest.name, "py-lib");
    assert!(manifest.find_dep("requests").is_some());
}

pub(crate) async fn ts_delegate_ignores_workspace_protocol_dependencies() {
    let dir = tmp("ts-workspace");
    std::fs::write(
        dir.join("package.json"),
        serde_json::json!({
            "name": "frontend",
            "version": "0.1.0",
            "dependencies": {
                "@core/shared": "workspace:*",
                "react": "^18.2.0"
            }
        })
        .to_string(),
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    let manifest = a.parse_manifest(&dir).await.unwrap();
    assert!(manifest.find_dep("react").is_some());
    assert!(manifest.find_dep("@core/shared").is_none());
}
