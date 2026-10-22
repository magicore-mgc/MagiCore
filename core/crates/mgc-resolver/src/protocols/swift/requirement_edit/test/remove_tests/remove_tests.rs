use super::*;

use crate::protocols::swift::parse_swift_package_deps;

const PKG: &str = concat!(
    "// swift-tools-version: 5.9\n",
    "import PackageDescription\n\n",
    "let package = Package(\n",
    "    name: \"demo\",\n",
    "    dependencies: [\n",
    "        .package(id: \"scope.lib\", from: \"1.0.0\"),\n",
    "        .package(url: \"https://github.com/example/other.git\", exact: \"2.0.0\"),\n",
    "    ],\n",
    ")\n",
);

#[test]
fn remove_registry_call() {
    let out = remove_swift_requirement(PKG, "scope.lib").expect("remove works");
    assert!(!out.contains("scope.lib"), "dep gone:\n{out}");
    assert!(out.contains("example/other"), "other survives:\n{out}");
    // Re-scan proves exactly one removal.
    let deps = parse_swift_package_deps(&out);
    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].dep_name(), "github.com/example/other");
}

#[test]
fn remove_git_call_by_tail() {
    let out = remove_swift_requirement(PKG, "example/other").expect("remove works");
    assert!(!out.contains("other.git"), "dep gone");
    assert!(out.contains("scope.lib"), "other survives");
}

#[test]
fn remove_unknown_is_none() {
    assert!(remove_swift_requirement(PKG, "scope.missing").is_none());
}
