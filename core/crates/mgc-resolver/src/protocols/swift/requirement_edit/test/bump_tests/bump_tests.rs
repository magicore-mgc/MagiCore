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
    "        .package(url: \"https://github.com/example/pinned.git\", branch: \"main\"),\n",
    "    ],\n",
    ")\n",
);

#[test]
fn bump_registry_from_requirement() {
    let out = bump_swift_requirement(PKG, "scope.lib", "1.1.0").expect("bump works");
    assert!(out.contains("from: \"1.1.0\""), "version bumped:\n{out}");
    assert!(!out.contains("from: \"1.0.0\""), "old pin gone:\n{out}");
    // Re-scan verifies the new pin (no toolchain needed).
    let deps = parse_swift_package_deps(&out);
    let hit = deps
        .iter()
        .find(|d| d.dep_name() == "scope/lib")
        .expect("re-scan finds dep");
    assert!(
        hit.requirement_text().contains("1.1.0"),
        "re-scan sees bump"
    );
}

#[test]
fn bump_git_exact_requirement() {
    let out = bump_swift_requirement(PKG, "example/other", "2.1.0").expect("bump works");
    assert!(out.contains("exact: \"2.1.0\""), "exact bumped:\n{out}");
}

#[test]
fn bump_refuses_branch_and_unknown() {
    // Branch pins have no version to bump — honest None, never a guess.
    assert!(bump_swift_requirement(PKG, "example/pinned", "9.9.9").is_none());
    assert!(bump_swift_requirement(PKG, "scope.missing", "1.0.0").is_none());
}
