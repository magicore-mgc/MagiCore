//! Project and persisted release-age policy tests.
//! Test policy tuổi phát hành của project và store.

use super::super::super as versions;

#[test]
#[cfg(feature = "web")]
fn scaffold_store_policy_read_failure_is_not_treated_as_no_policy() {
    let root = tempfile::tempdir().unwrap();
    let store_dir = mgc_web_adapter::lockfile::project_cache_dir(root.path());
    std::fs::create_dir_all(&store_dir).unwrap();
    let database_path = store_dir.join("store.db");
    std::fs::write(&database_path, b"not a sqlite database").unwrap();

    assert!(versions::store_release_policy(root.path()).is_err());

    std::fs::remove_file(&database_path).unwrap();
    std::fs::create_dir(&database_path).unwrap();
    let error = versions::store_release_policy(root.path()).unwrap_err();
    assert!(error.to_string().contains("must be a regular file"));
}

#[test]
#[cfg(feature = "web")]
fn scaffold_age_policy_uses_project_web_override_and_missing_time_escape() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("mgc.toml"),
        "[security]\nweb = 48\nallow_missing_time = true\n",
    )
    .unwrap();

    assert_eq!(
        versions::scaffold_age_policy(root.path()).unwrap(),
        Some(versions::ScaffoldAgePolicy {
            cutoff_hours: 48,
            allow_missing_time: true,
        })
    );
}

#[cfg(feature = "web")]
#[test]
fn untrusted_override_warning_names_the_disabled_scaffold_gate() {
    assert!(super::UNTRUSTED_OVERRIDE_WARNING.contains("MAGICORE_ALLOW_UNTRUSTED"));
    assert!(super::UNTRUSTED_OVERRIDE_WARNING.contains("release-age checks are disabled"));
}

#[cfg(not(feature = "web"))]
#[test]
fn non_web_core_build_does_not_require_the_optional_web_adapter() {
    let root = tempfile::tempdir().unwrap();
    assert!(
        versions::scaffold_age_policy(root.path())
            .unwrap()
            .is_none()
    );
}
