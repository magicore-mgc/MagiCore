//! Normalized Web scaffold policy type tests.
//! Test type policy scaffold Web đã chuẩn hóa.

use super::*;

#[test]
fn scaffold_age_policy_copy_preserves_both_policy_fields() {
    let policy = ScaffoldAgePolicy {
        cutoff_hours: 48,
        allow_missing_time: true,
    };

    let copied = policy;
    assert_eq!(copied, policy);
    assert_eq!(policy.cutoff_hours, 48);
    assert!(policy.allow_missing_time);
}

#[cfg(feature = "web")]
#[test]
fn scaffold_age_policy_maps_to_the_web_adapter_without_field_changes() {
    let policy = ScaffoldAgePolicy {
        cutoff_hours: 24,
        allow_missing_time: false,
    };
    let adapter_policy: mgc_web_adapter::provider::AgePolicy = policy.into();

    assert_eq!(adapter_policy.cutoff_hours, 24);
    assert!(!adapter_policy.allow_missing_time);
}
