//! Verify current publish-option defaults and serde behavior.
//! Kiểm tra default và serde hiện tại của publish options.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::PublishOptions;

#[test]
fn default_uses_current_field_values() {
    let options = PublishOptions::default();

    assert!(options.registry_url.is_empty());
    assert_eq!(options.tag, "latest");
    assert!(options.access.is_none());
    assert!(!options.dry_run);
    assert!(options.otp.is_none());
    assert!(!options.ignore_scripts);
    assert!(!options.force);
    assert_eq!(options.retries, 3);
}

#[test]
fn serde_applies_defaults_when_tag_and_retries_are_missing() {
    let json = r#"{
        "registry_url": "https://registry.invalid/",
        "access": null,
        "dry_run": false,
        "otp": null,
        "ignore_scripts": false,
        "force": false
    }"#;
    let options: PublishOptions = serde_json::from_str(json).unwrap();

    assert_eq!(options.tag, "latest");
    assert_eq!(options.retries, 3);
}
