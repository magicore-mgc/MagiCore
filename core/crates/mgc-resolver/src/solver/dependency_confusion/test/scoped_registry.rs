// SEC-05: Scoped registry mismatch tests.
// Attack: @scope/pkg should come from private registry but resolves from public.

use crate::solver::dependency_confusion::{DepInfo, check_dependency_confusion};
use std::collections::HashMap;

#[test]
fn detects_scoped_package_wrong_registry() {
    let workspace_packages = vec![];

    // Configuration: @mycompany/* should resolve from private registry.
    let mut scoped_registries = HashMap::new();
    scoped_registries.insert("@mycompany".to_string(), "npm.mycompany.internal".to_string());

    // Dependency resolved from public npm instead of private registry.
    let dependencies = vec![DepInfo {
        name: "@mycompany/db-client".to_string(),
        version: Some("1.0.0".to_string()),
        registry: Some("registry.npmjs.org".to_string()), // WRONG!
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &scoped_registries,
        &[],
    );

    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("Dependency confusion"));
    assert!(warnings[0].contains("@mycompany/db-client"));
    assert!(warnings[0].contains("npm.mycompany.internal"));
    assert!(warnings[0].contains("registry.npmjs.org"));
}

#[test]
fn allows_correct_scoped_registry() {
    let workspace_packages = vec![];

    // Configuration: @mycompany/* → private registry.
    let mut scoped_registries = HashMap::new();
    scoped_registries.insert("@mycompany".to_string(), "npm.mycompany.internal".to_string());

    // Correctly resolved from private registry.
    let dependencies = vec![DepInfo {
        name: "@mycompany/utils".to_string(),
        version: Some("2.0.0".to_string()),
        registry: Some("npm.mycompany.internal".to_string()), // CORRECT
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &scoped_registries,
        &[],
    );

    // No warning: correct registry.
    assert_eq!(warnings.len(), 0);
}

#[test]
fn allows_unscoped_packages() {
    let workspace_packages = vec![];

    // Scoped registry config exists but doesn't apply to unscoped packages.
    let mut scoped_registries = HashMap::new();
    scoped_registries.insert("@internal".to_string(), "npm.internal.local".to_string());

    // Unscoped package from public npm (not affected by scoped config).
    let dependencies = vec![DepInfo {
        name: "lodash".to_string(),
        version: Some("4.17.21".to_string()),
        registry: Some("registry.npmjs.org".to_string()),
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &scoped_registries,
        &[],
    );

    // No warning: scoped config doesn't apply to unscoped packages.
    assert_eq!(warnings.len(), 0);
}

#[test]
fn detects_multiple_scope_violations() {
    let workspace_packages = vec![];

    // Multiple scopes configured.
    let mut scoped_registries = HashMap::new();
    scoped_registries.insert("@company".to_string(), "npm.company.internal".to_string());
    scoped_registries.insert("@vendor".to_string(), "npm.vendor.com".to_string());

    // Multiple violations: both scopes resolved from wrong registries.
    let dependencies = vec![
        DepInfo {
            name: "@company/auth".to_string(),
            version: Some("1.0.0".to_string()),
            registry: Some("registry.npmjs.org".to_string()), // WRONG
        },
        DepInfo {
            name: "@vendor/sdk".to_string(),
            version: Some("2.0.0".to_string()),
            registry: Some("registry.npmjs.org".to_string()), // WRONG
        },
        DepInfo {
            name: "express".to_string(), // legitimate public package
            version: Some("4.18.0".to_string()),
            registry: Some("registry.npmjs.org".to_string()),
        },
    ];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &scoped_registries,
        &[],
    );

    // Should detect both scope violations.
    assert_eq!(warnings.len(), 2);
    assert!(warnings.iter().any(|w| w.contains("@company/auth")));
    assert!(warnings.iter().any(|w| w.contains("@vendor/sdk")));
}
