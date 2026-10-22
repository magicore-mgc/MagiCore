//! Queue provenance tests.
//! Test nguồn gốc queue.

use super::super::QueuedDependency;
use mgc_types::PackageName;

#[test]
fn root_and_transitive_requests_keep_distinct_origins() {
    let name = PackageName::new("widget").expect("parse package name");
    let root = QueuedDependency::root(4, name.clone(), "^1.0.0".to_owned());
    let transitive = QueuedDependency::transitive(name, "^1.8.0".to_owned());

    assert_eq!(root.root_index, Some(4));
    assert_eq!(transitive.root_index, None);
    assert_eq!(root.name.as_str(), transitive.name.as_str());
}
