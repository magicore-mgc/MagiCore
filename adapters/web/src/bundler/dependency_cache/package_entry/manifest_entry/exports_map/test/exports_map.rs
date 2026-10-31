//! `exports` subpath and condition selection tests.
//! Test chọn subpath và condition `exports`.

use serde_json::json;

use super::super::{exports_root_target, exports_subpath_target};

#[test]
fn selects_exact_extensionless_subpath_keys() {
    let manifest = json!({
        "exports": {
            ".": "./index.js",
            "./client": "./client.js",
            "./client.js": "./client.js"
        }
    });

    assert_eq!(
        exports_subpath_target(&manifest, "client").as_deref(),
        Some("./client.js")
    );
    assert_eq!(
        exports_root_target(&manifest).as_deref(),
        Some("./index.js")
    );
}

#[test]
fn selects_condition_and_nested_subpath_maps() {
    let manifest = json!({
        "exports": {
            "./client": {
                "browser": "./client.browser.js",
                "import": "./client.mjs",
                "require": "./client.cjs"
            }
        }
    });
    assert_eq!(
        exports_subpath_target(&manifest, "client").as_deref(),
        Some("./client.browser.js")
    );

    let nested =
        json!({ "exports": { "import": { "types": "./index.d.ts", "default": "./index.mjs" } } });
    assert_eq!(exports_root_target(&nested).as_deref(), Some("./index.mjs"));
}

#[test]
fn expands_the_most_specific_pattern_key() {
    let manifest = json!({
        "exports": {
            "./*": "./src/*.js",
            "./features/*": "./dist/features/*.js"
        }
    });

    assert_eq!(
        exports_subpath_target(&manifest, "features/map").as_deref(),
        Some("./dist/features/map.js")
    );
    assert_eq!(
        exports_subpath_target(&manifest, "other").as_deref(),
        Some("./src/other.js")
    );
}

#[test]
fn ignores_manifests_without_exports_or_subpath_keys() {
    for manifest in [
        json!({ "main": "./index.js" }),
        json!({ "exports": "./index.js" }),
        json!({ "exports": { "import": "./index.mjs" } }),
    ] {
        assert!(exports_subpath_target(&manifest, "client").is_none());
    }
}
