// lockfile_graph_closure_tests — dependency and peer closure.
// lockfile_graph_closure_tests — kiểm thử dependency and peer closure.
use super::*;

#[test]
fn web_resolver_prunes_lock_orphans_but_keeps_dependency_and_peer_closure() {
    let package_id = |name: &str, version: &str| {
        PackageId::new(
            PackageName::new(name).unwrap(),
            Version::parse(version).unwrap(),
        )
    };
    let root = package_id("root", "1.0.0");
    let child = package_id("child", "2.0.0");
    let peer = package_id("react", "19.3.0");
    let orphan = package_id("strip-ansi-cjs", "6.0.1");
    let mut graph = ResolvedGraph {
        packages: vec![
            ResolvedPackage {
                id: root,
                integrity: String::new(),
                tarball_url: String::new(),
                deps: vec![child.clone()],
                peer_deps: vec![peer.clone()],
                direct: true,
                dev: false,
            },
            ResolvedPackage {
                id: child.clone(),
                integrity: String::new(),
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: false,
                dev: false,
            },
            ResolvedPackage {
                id: peer.clone(),
                integrity: String::new(),
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: false,
                dev: false,
            },
            ResolvedPackage {
                id: orphan,
                integrity: String::new(),
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: false,
                dev: false,
            },
        ],
    };

    mgc_js_service::resolve::prune_unreachable_packages(&mut graph);

    let retained: std::collections::HashSet<_> = graph
        .packages
        .iter()
        .map(|package| package.id.to_string())
        .collect();
    assert_eq!(retained.len(), 3);
    assert!(retained.contains(&child.to_string()));
    assert!(retained.contains(&peer.to_string()));
    assert!(!retained.contains("strip-ansi-cjs@6.0.1"));
}
