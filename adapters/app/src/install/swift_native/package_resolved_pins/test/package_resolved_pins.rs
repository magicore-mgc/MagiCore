//! Behavior tests for the package_resolved_pins owner.
//! Test hành vi cho owner package_resolved_pins.

use super::super::*;
use mgc_types::{PackageId, ResolvedGraph, ResolvedPackage};

fn package(name: &str, version: &str, tarball_url: &str) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::parse(&format!("{name}@{version}")).unwrap(),
        integrity: String::new(),
        tarball_url: tarball_url.to_string(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: true,
        dev: false,
    }
}

#[test]
fn pin_projection_preserves_graph_order_and_source_specific_fields() {
    let graph = ResolvedGraph {
        packages: vec![
            package("scope/lib", "1.4.2", "https://registry.test/scope/lib.zip"),
            package(
                "github.com/Owner/Repo",
                "2.0.0",
                "https://git.test/Owner/Repo",
            ),
        ],
    };
    let mut lock_package = mgc_lockfile::Package::new(
        "github.com/Owner/Repo".into(),
        "2.0.0".into(),
        String::new(),
        String::new(),
    );
    lock_package.markers = Some(vec!["git-commit:abc123".into()]);

    let pins = build_swift_pins(&graph, &[lock_package]);

    assert_eq!(pins.len(), 2);
    assert_eq!(pins[0].identity, "scope.lib");
    assert_eq!(pins[0].version.as_deref(), Some("1.4.2"));
    assert_eq!(pins[0].revision, None);
    assert_eq!(pins[0].location, None);
    assert_eq!(pins[1].identity, "repo");
    assert_eq!(pins[1].version.as_deref(), Some("2.0.0"));
    assert_eq!(pins[1].revision.as_deref(), Some("abc123"));
    assert_eq!(pins[1].branch, None);
    assert_eq!(
        pins[1].location.as_deref(),
        Some("https://github.com/Owner/Repo.git")
    );
}
