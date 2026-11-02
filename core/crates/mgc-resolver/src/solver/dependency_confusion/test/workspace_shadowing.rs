// SEC-05: Workspace package shadowing tests.
// Attack: External package has same name as workspace package.

use crate::solver::dependency_confusion::{DepInfo, check_dependency_confusion};
use std::collections::HashMap;

#[test]
fn detects_workspace_package_shadowing() {
    // Workspace has internal package "auth-service".
    let workspace_packages = vec!["auth-service".to_string()];

    // Dependency resolver found external "auth-service" from registry.
    let dependencies = vec![DepInfo {
        name: "auth-service".to_string(),
        version: Some("1.0.0".to_string()),
        registry: Some("registry.npmjs.org".to_string()),
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("Dependency confusion"));
    assert!(warnings[0].contains("auth-service"));
    assert!(warnings[0].contains("workspace package"));
}

#[test]
fn allows_workspace_protocol() {
    // Workspace package correctly using workspace protocol.
    let workspace_packages = vec!["internal-lib".to_string()];

    // Using workspace:* spec (no version = workspace protocol).
    let dependencies = vec![DepInfo {
        name: "internal-lib".to_string(),
        version: None, // workspace:* has no version
        registry: None,
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    // No warning: workspace protocol is correct usage.
    assert_eq!(warnings.len(), 0);
}

#[test]
fn allows_different_packages() {
    // Workspace has "app-core".
    let workspace_packages = vec!["app-core".to_string()];

    // External dependency "lodash" (different name).
    let dependencies = vec![DepInfo {
        name: "lodash".to_string(),
        version: Some("4.17.21".to_string()),
        registry: Some("registry.npmjs.org".to_string()),
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    // No warning: different package names.
    assert_eq!(warnings.len(), 0);
}

#[test]
fn detects_multiple_shadowed_packages() {
    // Workspace has 3 internal packages.
    let workspace_packages = vec![
        "auth".to_string(),
        "db-client".to_string(),
        "logger".to_string(),
    ];

    // Attacker published all 3 to npm.
    let dependencies = vec![
        DepInfo {
            name: "auth".to_string(),
            version: Some("1.0.0".to_string()),
            registry: Some("registry.npmjs.org".to_string()),
        },
        DepInfo {
            name: "db-client".to_string(),
            version: Some("2.0.0".to_string()),
            registry: Some("registry.npmjs.org".to_string()),
        },
        DepInfo {
            name: "logger".to_string(),
            version: Some("1.5.0".to_string()),
            registry: Some("registry.npmjs.org".to_string()),
        },
        DepInfo {
            name: "express".to_string(), // legitimate external dep
            version: Some("4.18.0".to_string()),
            registry: Some("registry.npmjs.org".to_string()),
        },
    ];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    // Should detect all 3 shadowed packages.
    assert_eq!(warnings.len(), 3);
    assert!(warnings.iter().any(|w| w.contains("auth")));
    assert!(warnings.iter().any(|w| w.contains("db-client")));
    assert!(warnings.iter().any(|w| w.contains("logger")));
}
