//! Alias metadata and version-cache identity / identity metadata và cache alias.

use crate::provider::NpmDependencyProvider;
use mgc_types::{PackageName, Version};

#[test]
fn records_alias_source_and_mirrors_cached_versions() {
    let provider = NpmDependencyProvider::new("https://registry.example", None, None);
    let alias = PackageName::new("react-alias").unwrap();
    let target = PackageName::new("react").unwrap();
    provider.record_alias_target(&alias, &target);
    assert_eq!(provider.source_package_name(&alias), target);
    provider.insert_versions_for(&alias, vec![Version::parse("18.3.0").unwrap()]);
    assert!(provider.cached_versions_for(&alias).is_some());
    assert!(
        provider
            .cached_versions_for(&PackageName::new("react").unwrap())
            .is_some()
    );
}
