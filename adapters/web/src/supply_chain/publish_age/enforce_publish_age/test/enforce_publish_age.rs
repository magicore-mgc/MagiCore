//! Resolved-package quarantine tests.
//! Test cách ly package đã resolve.

use super::super::enforce_publish_age;
use crate::native::npm_registry::PackageMetadata;
use crate::supply_chain::error::SupplyChainError;
use mgc_resolver::solver::Resolution;
use mgc_types::PackageId;
use std::collections::HashMap;
use std::sync::Arc;

fn resolution(package: &str) -> Resolution {
    let package_id = PackageId::parse(package).expect("parse package identity");
    Resolution {
        version: package_id.version().clone(),
        package_id,
        integrity: String::new(),
        deps: Vec::new(),
        dep_specs: Vec::new(),
        peer_deps: Vec::new(),
    }
}

fn metadata(name: &str, version: &str, age_hours: i64) -> Arc<PackageMetadata> {
    let mut time = HashMap::new();
    time.insert(
        version.to_owned(),
        (chrono::Utc::now() - chrono::Duration::hours(age_hours)).to_rfc3339(),
    );
    Arc::new(PackageMetadata {
        name: name.to_owned(),
        description: None,
        versions: HashMap::new(),
        dist_tags: HashMap::new(),
        time,
    })
}

#[test]
fn rejects_a_known_timestamp_inside_the_quarantine_window() {
    let resolution = resolution("new-package@1.0.0");
    let metadata = HashMap::from([(
        "new-package".to_owned(),
        metadata("new-package", "1.0.0", 1),
    )]);

    let error = enforce_publish_age(&[resolution], &metadata, 86_400, false)
        .expect_err("young release must be rejected");

    assert!(matches!(&error, SupplyChainError::PublishAge { .. }));
    assert!(error.to_string().contains("quarantine"));
}

#[test]
fn missing_time_escape_does_not_accept_a_known_young_release() {
    let resolution = resolution("young-package@1.0.0");
    let metadata = HashMap::from([(
        "young-package".to_owned(),
        metadata("young-package", "1.0.0", 1),
    )]);

    let error = enforce_publish_age(&[resolution], &metadata, 86_400, true)
        .expect_err("missing-time escape must not bypass the quarantine threshold");

    assert!(matches!(error, SupplyChainError::PublishAge { .. }));
}

#[test]
fn accepts_a_known_timestamp_outside_the_quarantine_window() {
    let resolution = resolution("old-package@1.0.0");
    let metadata = HashMap::from([(
        "old-package".to_owned(),
        metadata("old-package", "1.0.0", 48),
    )]);

    assert!(enforce_publish_age(&[resolution], &metadata, 86_400, false).is_ok());
}

#[test]
fn rejects_a_resolution_without_a_metadata_entry_when_age_guard_is_active() {
    let resolution = resolution("unindexed-package@1.0.0");

    let error = enforce_publish_age(&[resolution], &HashMap::new(), 86_400, false)
        .expect_err("missing metadata must fail closed");

    assert!(matches!(
        error,
        SupplyChainError::MissingPackageMetadata { .. }
    ));
}

#[test]
fn missing_time_escape_does_not_allow_absent_package_metadata() {
    let resolution = resolution("unindexed-package@1.0.0");

    let error = enforce_publish_age(&[resolution], &HashMap::new(), 86_400, true)
        .expect_err("missing-time escape cannot replace package metadata");

    assert!(matches!(
        error,
        SupplyChainError::MissingPackageMetadata { .. }
    ));
}

#[test]
fn rejects_missing_timestamp_without_explicit_project_escape() {
    let resolution = resolution("unstamped-package@1.0.0");
    let metadata = HashMap::from([(
        "unstamped-package".to_owned(),
        Arc::new(PackageMetadata {
            name: "unstamped-package".to_owned(),
            description: None,
            versions: HashMap::new(),
            dist_tags: HashMap::new(),
            time: HashMap::new(),
        }),
    )]);

    let error = enforce_publish_age(&[resolution], &metadata, 86_400, false)
        .expect_err("missing timestamp must fail closed");

    assert!(matches!(
        error,
        SupplyChainError::MissingPublishTimestamp { .. }
    ));
}

#[test]
fn allows_missing_timestamp_only_when_project_policy_opts_in() {
    let resolution = resolution("private-package@1.0.0");
    let metadata = HashMap::from([(
        "private-package".to_owned(),
        Arc::new(PackageMetadata {
            name: "private-package".to_owned(),
            description: None,
            versions: HashMap::new(),
            dist_tags: HashMap::new(),
            time: HashMap::new(),
        }),
    )]);

    assert!(enforce_publish_age(&[resolution], &metadata, 86_400, true).is_ok());
}

#[test]
fn malformed_timestamp_uses_the_same_explicit_escape_policy() {
    let resolution = resolution("private-package@1.0.0");
    let mut time = HashMap::new();
    time.insert("1.0.0".to_owned(), "not-a-timestamp".to_owned());
    let metadata = HashMap::from([(
        "private-package".to_owned(),
        Arc::new(PackageMetadata {
            name: "private-package".to_owned(),
            description: None,
            versions: HashMap::new(),
            dist_tags: HashMap::new(),
            time,
        }),
    )]);

    let strict_error =
        enforce_publish_age(std::slice::from_ref(&resolution), &metadata, 86_400, false)
            .expect_err("malformed timestamp must fail closed");
    assert!(matches!(
        strict_error,
        SupplyChainError::InvalidPublishTimestamp { .. }
    ));
    assert!(enforce_publish_age(&[resolution], &metadata, 86_400, true).is_ok());
}

#[test]
fn zero_threshold_disables_metadata_requirements() {
    let resolution = resolution("unindexed-package@1.0.0");

    assert!(enforce_publish_age(&[resolution], &HashMap::new(), 0, false).is_ok());
}

#[test]
fn unrepresentable_duration_is_rejected_without_panicking() {
    let resolution = resolution("old-package@1.0.0");

    let error = enforce_publish_age(&[resolution], &HashMap::new(), i64::MAX, false)
        .expect_err("unrepresentable chrono duration must fail closed");

    assert!(matches!(
        error,
        SupplyChainError::MinimumAgeSecondsOutOfRange { seconds: i64::MAX }
    ));
}
