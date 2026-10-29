//! Install-then-activate tests: idempotency, rollback on activation failure.
//! Test cài rồi kích hoạt: idempotent, rollback khi kích hoạt lỗi.

use super::super::active::read_active;
use super::super::error::KegError;
use super::super::install::install_and_activate;
use super::super::layout::ACTIVE_LINK;
use super::super::lib_keg::lib_install;
use super::support::{Fixture, thaw_all, write_tree};

#[test]
fn install_creates_a_keg_and_activates_it() {
    let fixture = Fixture::new();
    let outcome = lib_install(&fixture.store, "libcxx", "1.0.0", |staging| {
        write_tree(staging, "libcxx", "1.0.0")
    })
    .unwrap();

    assert!(!outcome.reused);
    assert_eq!(outcome.activation.active, "1.0.0");
    assert_eq!(outcome.activation.previous, None);
    let current = fixture.layout.opt_dir("libcxx").join(ACTIVE_LINK);
    assert_eq!(
        std::fs::read(current.join("README")).unwrap(),
        b"README 1.0.0"
    );
}

#[test]
fn reinstalling_the_same_version_reuses_the_keg_without_rewriting() {
    let fixture = Fixture::new();
    fixture
        .store
        .create_keg("libcxx", "1.0.0", |staging| {
            write_tree(staging, "libcxx", "1.0.0")
        })
        .unwrap();
    let record_before = fixture.store.open_keg("libcxx", "1.0.0").unwrap();

    let outcome = install_and_activate(&fixture.store, "libcxx", "1.0.0", |_| {
        panic!("materialize must not run for an existing keg");
    })
    .unwrap();

    assert!(outcome.reused);
    assert_eq!(outcome.keg, record_before);
}

#[test]
fn installing_a_second_version_keeps_the_first_keg() {
    let fixture = Fixture::new();
    install_and_activate(&fixture.store, "libcxx", "1.0.0", |staging| {
        write_tree(staging, "libcxx", "1.0.0")
    })
    .unwrap();
    install_and_activate(&fixture.store, "libcxx", "2.0.0", |staging| {
        write_tree(staging, "libcxx", "2.0.0")
    })
    .unwrap();

    let versions: Vec<String> = fixture
        .store
        .list_kegs("libcxx")
        .unwrap()
        .into_iter()
        .map(|record| record.version)
        .collect();
    assert_eq!(versions, vec!["1.0.0", "2.0.0"]);
    let state = read_active(&fixture.layout, "libcxx").unwrap().unwrap();
    assert_eq!(state.version, "2.0.0");
    assert_eq!(state.previous.as_deref(), Some("1.0.0"));
}

#[test]
fn activation_failure_removes_the_new_keg_and_keeps_the_old_active() {
    let fixture = Fixture::new();
    install_and_activate(&fixture.store, "libcxx", "1.0.0", |staging| {
        write_tree(staging, "libcxx", "1.0.0")
    })
    .unwrap();

    // Make the active link path a real directory: the switch cannot replace it,
    // so activation fails after the new keg was created.
    // (Biến link active thành thư mục thật: switch không thay được, nên kích hoạt
    // lỗi sau khi keg mới đã được tạo.)
    let current = fixture.layout.opt_dir("libcxx").join(ACTIVE_LINK);
    std::fs::remove_file(&current).unwrap();
    std::fs::create_dir(&current).unwrap();
    std::fs::write(current.join("Intruder"), b"x").unwrap();

    let error = install_and_activate(&fixture.store, "libcxx", "2.0.0", |staging| {
        write_tree(staging, "libcxx", "2.0.0")
    })
    .expect_err("activation must fail closed");

    assert!(
        matches!(error, KegError::Io { .. } | KegError::UnsafePath { .. }),
        "{error:?}"
    );
    let versions: Vec<String> = fixture
        .store
        .list_kegs("libcxx")
        .unwrap()
        .into_iter()
        .map(|record| record.version)
        .collect();
    assert_eq!(
        versions,
        vec!["1.0.0"],
        "the failed keg must be rolled back"
    );
    let state = read_active(&fixture.layout, "libcxx").unwrap().unwrap();
    assert_eq!(
        state.version, "1.0.0",
        "previous active version stays active"
    );
    thaw_all(&fixture, "libcxx");
}
