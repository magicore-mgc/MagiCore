use super::super::parse_swift_package_deps;
use crate::protocols::swift::SwiftDep;

#[test]
fn package_swift_scan_registry_git_and_requirements() {
    let text = r#"
// swift-tools-version:5.9
let package = Package(
    name: "App",
    dependencies: [
        .package(id: "scope.lib", from: "1.0.0"),
        .package(url: "https://github.com/example/repo.git", .upToNextMajor(from: "2.1.0")),
        .package(url: "https://github.com/example/pinned.git", exact: "3.0.0"),
        .package(url: "https://github.com/example/br.git", branch: "main"),
        .package(id: "other.lib", "4.0.0"..<"5.0.0"),
        .packages([])  // not a dependency
    ]
)"#;
    let deps = parse_swift_package_deps(text);
    assert_eq!(deps.len(), 5, "{deps:?}");
    assert_eq!(
        deps[0],
        SwiftDep::Registry {
            identity: "scope.lib".into(),
            requirement: "from:1.0.0".into()
        }
    );
    assert_eq!(deps[0].dep_name(), "scope/lib");
    assert_eq!(
        deps[1],
        SwiftDep::Git {
            url: "https://github.com/example/repo.git".into(),
            requirement: "from:2.1.0".into()
        }
    );
    assert_eq!(deps[1].dep_name(), "github.com/example/repo");
    assert_eq!(deps[2].requirement_text(), "exact:3.0.0", "{:?}", deps[2]);
    assert_eq!(deps[3].requirement_text(), "branch:main", "{:?}", deps[3]);
    assert_eq!(
        deps[4].requirement_text(),
        "range:4.0.0..<5.0.0",
        "{:?}",
        deps[4]
    );
}
