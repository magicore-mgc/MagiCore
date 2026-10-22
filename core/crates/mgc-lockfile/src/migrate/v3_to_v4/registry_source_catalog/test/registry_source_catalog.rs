//! Checks registry deduplication and the explicit unknown-source lane.
//! Kiểm tra gộp registry trùng và nhánh nguồn `unknown` tường minh.

use super::super::super::{build_source_catalog, package_source_id};
use crate::Package;

#[test]
fn deduplicates_registry_urls_and_labels_unattributed_packages() {
    let mut first = package("first");
    first.registry = Some("https://registry.example".into());
    let mut duplicate = package("second");
    duplicate.registry = first.registry.clone();
    let unknown = package("unknown");
    let packages = [first.clone(), duplicate, unknown.clone()];

    let catalog = build_source_catalog(&packages);

    assert_eq!(catalog.sources.len(), 2);
    assert_eq!(catalog.source_ids.len(), 1);
    assert_eq!(catalog.warnings.len(), 1);
    assert_eq!(
        package_source_id(&first, &catalog),
        package_source_id(&packages[1], &catalog)
    );
    assert_eq!(package_source_id(&unknown, &catalog), "unknown");
}

fn package(name: &str) -> Package {
    Package::new(
        name.into(),
        "1.0.0".into(),
        "https://example.invalid/pkg.tgz".into(),
        String::new(),
    )
}
