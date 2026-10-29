//! Switch tests: active version changes, previous keg survives, failures roll back.
//! Test switch: version active đổi, keg cũ sống, lỗi thì rollback.

use super::super::active::{ActiveState, read_active, write_active};
use super::super::error::KegError;
use super::super::layout::{ACTIVE_LINK, validate_version};
use super::super::switch::{rollback_active, switch_active};
use super::support::{Fixture, thaw_all};

fn two_versions(fixture: &Fixture) {
    fixture.install("libcxx", "1.0.0");
    fixture.install("libcxx", "2.0.0");
    write_active(&fixture.layout, "libcxx", &ActiveState::initial("1.0.0")).unwrap();
    switch_active(&fixture.store, "libcxx", "1.0.0").unwrap();
}

#[test]
fn switch_activates_another_version_and_keeps_the_old_keg() {
    let fixture = Fixture::new();
    two_versions(&fixture);
    let first = fixture
        .layout
        .keg_dir("libcxx", &validate_version("1.0.0").unwrap());

    let outcome = switch_active(&fixture.store, "libcxx", "2.0.0").unwrap();

    assert_eq!(outcome.active, "2.0.0");
    assert_eq!(outcome.previous.as_deref(), Some("1.0.0"));
    assert_eq!(outcome.links, 4, "two versions x (active + prefix) links");
    assert!(first.is_dir(), "the previous keg must survive the switch");
    let current = fixture.layout.opt_dir("libcxx").join(ACTIVE_LINK);
    assert_eq!(
        std::fs::read(current.join("README")).unwrap(),
        b"README 2.0.0"
    );
}

#[test]
fn switch_to_a_missing_version_fails_and_changes_nothing() {
    let fixture = Fixture::new();
    two_versions(&fixture);
    let state_before = read_active(&fixture.layout, "libcxx").unwrap();
    let current = fixture.layout.opt_dir("libcxx").join(ACTIVE_LINK);
    let link_before = std::fs::read_link(&current).unwrap();

    let error = switch_active(&fixture.store, "libcxx", "9.9.9").unwrap_err();

    assert!(matches!(error, KegError::KegNotFound { .. }), "{error:?}");
    assert_eq!(
        read_active(&fixture.layout, "libcxx").unwrap(),
        state_before
    );
    assert_eq!(std::fs::read_link(&current).unwrap(), link_before);
}

#[test]
fn switch_to_a_tampered_keg_is_refused_and_keeps_the_old_active() {
    let fixture = Fixture::new();
    two_versions(&fixture);
    let target = fixture.layout.cellar_root().join("libcxx");
    let keg = fixture
        .layout
        .keg_dir("libcxx", &validate_version("2.0.0").unwrap());
    let file = keg.join("README");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&keg, std::fs::Permissions::from_mode(0o755)).unwrap();
        std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o755)).unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o644)).unwrap();
        std::fs::write(&file, b"swapped").unwrap();
    }

    let error = switch_active(&fixture.store, "libcxx", "2.0.0").unwrap_err();
    assert!(matches!(error, KegError::Tampered { .. }), "{error:?}");

    let state = read_active(&fixture.layout, "libcxx").unwrap().unwrap();
    assert_eq!(state.version, "1.0.0", "old active version stays active");
    let current = fixture.layout.opt_dir("libcxx").join(ACTIVE_LINK);
    assert_eq!(
        std::fs::read(current.join("README")).unwrap(),
        b"README 1.0.0"
    );
    thaw_all(&fixture, "libcxx");
}

#[test]
fn failed_switch_restores_a_prefix_link_it_already_moved() {
    let fixture = Fixture::new();
    two_versions(&fixture);
    let active_link = fixture.layout.opt_dir("libcxx").join(ACTIVE_LINK);
    let backup = fixture.layout.opt_dir("libcxx").join("backup-link");
    std::fs::rename(&active_link, &backup).unwrap();
    // A real directory at the active-link path makes the final link step fail.
    // (Thư mục thật tại path link active làm bước link cuối lỗi.)
    std::fs::create_dir(&active_link).unwrap();
    let prefix_v2 = fixture
        .layout
        .version_prefix_dir("libcxx", &validate_version("2.0.0").unwrap());
    let prefix_before = std::fs::read_link(&prefix_v2).unwrap();
    let state_before = read_active(&fixture.layout, "libcxx").unwrap();

    let error = switch_active(&fixture.store, "libcxx", "2.0.0").unwrap_err();
    assert!(
        matches!(error, KegError::Io { .. } | KegError::UnsafePath { .. }),
        "{error:?}"
    );

    // The version prefix touched before the failure must be restored.
    // (Prefix version bị đụng trước lỗi phải được khôi phục.)
    assert_eq!(std::fs::read_link(&prefix_v2).unwrap(), prefix_before);
    // The pointer is only the commit marker; it must not have advanced.
    // (Con trỏ chỉ là mốc commit; không được tiến lên.)
    assert_eq!(
        read_active(&fixture.layout, "libcxx").unwrap(),
        state_before
    );
    assert!(active_link.is_dir());
    std::fs::remove_dir(&active_link).unwrap();
    std::fs::rename(&backup, &active_link).unwrap();
    thaw_all(&fixture, "libcxx");
}

#[test]
fn rollback_reactivates_the_previous_version() {
    let fixture = Fixture::new();
    two_versions(&fixture);
    switch_active(&fixture.store, "libcxx", "2.0.0").unwrap();

    let outcome = rollback_active(&fixture.store, "libcxx").unwrap();

    assert_eq!(outcome.active, "1.0.0");
    assert_eq!(outcome.previous.as_deref(), Some("2.0.0"));
    let current = fixture.layout.opt_dir("libcxx").join(ACTIVE_LINK);
    assert_eq!(
        std::fs::read(current.join("README")).unwrap(),
        b"README 1.0.0"
    );
}

#[test]
fn rollback_without_a_previous_version_is_a_loud_error() {
    let fixture = Fixture::new();
    fixture.install("libcxx", "1.0.0");
    switch_active(&fixture.store, "libcxx", "1.0.0").unwrap();

    let error = rollback_active(&fixture.store, "libcxx").unwrap_err();
    assert!(matches!(error, KegError::NoActiveVersion(_)), "{error:?}");
}

#[test]
fn switching_to_the_active_version_reports_no_previous() {
    let fixture = Fixture::new();
    two_versions(&fixture);
    let outcome = switch_active(&fixture.store, "libcxx", "1.0.0").unwrap();
    assert_eq!(outcome.active, "1.0.0");
    assert_eq!(outcome.previous, None);
}

#[test]
fn switch_repairs_a_deleted_version_prefix_link() {
    let fixture = Fixture::new();
    two_versions(&fixture);
    let prefix = fixture
        .layout
        .version_prefix_dir("libcxx", &validate_version("2.0.0").unwrap());
    std::fs::remove_file(&prefix).unwrap();

    switch_active(&fixture.store, "libcxx", "1.0.0").unwrap();

    assert!(
        super::super::prefix::link_target(&prefix)
            .unwrap()
            .is_some(),
        "switch must repair the missing version prefix"
    );
}
