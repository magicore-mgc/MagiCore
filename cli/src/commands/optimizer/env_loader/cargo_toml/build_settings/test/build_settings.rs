//! Cargo `[build]` schema tests.
//! (Kiểm thử schema `[build]` của Cargo.)

use std::path::Path;

#[test]
fn build_settings_map_jobs_and_rustflags() {
    let path = Path::new("project/Cargo.toml");
    let value: toml::Value =
        toml::from_str("jobs = 4\nrustflags = [\"-C\", \"target-cpu=native\"]\n").unwrap();
    let settings = super::parse_build_settings(path, &value).unwrap();

    assert_eq!(settings.get("CARGO_BUILD_JOBS"), Some(&"4".to_string()));
    assert_eq!(
        settings.get("RUSTFLAGS"),
        Some(&"-C target-cpu=native".to_string())
    );
}

#[test]
fn build_settings_reject_zero_jobs() {
    let path = Path::new("project/Cargo.toml");
    let value: toml::Value = toml::from_str("jobs = 0\n").unwrap();

    assert!(super::parse_build_settings(path, &value).is_err());
}
