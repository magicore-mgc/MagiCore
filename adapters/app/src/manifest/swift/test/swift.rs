//! Tests stable Swift manifest helpers and the explicit writer boundary.
//! Kiểm tra helper manifest Swift ổn định và boundary writer tường minh.

use super::super::{parse_package_swift, scan_package_swift_text, write_package_swift};
use mgc_types::{Ecosystem, Manifest, MgError};

#[test]
fn text_scanner_keeps_the_public_dependency_shape() {
    let deps = scan_package_swift_text(
        "let package = Package(dependencies: [.package(id: \"scope.lib\", from: \"1.0.0\")])",
    );

    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].dep_name(), "scope/lib");
}

#[test]
fn parser_keeps_literal_manifest_requirements() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Package.swift"),
        "let package = Package(name: \"App\", dependencies: [.package(id: \"scope.lib\", from: \"1.0.0\")])\n",
    )
    .unwrap();

    let manifest = parse_package_swift(dir.path()).unwrap();

    assert_eq!(manifest.dependencies.len(), 1);
    assert_eq!(manifest.dependencies[0].name.as_str(), "scope/lib");
    assert_eq!(manifest.dependencies[0].range.as_str(), "from:1.0.0");
}

#[test]
fn writer_remains_explicitly_unsupported() {
    let dir = tempfile::tempdir().unwrap();
    let error = write_package_swift(dir.path(), &Manifest::new("app", Ecosystem::App)).unwrap_err();

    assert!(matches!(error, MgError::Unsupported { .. }));
    assert!(error.to_string().contains("write-swift-manifest"));
}
