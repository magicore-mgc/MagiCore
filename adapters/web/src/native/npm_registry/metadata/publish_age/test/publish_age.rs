//! Existing publish-age predicate tests moved to the nearest owner.
//! Chuyển test predicate tuổi phát hành hiện hữu về owner gần nhất.

use super::super::check_publish_age;
use crate::native::npm_registry::PackageMetadata;

#[test]
fn test_check_publish_age_blocks_new_package() {
    let mut meta = PackageMetadata {
        name: "evil-pkg".to_string(),
        description: None,
        versions: Default::default(),
        dist_tags: Default::default(),
        time: Default::default(),
    };
    let published_at = (chrono::Utc::now() - chrono::Duration::hours(1)).to_rfc3339();
    meta.time.insert("1.0.0".to_string(), published_at);

    let result = check_publish_age(&meta, "1.0.0", 86400);
    assert!(result.is_err(), "should block packages published < 24h ago");
    assert!(result.unwrap_err().contains("quarantine"));

    let result = check_publish_age(&meta, "1.0.0", 1800);
    assert!(result.is_ok());
}

#[test]
fn test_check_publish_age_allows_old_package() {
    let mut meta = PackageMetadata {
        name: "safe-pkg".to_string(),
        description: None,
        versions: Default::default(),
        dist_tags: Default::default(),
        time: Default::default(),
    };
    let published_at = (chrono::Utc::now() - chrono::Duration::hours(48)).to_rfc3339();
    meta.time.insert("2.0.0".to_string(), published_at);

    let result = check_publish_age(&meta, "2.0.0", 86400);
    assert!(result.is_ok());
}

#[test]
fn check_publish_age_reports_unparseable_timestamp_as_unknown() {
    let mut meta = PackageMetadata {
        name: "private-pkg".to_string(),
        description: None,
        versions: Default::default(),
        dist_tags: Default::default(),
        time: Default::default(),
    };
    meta.time
        .insert("1.0.0".to_string(), "not-a-timestamp".to_string());

    assert_eq!(check_publish_age(&meta, "1.0.0", 86_400), Ok(false));
}

#[test]
fn check_publish_age_reports_missing_timestamp_as_unknown() {
    let meta = PackageMetadata {
        name: "private-pkg".to_string(),
        description: None,
        versions: Default::default(),
        dist_tags: Default::default(),
        time: Default::default(),
    };

    assert_eq!(check_publish_age(&meta, "1.0.0", 86_400), Ok(false));
}

#[test]
fn check_publish_age_rejects_unrepresentable_duration_without_panicking() {
    let meta = PackageMetadata {
        name: "safe-pkg".to_string(),
        description: None,
        versions: Default::default(),
        dist_tags: Default::default(),
        time: Default::default(),
    };

    assert!(check_publish_age(&meta, "1.0.0", i64::MAX).is_err());
}

#[test]
fn check_publish_age_formats_subhour_threshold_without_truncating_to_zero() {
    let mut meta = PackageMetadata {
        name: "new-pkg".to_string(),
        description: None,
        versions: Default::default(),
        dist_tags: Default::default(),
        time: Default::default(),
    };
    meta.time.insert(
        "1.0.0".to_string(),
        (chrono::Utc::now() - chrono::Duration::minutes(10)).to_rfc3339(),
    );

    let error = check_publish_age(&meta, "1.0.0", 1_800)
        .expect_err("young release must remain inside a 30-minute quarantine");

    assert!(error.contains("< 0.5h quarantine"));
}
