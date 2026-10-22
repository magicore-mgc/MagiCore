//! Tests fail-closed static Package.swift shape validation.
//! Kiểm tra validation shape Package.swift tĩnh fail-closed.

use super::super::read_literal_dependencies;

#[test]
fn accepts_literal_package_dependency() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Package.swift");
    std::fs::write(
        &path,
        "let package = Package(name: \"App\", dependencies: [.package(id: \"scope.lib\", from: \"1.0.0\")])\n",
    )
    .unwrap();

    let deps = read_literal_dependencies(&path).unwrap();

    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].dep_name(), "scope/lib");
}

#[test]
fn rejects_computed_dependency_array() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Package.swift");
    std::fs::write(
        &path,
        "let package = Package(name: \"App\", dependencies: makeDependencies())\n",
    )
    .unwrap();

    let error = read_literal_dependencies(&path).unwrap_err();

    assert!(error.to_string().contains("computed"));
}
