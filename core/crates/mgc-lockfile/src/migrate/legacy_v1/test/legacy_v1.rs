//! Checks v1 lockfile TOML parsing and version validation.
//! Kiểm tra parse TOML lockfile v1 và xác thực version.

use super::super::parse_lockfile_v1;

#[test]
fn parses_legacy_package_records_and_defaults_dependencies() {
    let text = "version = \"1\"\n[[package]]\nname = \"sample\"\nversion = \"1.0.0\"\nresolved = \"https://example.invalid/sample.tgz\"\n";
    let lock = parse_lockfile_v1(text).expect("test operation must succeed");
    assert_eq!(lock.packages.len(), 1);
    assert!(lock.packages[0].dependencies.is_empty());
}
