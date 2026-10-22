//! Checks supported and rejected lockfile versions.
//! Kiểm tra version lockfile được hỗ trợ và bị từ chối.

use super::super::detect_lockfile_version;

#[test]
fn detects_legacy_versions_and_rejects_unknown_versions() {
    for version in ["1", "2", "3"] {
        let text = format!("version = \"{version}\"\n");
        assert_eq!(
            detect_lockfile_version(&text)
                .expect("test operation must succeed")
                .to_string(),
            version
        );
    }
    assert!(detect_lockfile_version("version = \"99\"\n").is_err());
}
