//! Verify resolved versions win and unusable pins remain explicit skips.
//! Kiểm tra resolved được ưu tiên và pin thiếu vẫn được báo skip.

#[test]
fn resolved_version_wins_and_missing_pin_is_reported() {
    let raw = r#"{"dependencies":{"net8.0":{"A":{"version":"1.0","resolved":"2.0"},"B":{"type":"Transitive"}}}}"#;
    let (pins, skipped) = super::read_packages_lock(raw).unwrap();

    assert_eq!(pins.len(), 1);
    assert_eq!(pins[0].name, "A");
    assert_eq!(pins[0].version, "2.0");
    assert_eq!(
        skipped,
        vec!["B: no version pin in packages.lock.json".to_string()]
    );
}
