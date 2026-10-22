//! Resolver trait API dispatch / dispatch API trait resolver.

use crate::provider::NpmDependencyProvider;
use mgc_resolver::DependencyProvider;
use mgc_types::{PackageName, Version};

#[tokio::test]
async fn trait_version_lookup_uses_the_provider_cache() {
    let provider = NpmDependencyProvider::new("https://registry.invalid", None, None);
    let package = PackageName::new("react").unwrap();
    let expected = vec![Version::parse("18.3.0").unwrap()];
    provider.insert_versions_for(&package, expected.clone());
    assert_eq!(
        DependencyProvider::get_versions(&provider, &package)
            .await
            .unwrap(),
        expected
    );
}
