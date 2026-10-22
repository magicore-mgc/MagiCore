//! Provider-local age policy/cache behavior / policy tuổi và cache theo provider.

use mgc_types::{PackageName, Version};

/// P0/F6-RED: two provider instances hold INDEPENDENT policies (no
/// process-global first-wins) — multi-project processes arm per
/// operation.
/// Hai provider giữ policy ĐỘC LẬP (không global first-wins).
#[test]
fn test_age_policy_changes_invalidate_cached_version_decisions() {
    use crate::provider::{AgePolicy, NpmDependencyProvider};

    let provider = NpmDependencyProvider::new("https://example.invalid", None, None);
    let package = PackageName::new("react").unwrap();
    provider.insert_versions_for(&package, vec![Version::parse("18.2.0").unwrap()]);
    provider
        .optional_enqueue_cache
        .insert("stale-policy-decision".to_string(), true);
    assert!(provider.cached_versions_for(&package).is_some());

    provider.set_age_policy(Some(AgePolicy {
        cutoff_hours: 24,
        allow_missing_time: false,
    }));

    assert!(
        provider.cached_versions_for(&package).is_none(),
        "version lists selected under the previous policy must be discarded"
    );
    assert!(
        provider.optional_enqueue_cache.is_empty(),
        "optional dependency decisions can depend on the age-filtered version set"
    );
}

#[test]
fn test_age_policy_is_per_provider_instance() {
    use crate::provider::{AgePolicy, NpmDependencyProvider};
    let strict = NpmDependencyProvider::new("https://example.invalid", None, None);
    let plain = NpmDependencyProvider::new("https://example.invalid", None, None);
    strict.set_age_policy(Some(AgePolicy {
        cutoff_hours: 1_000_000,
        allow_missing_time: false,
    }));
    plain.set_age_policy(None);
    assert!(strict.age_gate_armed_for_test());
    assert!(!plain.age_gate_armed_for_test());
}
