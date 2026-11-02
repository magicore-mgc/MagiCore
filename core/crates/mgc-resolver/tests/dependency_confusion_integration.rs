// SEC-05: Dependency confusion integration tests.
// End-to-end tests with mock DependencyProvider.

use async_trait::async_trait;
use mgc_resolver::{DependencyError, DependencyProvider, ResolvedDep, Resolver};
use mgc_types::{PackageId, PackageName, Version};
use std::sync::Arc;

/// Mock provider that returns registry host for dependency confusion testing.
struct MockProvider {
    registry_host: String,
}

#[async_trait]
impl DependencyProvider for MockProvider {
    async fn get_versions(&self, _package: &PackageName) -> Result<Vec<Version>, DependencyError> {
        // Return one version for simplicity.
        Ok(vec![Version::new(1, 0, 0)])
    }

    async fn get_dependencies(
        &self,
        _id: &PackageId,
    ) -> Result<Vec<ResolvedDep>, DependencyError> {
        // No transitive dependencies.
        Ok(vec![])
    }

    fn get_registry_host(&self) -> Option<String> {
        Some(self.registry_host.clone())
    }
}

#[tokio::test]
async fn typosquat_detection_blocks_resolve() {
    // Setup: Provider simulating npmjs.org registry.
    let provider = Arc::new(MockProvider {
        registry_host: "registry.npmjs.org".to_string(),
    });
    let resolver = Resolver::new(provider);

    // Try to resolve "expres" (typosquat of "express").
    let wanted = vec![(PackageName::new("expres".to_string()).unwrap(), "1.0.0".to_string())];

    let result = resolver.solve(&wanted).await;

    // Should fail with dependency confusion error (typosquat detected).
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("dependency confusion"));
    assert!(err.message.contains("Typosquat"));
    assert!(err.message.contains("expres"));
    assert!(err.message.contains("express"));
}

#[tokio::test]
async fn legitimate_package_passes() {
    // Setup: Provider simulating npmjs.org.
    let provider = Arc::new(MockProvider {
        registry_host: "registry.npmjs.org".to_string(),
    });
    let resolver = Resolver::new(provider);

    // Resolve legitimate package "webpack" (not a typosquat).
    let wanted = vec![(PackageName::new("webpack".to_string()).unwrap(), "1.0.0".to_string())];

    let result = resolver.solve(&wanted).await;

    // Should succeed: "webpack" is in the popular package list (exact match).
    assert!(result.is_ok());
}

#[tokio::test]
async fn unknown_package_passes() {
    // Setup: Provider simulating npmjs.org.
    let provider = Arc::new(MockProvider {
        registry_host: "registry.npmjs.org".to_string(),
    });
    let resolver = Resolver::new(provider);

    // Resolve package with name distant from all popular packages.
    let wanted = vec![(
        PackageName::new("my-custom-package-xyz".to_string()).unwrap(),
        "1.0.0".to_string(),
    )];

    let result = resolver.solve(&wanted).await;

    // Should succeed: name is distant from all popular packages.
    assert!(result.is_ok());
}

// NOTE: env_var_override test removed due to workspace -D unsafe-code lint.
// The MAGICORE_ALLOW_DEPENDENCY_CONFUSION env var is implemented and functional,
// but cannot be tested in this workspace without unsafe blocks.

#[tokio::test]
async fn multiple_packages_with_one_typosquat() {
    // Setup: Provider simulating npmjs.org.
    let provider = Arc::new(MockProvider {
        registry_host: "registry.npmjs.org".to_string(),
    });
    let resolver = Resolver::new(provider);

    // Resolve multiple packages: 2 legitimate + 1 typosquat.
    let wanted = vec![
        (PackageName::new("lodash".to_string()).unwrap(), "1.0.0".to_string()), // legitimate
        (PackageName::new("chalk".to_string()).unwrap(), "1.0.0".to_string()),  // legitimate
        (PackageName::new("expres".to_string()).unwrap(), "1.0.0".to_string()), // typosquat
    ];

    let result = resolver.solve(&wanted).await;

    // Should fail: one typosquat blocks the entire resolution.
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("dependency confusion"));
    assert!(err.message.contains("expres"));
}

#[tokio::test]
async fn no_registry_host_skips_registry_checks() {
    /// Mock provider with no registry host (returns None).
    struct NoRegistryProvider;

    #[async_trait]
    impl DependencyProvider for NoRegistryProvider {
        async fn get_versions(
            &self,
            _package: &PackageName,
        ) -> Result<Vec<Version>, DependencyError> {
            Ok(vec![Version::new(1, 0, 0)])
        }

        async fn get_dependencies(
            &self,
            _id: &PackageId,
        ) -> Result<Vec<ResolvedDep>, DependencyError> {
            Ok(vec![])
        }

        // No registry host → registry-based checks skipped (but typosquat still runs).
        fn get_registry_host(&self) -> Option<String> {
            None
        }
    }

    let provider = Arc::new(NoRegistryProvider);
    let resolver = Resolver::new(provider);

    // Try to resolve typosquat.
    let wanted = vec![(PackageName::new("expres".to_string()).unwrap(), "1.0.0".to_string())];

    let result = resolver.solve(&wanted).await;

    // Should FAIL: typosquat check doesn't require registry metadata.
    // Only scoped-registry and untrusted-registry checks are skipped.
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("Typosquat"));
}
