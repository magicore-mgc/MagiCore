//! Cached version-prefetch branch / nhánh prefetch version từ cache.

use crate::provider::NpmDependencyProvider;
use mgc_types::{PackageName, Version};

#[tokio::test]
async fn returns_cached_versions_without_registry_fetch() {
    let provider = NpmDependencyProvider::new("https://registry.invalid", None, None);
    let package = PackageName::new("react").unwrap();
    let expected = vec![Version::parse("18.3.0").unwrap()];
    provider.insert_versions_for(&package, expected.clone());
    let result = provider
        .prefetch_versions_impl(std::slice::from_ref(&package))
        .await
        .unwrap();
    assert_eq!(result, vec![(package, expected)]);
}
