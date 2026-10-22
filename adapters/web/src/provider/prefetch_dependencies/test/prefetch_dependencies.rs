//! Cached dependency-prefetch branch / nhánh prefetch dependency từ cache.

use crate::provider::NpmDependencyProvider;
use mgc_resolver::ResolvedDep;
use mgc_types::{PackageId, PackageName, Version};

#[tokio::test]
async fn returns_cached_edges_without_registry_fetch() {
    let provider = NpmDependencyProvider::new("https://registry.invalid", None, None);
    let package = PackageName::new("react").unwrap();
    let id = PackageId::new(package.clone(), Version::parse("18.3.0").unwrap());
    let dependency = ResolvedDep {
        package: PackageName::new("scheduler").unwrap(),
        spec: "^0.23.0".into(),
        optional: false,
        peer: false,
    };
    let expected = vec![dependency];
    provider
        .registry_cache
        .insert_deps(NpmDependencyProvider::version_key(&id), expected.clone());
    let result = provider
        .prefetch_dependencies_impl(std::slice::from_ref(&id))
        .await
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].0, id);
    assert_eq!(result[0].1.len(), expected.len());
    for (actual, expected) in result[0].1.iter().zip(&expected) {
        assert_eq!(actual.package, expected.package);
        assert_eq!(actual.spec, expected.spec);
        assert_eq!(actual.optional, expected.optional);
        assert_eq!(actual.peer, expected.peer);
    }
}
