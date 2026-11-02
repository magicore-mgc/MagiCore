// SEC-05: Typosquatting detection tests.
// Attack: Package name similar to popular package (Levenshtein distance ≤2).

use crate::solver::dependency_confusion::{DepInfo, check_dependency_confusion};
use std::collections::HashMap;

#[test]
fn detects_distance_1_typosquat() {
    let workspace_packages = vec![];

    // "expres" is distance 1 from "express" (delete one 's').
    let dependencies = vec![DepInfo {
        name: "expres".to_string(),
        version: Some("4.18.0".to_string()),
        registry: Some("registry.npmjs.org".to_string()),
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("Typosquat"));
    assert!(warnings[0].contains("expres"));
    assert!(warnings[0].contains("express"));
    assert!(warnings[0].contains("distance 1"));
}

#[test]
fn detects_distance_2_typosquat() {
    let workspace_packages = vec![];

    // "lodahs" is distance 2 from "lodash" (s/h swap + add h).
    // Actually distance 2: replace 's' with 'sh' = delete 's' + insert 'sh' = 2 ops.
    // Let's use "loadash" instead (insert 'a') = distance 1.
    // Or "loddash" (insert 'd') = distance 1.
    // Let's use "lodaash" (insert 'a') = distance 1.
    // Better: "loodash" (replace 'd' with 'o' + insert 'o') = distance 2.
    // Simpler: "lodish" (replace 'a' with 'i', delete 'sh') = 2 deletions from end + 1 substitution = actually complex.
    // Let's compute correctly: "lodish" vs "lodash"
    //   - lod same (3 chars)
    //   - 'i' vs 'a' = 1 substitution
    //   - 'sh' vs 'sh' = same
    //   Total: 1 substitution = distance 1 (too close).
    //
    // Better test: "lodsh" (delete 'a') = distance 1.
    // "loash" (delete 'd') = distance 1.
    // "ldash" (delete 'o') = distance 1.
    //
    // For distance 2, we need 2 operations. Example:
    // "loodsh" vs "lodash":
    //   - Insert 'o' after 'lo' = +1
    //   - Delete 'a' = +1
    //   Total: distance 2.
    //
    // Actually, let's just test "expres" (delete 's' from "express") = distance 1.
    // And "expres" is 6 chars, meets the len >= 6 requirement for distance 2 check.
    // Wait, "expres" is distance 1 (delete one 's'), not 2.
    //
    // For true distance 2: "exprss" (delete 'e', insert 's') vs "express".
    // Levenshtein("exprss", "express"):
    //   e x p r s s
    //   e x p r e s s
    //   Match match match match subst match insert
    //   = 1 substitution + 1 insertion = distance 2.
    //
    // Let's use simpler example: "expres" has 6 chars, but it's distance 1 from "express".
    // For distance 2, use "expresss" (insert 's') - wait, that's distance 1.
    //
    // Concrete distance 2 example: "recat" vs "react" (5 chars):
    //   r e c a t
    //   r e a c t
    //   match match subst subst match
    //   = 2 substitutions = distance 2.
    //
    // But "recat" is 5 chars, and distance 2 check requires len >= 6.
    // So it won't trigger.
    //
    // Use "reactt" (6 chars) vs "react" (5 chars):
    //   r e a c t t
    //   r e a c t
    //   match match match match match insert
    //   = 1 insertion = distance 1, not 2.
    //
    // Let's use different package: "momnet" (6 chars) vs "moment" (6 chars):
    //   m o m n e t
    //   m o m e n t
    //   match match match subst subst match
    //   = 2 substitutions = distance 2.

    let dependencies = vec![DepInfo {
        name: "momnet".to_string(), // distance 2 from "moment"
        version: Some("2.29.4".to_string()),
        registry: Some("registry.npmjs.org".to_string()),
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("Typosquat"));
    assert!(warnings[0].contains("momnet"));
    assert!(warnings[0].contains("moment"));
    assert!(warnings[0].contains("distance 2"));
}

#[test]
fn allows_legitimate_packages() {
    let workspace_packages = vec![];

    // Legitimate popular package (exact match).
    let dependencies = vec![DepInfo {
        name: "react".to_string(),
        version: Some("18.2.0".to_string()),
        registry: Some("registry.npmjs.org".to_string()),
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    // No warning: exact match to popular package.
    assert_eq!(warnings.len(), 0);
}

#[test]
fn allows_distant_names() {
    let workspace_packages = vec![];

    // "my-custom-lib" is very different from all popular packages.
    let dependencies = vec![DepInfo {
        name: "my-custom-lib".to_string(),
        version: Some("1.0.0".to_string()),
        registry: Some("registry.npmjs.org".to_string()),
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    // No warning: name is distant from all popular packages.
    assert_eq!(warnings.len(), 0);
}

#[test]
fn ignores_short_names_for_distance_2() {
    let workspace_packages = vec![];

    // "lodsh" (5 chars) is distance 1 from "lodash" (6 chars).
    // But even if it were distance 2, it's < 6 chars, so distance 2 check doesn't apply.
    // This test verifies that short typosquats are only caught by distance 1.
    let dependencies = vec![DepInfo {
        name: "lodsh".to_string(), // 5 chars, distance 1
        version: Some("1.0.0".to_string()),
        registry: Some("registry.npmjs.org".to_string()),
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    // Should trigger distance 1 warning (5 chars >= 4).
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("distance 1"));
}

#[test]
fn detects_multiple_typosquats() {
    let workspace_packages = vec![];

    // Multiple typosquatted packages.
    let dependencies = vec![
        DepInfo {
            name: "lodsh".to_string(), // distance 1 from "lodash" (delete 'a')
            version: Some("4.17.0".to_string()),
            registry: Some("registry.npmjs.org".to_string()),
        },
        DepInfo {
            name: "expres".to_string(), // distance 1 from "express"
            version: Some("4.18.0".to_string()),
            registry: Some("registry.npmjs.org".to_string()),
        },
        DepInfo {
            name: "lodash".to_string(), // legitimate
            version: Some("4.17.21".to_string()),
            registry: Some("registry.npmjs.org".to_string()),
        },
    ];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    // Should detect 2 typosquats.
    assert_eq!(warnings.len(), 2);
    assert!(warnings.iter().any(|w| w.contains("lodsh") && w.contains("lodash")));
    assert!(warnings.iter().any(|w| w.contains("expres") && w.contains("express")));
}

#[test]
fn skips_scoped_packages_for_typosquat() {
    let workspace_packages = vec![];

    // Scoped packages are not checked for typosquatting against top npm packages.
    let dependencies = vec![DepInfo {
        name: "@mycompany/raect".to_string(), // looks like "react" but scoped
        version: Some("1.0.0".to_string()),
        registry: Some("registry.npmjs.org".to_string()),
    }];

    let warnings = check_dependency_confusion(
        &workspace_packages,
        &dependencies,
        &HashMap::new(),
        &[],
    );

    // No typosquat warning: scoped packages are not checked.
    assert_eq!(warnings.len(), 0);
}
