//! Stable dependency-order and registry-input cache-key behavior.
//! Hành vi cache key ổn định theo thứ tự dependency và registry.

use super::super::manifest_resolution_cache_key;
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

fn manifest(name: &str, dependencies: &[(&str, &str)]) -> Manifest {
    let mut manifest = Manifest::new(name, Ecosystem::Web);
    for (package, range) in dependencies {
        manifest.add_dep(
            DependencySpec::new(
                PackageName::new(*package).unwrap(),
                VersionRange::parse(range).unwrap(),
            ),
            false,
            false,
            false,
        );
    }
    manifest
}

#[test]
fn key_ignores_dependency_order_app_name_and_registry_trailing_slash() {
    let left = manifest("demo-a", &[("react", "^18.2.0"), ("zod", "^3.22.4")]);
    let right = manifest("demo-b", &[("zod", "^3.22.4"), ("react", "^18.2.0")]);

    assert_eq!(
        manifest_resolution_cache_key(&left, "https://registry.npmjs.org"),
        manifest_resolution_cache_key(&right, "https://registry.npmjs.org/")
    );
}

#[test]
fn key_changes_when_registry_or_dependency_range_changes() {
    let original = manifest("demo", &[("react", "^18.2.0")]);
    let changed_range = manifest("demo", &[("react", "^19.0.0")]);

    let original_key = manifest_resolution_cache_key(&original, "https://registry.npmjs.org");
    assert_ne!(
        original_key,
        manifest_resolution_cache_key(&changed_range, "https://registry.npmjs.org")
    );
    assert_ne!(
        original_key,
        manifest_resolution_cache_key(&original, "https://registry.example.test")
    );
}
