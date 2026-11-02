// SEC-05: Untrusted registry detection tests.
// Attack: Package resolved from registry not in trusted allowlist.

use crate::solver::dependency_confusion::{DepInfo, check_dependency_confusion};
use std::collections::HashMap;

#[test]
fn detects_untrusted_registry() {
    let workspace_packages = vec![];

    // Trusted registries allowlist.
    let trusted_registries = vec![
        "npm.company.internal".to_string(),
        "registry.npmjs.org".to_string(),
    ];

    // Package from untrusted registry.
    let dependencies = vec![DepInfo {
        name: "suspicious-pkg".to_string(),
        version: Some("1.0.0".to_string()),
        registry: Some("npm.attacker.evil".to_string()), // NOT IN ALLOWLIST
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &trusted_registries,
    );

    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("Dependency confusion"));
    assert!(warnings[0].contains("suspicious-pkg"));
    assert!(warnings[0].contains("npm.attacker.evil"));
    assert!(warnings[0].contains("not in trusted registries"));
}

#[test]
fn allows_trusted_registry() {
    let workspace_packages = vec![];

    // Trusted registries.
    let trusted_registries = vec![
        "registry.npmjs.org".to_string(),
        "npm.company.internal".to_string(),
    ];

    // Package from trusted registry.
    let dependencies = vec![DepInfo {
        name: "lodash".to_string(),
        version: Some("4.17.21".to_string()),
        registry: Some("registry.npmjs.org".to_string()), // IN ALLOWLIST
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &trusted_registries,
    );

    // No warning: registry is trusted.
    assert_eq!(warnings.len(), 0);
}

#[test]
fn skips_check_when_no_allowlist() {
    let workspace_packages = vec![];

    // No trusted registries configured (empty allowlist = trust all).
    let trusted_registries: Vec<String> = vec![];

    // Package from any registry.
    let dependencies = vec![DepInfo {
        name: "any-pkg".to_string(),
        version: Some("1.0.0".to_string()),
        registry: Some("npm.random-registry.com".to_string()),
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &trusted_registries,
    );

    // No warning: empty allowlist means no untrusted registry check.
    assert_eq!(warnings.len(), 0);
}

#[test]
fn detects_multiple_untrusted() {
    let workspace_packages = vec![];

    // Strict allowlist: only internal registry.
    let trusted_registries = vec!["npm.company.internal".to_string()];

    // Multiple packages from untrusted registries.
    let dependencies = vec![
        DepInfo {
            name: "pkg-a".to_string(),
            version: Some("1.0.0".to_string()),
            registry: Some("registry.npmjs.org".to_string()), // NOT TRUSTED
        },
        DepInfo {
            name: "pkg-b".to_string(),
            version: Some("2.0.0".to_string()),
            registry: Some("npm.attacker.com".to_string()), // NOT TRUSTED
        },
        DepInfo {
            name: "internal-lib".to_string(),
            version: Some("3.0.0".to_string()),
            registry: Some("npm.company.internal".to_string()), // TRUSTED
        },
    ];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &trusted_registries,
    );

    // Should detect 2 untrusted registries.
    assert_eq!(warnings.len(), 2);
    assert!(warnings.iter().any(|w| w.contains("pkg-a") && w.contains("registry.npmjs.org")));
    assert!(warnings.iter().any(|w| w.contains("pkg-b") && w.contains("npm.attacker.com")));
}
