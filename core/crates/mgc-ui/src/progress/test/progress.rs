//! Progress behavior tests — kiểm thử hành vi progress.
//! Quiet-dependent checks share the crate test gate and reset the state.
//! Kiểm tra phụ thuộc quiet dùng cổng chung và khôi phục state.

use super::{
    ProgressManager, add_multi_bar, create_multi_progress, create_progress_bar, create_spinner,
};
use crate::quiet::{is_quiet, set_quiet, test_lock};

#[test]
fn progress_helpers_preserve_lengths_and_quiet_visibility() {
    let _guard = test_lock();
    set_quiet(false);

    let bar = create_progress_bar(12, "download");
    assert_eq!(bar.length(), Some(12));

    let multi = create_multi_progress();
    let multi_bar = add_multi_bar(&multi, 7, "package");
    assert_eq!(multi_bar.length(), Some(7));

    let _spinner = create_spinner("resolve");

    set_quiet(true);
    assert!(is_quiet());
    assert!(create_progress_bar(12, "download").is_hidden());
    assert!(add_multi_bar(&multi, 7, "package").is_hidden());
    assert!(create_spinner("resolve").is_hidden());

    set_quiet(false);
    assert!(!is_quiet());
}

#[test]
fn progress_manager_attaches_a_bar() {
    let _guard = test_lock();
    set_quiet(false);

    let manager = ProgressManager::default();
    let attached = manager.add(create_progress_bar(3, "task"));
    assert_eq!(attached.length(), Some(3));
}
