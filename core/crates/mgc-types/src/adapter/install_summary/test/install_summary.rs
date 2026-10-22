//! Install Summary nearest-owner contract tests.
//! Test contract gần owner install summary.

use crate::adapter::InstallSummary;

#[test]
fn install_summary_default() {
    let s = InstallSummary::default();
    assert!(s.added.is_empty());
    assert_eq!(s.bytes_from_cache, 0);
    assert_eq!(s.duration_ms, 0);
}

#[test]
fn cache_modes_keep_their_serde_variant_names() {
    assert_eq!(
        serde_json::to_string(&crate::adapter::InstallCacheMode::MgCStore).unwrap(),
        "\"MgCStore\""
    );
    assert_eq!(
        serde_json::to_string(&crate::adapter::InstallCacheMode::Delegated).unwrap(),
        "\"Delegated\""
    );
}
