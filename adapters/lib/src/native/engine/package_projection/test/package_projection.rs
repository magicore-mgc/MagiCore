//! Package Projection source regression test.
//! Test hồi quy source cho package projection.

use std::collections::HashSet;

use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::ResolvedEntry;

use crate::native::engine::engine::entry_index::index_entries;
use crate::native::engine::engine::package_projection::project_packages;

#[test]
fn projects_dependency_edges_directness_and_rust_lock_keys() {
    let entries = vec![
        ResolvedEntry {
            name: "root".to_string(),
            version: "1.0.0".to_string(),
            deps: vec![("child".to_string(), "=1.0.0".to_string())],
            artifact_url: "https://registry.invalid/root".to_string(),
            sha256: "aabb".to_string(),
            extra_markers: Vec::new(),
        },
        ResolvedEntry {
            name: "child".to_string(),
            version: "1.0.0".to_string(),
            deps: Vec::new(),
            artifact_url: "https://registry.invalid/child".to_string(),
            sha256: "ccdd".to_string(),
            extra_markers: Vec::new(),
        },
    ];
    let (chosen, by_name) = index_entries(&entries).unwrap();
    let direct = HashSet::from(["root@1.0.0".to_string()]);
    let runtime = HashSet::from(["root@1.0.0".to_string(), "child@1.0.0".to_string()]);

    let output = project_packages(
        EcosystemTag::Rust,
        "crates://test",
        &entries,
        &chosen,
        &by_name,
        &direct,
        &runtime,
    )
    .unwrap();

    let root = output
        .graph
        .packages
        .iter()
        .find(|p| p.id.name_str() == "root")
        .unwrap();
    assert!(root.direct);
    assert!(!root.dev);
    assert_eq!(root.deps[0].name_str(), "child");
    let locked = output
        .lock_packages
        .iter()
        .find(|p| p.name == "root")
        .unwrap();
    assert_eq!(locked.dependencies, vec!["child@1.0.0"]);
    assert_eq!(locked.registry.as_deref(), Some("crates://test"));
}
