//! Guard tests: undo on error, undo on drop without commit, commit drops undos.
//! Test guard: hoàn tác khi lỗi, khi drop không commit; commit bỏ hoàn tác.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::super::error::KegError;
use super::super::guard::KegGuard;
use super::super::layout::{ACTIVE_LINK, validate_version};
use super::support::{Fixture, thaw_all};

#[test]
fn error_result_rolls_back_registered_undos() {
    let fixture = Fixture::new();
    let calls = Arc::new(AtomicUsize::new(0));
    let mut guard = KegGuard::acquire(&fixture.layout, "libcxx").unwrap();
    for _ in 0..3 {
        let calls = Arc::clone(&calls);
        guard.push_undo(Box::new(move || {
            calls.fetch_add(1, Ordering::SeqCst);
        }));
    }

    let error = guard
        .run(|_| Err::<(), _>(KegError::Layout("boom".to_string())))
        .unwrap_err();

    assert!(matches!(error, KegError::Layout(_)));
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    assert_eq!(guard.pending_undos(), 0, "undos must not replay on drop");
}

#[test]
fn dropping_without_commit_rolls_back() {
    let fixture = Fixture::new();
    let calls = Arc::new(AtomicUsize::new(0));
    {
        let mut guard = KegGuard::acquire(&fixture.layout, "libcxx").unwrap();
        let calls_inner = Arc::clone(&calls);
        guard.push_undo(Box::new(move || {
            calls_inner.fetch_add(1, Ordering::SeqCst);
        }));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn commit_discards_undos() {
    let fixture = Fixture::new();
    let calls = Arc::new(AtomicUsize::new(0));
    let mut guard = KegGuard::acquire(&fixture.layout, "libcxx").unwrap();
    let calls_inner = Arc::clone(&calls);
    guard.push_undo(Box::new(move || {
        calls_inner.fetch_add(1, Ordering::SeqCst);
    }));
    guard.run(|_| Ok::<_, KegError>(())).unwrap();
    guard.commit();
    drop(guard);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[test]
fn undo_steps_run_in_reverse_order() {
    let fixture = Fixture::new();
    let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut guard = KegGuard::acquire(&fixture.layout, "libcxx").unwrap();
    for step in 1..=3 {
        let seen = Arc::clone(&seen);
        guard.push_undo(Box::new(move || {
            seen.lock().unwrap().push(step);
        }));
    }
    drop(guard);
    assert_eq!(*seen.lock().unwrap(), vec![3, 2, 1]);
}

#[test]
fn restore_entry_undo_recreates_and_removes_links() {
    let fixture = Fixture::new();
    let link = fixture.layout.opt_dir("libcxx").join(ACTIVE_LINK);
    std::fs::create_dir_all(link.parent().unwrap()).unwrap();

    // Missing -> the undo removes whatever the failed switch created.
    // (Chưa có -> hoàn tác xoá thứ switch lỗi đã tạo.)
    let state = super::super::guard::capture_entry(&link).unwrap();
    assert_eq!(state, super::super::guard::EntryState::Missing);
    super::super::prefix::create_symlink(std::path::Path::new("anything"), &link).unwrap();
    super::super::guard::restore_entry(link.clone(), state)();
    assert!(std::fs::symlink_metadata(&link).is_err());

    // Link -> restored to its original target.
    // (Link -> hoàn tác về đích ban đầu.)
    super::super::prefix::create_symlink(std::path::Path::new("original"), &link).unwrap();
    let state = super::super::guard::capture_entry(&link).unwrap();
    super::super::prefix::remove_link(&link).unwrap();
    super::super::prefix::create_symlink(std::path::Path::new("changed"), &link).unwrap();
    super::super::guard::restore_entry(link.clone(), state)();
    assert_eq!(
        super::super::prefix::link_target(&link).unwrap(),
        Some(std::path::PathBuf::from("original"))
    );
    let _ = validate_version("1.0.0");
    thaw_all(&fixture, "libcxx");
}

#[test]
fn guard_rejects_an_unsafe_library_name() {
    let fixture = Fixture::new();
    let error = KegGuard::acquire(&fixture.layout, "../escape").unwrap_err();
    assert!(matches!(error, KegError::UnsafePath { .. }), "{error:?}");
}
