//! Highest-compatible-version and digest mapping tests.
//! Test chọn version phù hợp cao nhất và ánh xạ lỗi digest.

use super::super::super::types::{PubPackage, PubSpec, PubVersion};
use super::super::select_entry;
use mgc_types::MgError;
use std::collections::HashMap;

fn package(version: &str, digest: Option<String>) -> PubVersion {
    PubVersion {
        version: version.to_string(),
        pubspec: Some(PubSpec {
            environment: None,
            dependencies: Some(HashMap::new()),
        }),
        archive_url: format!("https://pub.dev/packages/sample-{version}.tar.gz"),
        archive_sha256: digest,
    }
}

#[test]
fn select_entry_chooses_highest_matching_version_and_normalizes_digest() {
    let doc = PubPackage {
        versions: vec![
            package("1.2.0", Some("A".repeat(64))),
            package("1.9.0", Some("B".repeat(64))),
            package("2.0.0", Some("C".repeat(64))),
        ],
    };
    let entry =
        select_entry("sample", &doc, &["^1.0.0".to_string()]).expect("test operation must succeed");

    assert_eq!(entry.version, "1.9.0");
    assert_eq!(entry.sha256, "b".repeat(64));
}

#[test]
fn missing_archive_digest_remains_an_integrity_failure() {
    let doc = PubPackage {
        versions: vec![package("1.0.0", None)],
    };
    let error =
        select_entry("sample", &doc, &["any".to_string()]).expect_err("test operation must fail");

    assert!(matches!(error, MgError::Integrity(message) if message.contains("archive_sha256")));
}
