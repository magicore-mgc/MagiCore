//! Publish lifecycle policy tests.
//! Kiểm thử policy lifecycle khi publish.
use super::*;

#[test]
fn publish_hooks_require_explicit_opt_in_by_default() {
    let package = json!({"scripts": {"prepare": "node build.js"}});
    let error = publish_lifecycle_decision(&package, false, false)
        .expect_err("publish hooks must be blocked without explicit approval");
    assert!(error.to_string().contains("--allow-scripts"));
}

#[test]
fn publish_hooks_run_after_explicit_opt_in() {
    let package = json!({"scripts": {"prepublishOnly": "node build.js"}});
    assert!(
        publish_lifecycle_decision(&package, false, true)
            .expect("explicit opt-in should allow supported publish hooks")
    );
}

#[test]
fn ignore_scripts_skips_publish_hooks() {
    let package = json!({"scripts": {"prepare": "node build.js"}});
    assert!(
        !publish_lifecycle_decision(&package, true, false)
            .expect("ignore-scripts should skip lifecycle execution")
    );
}

#[test]
fn packages_without_publish_hooks_need_no_opt_in() {
    let package = json!({"scripts": {"test": "node test.js"}});
    assert!(
        !publish_lifecycle_decision(&package, false, false)
            .expect("unrelated scripts should not gate publishing")
    );
}

#[test]
fn malformed_publish_lifecycle_metadata_fails_closed() {
    let package = json!({"scripts": {"prepare": false}});
    assert!(publish_lifecycle_decision(&package, false, true).is_err());
}
