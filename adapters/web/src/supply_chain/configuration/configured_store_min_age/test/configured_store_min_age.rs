//! Project/store precedence and fail-closed tests for Web release age.
//! Test precedence project/store và fail-closed cho tuổi phát hành Web.

use super::super::{configured_store_min_age_at, read_store_min_age_secs};
use mgc_store::{Database, Layout};
use std::path::Path;

fn write_store_policy(root: &Path, seconds: u64) {
    let layout = Layout::new(root.join(".magicore").join("cache").join("web"));
    std::fs::create_dir_all(layout.root()).expect("create store directory");
    let database = Database::open(&layout.db_path()).expect("create store database");
    database
        .upsert_release_policy("web", seconds)
        .expect("write release policy");
}

#[test]
fn project_age_uses_hours_and_precedes_store_seconds() {
    let project = tempfile::tempdir().expect("create project directory");
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"web-age-owner-test\"\n[security]\nweb = 24\n",
    )
    .expect("write project security configuration");
    write_store_policy(project.path(), 30);

    let policy = configured_store_min_age_at(project.path()).expect("load age policy");
    assert_eq!(policy.min_age_secs, Some(24 * 60 * 60));
    assert!(!policy.allow_missing_time);
}

#[test]
fn project_age_override_does_not_open_an_unused_store_database() {
    let project = tempfile::tempdir().expect("create project directory");
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"web-age-owner-test\"\n[security]\nweb = 24\n",
    )
    .expect("write project security configuration");
    let store_path = project
        .path()
        .join(".magicore")
        .join("cache")
        .join("web")
        .join("store.db");
    std::fs::create_dir_all(store_path.parent().expect("store parent"))
        .expect("create store parent");
    std::fs::write(&store_path, "not a sqlite database").expect("write malformed store");

    let policy = configured_store_min_age_at(project.path()).expect("project age wins");

    assert_eq!(policy.min_age_secs, Some(24 * 60 * 60));
}

#[test]
fn project_missing_timestamp_escape_is_carried_with_store_policy() {
    let project = tempfile::tempdir().expect("create project directory");
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"web-age-owner-test\"\n[security]\nallow_missing_time = true\n",
    )
    .expect("write project security configuration");
    write_store_policy(project.path(), 90);

    let policy = configured_store_min_age_at(project.path()).expect("load age policy");
    assert_eq!(policy.min_age_secs, Some(90));
    assert!(policy.allow_missing_time);
}

#[test]
fn malformed_project_security_does_not_fall_back_to_store_policy() {
    let project = tempfile::tempdir().expect("create project directory");
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"web-age-owner-test\"\n[security\nweb = 24\n",
    )
    .expect("write malformed project security configuration");
    write_store_policy(project.path(), 90);

    let error = configured_store_min_age_at(project.path())
        .expect_err("malformed project policy must fail closed");

    assert!(error.to_string().contains("mgc.toml"));
}

#[test]
fn absent_database_returns_no_store_policy_without_creating_it() {
    let project = tempfile::tempdir().expect("create project directory");
    let path = project
        .path()
        .join(".magicore")
        .join("cache")
        .join("web")
        .join("store.db");

    assert_eq!(
        read_store_min_age_secs(&path).expect("read absent store"),
        None
    );
    assert!(!path.exists());
}

#[test]
fn malformed_existing_database_fails_closed() {
    let project = tempfile::tempdir().expect("create project directory");
    let path = project.path().join("store.db");
    std::fs::write(&path, "not a sqlite database").expect("write malformed database");

    let error = read_store_min_age_secs(&path).expect_err("malformed store must fail closed");

    assert!(error.to_string().contains("store.db"));
}

#[cfg(unix)]
#[test]
fn symlinked_database_is_rejected() {
    let project = tempfile::tempdir().expect("create project directory");
    let target = project.path().join("target.db");
    let link = project.path().join("store.db");
    let _database = Database::open(&target).expect("create target database");
    std::os::unix::fs::symlink(&target, &link).expect("create database symlink");

    let error = read_store_min_age_secs(&link).expect_err("database symlink must be rejected");

    assert!(error.to_string().contains("store.db"));
}

#[test]
fn project_age_rejects_values_outside_the_supported_seconds_range() {
    let path = std::path::Path::new("mgc.toml");

    let error = super::super::project_hours_to_seconds(u64::MAX, path)
        .expect_err("unrepresentable project age must fail closed");

    assert!(error.to_string().contains("outside the supported range"));
}
