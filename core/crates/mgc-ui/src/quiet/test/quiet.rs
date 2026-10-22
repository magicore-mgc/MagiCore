//! Quiet-state tests — kiểm thử trạng thái quiet.
//! A single owner test gate avoids races with other quiet-state tests.
//! Cổng test riêng tránh tranh chấp giữa các kiểm thử state quiet.

use super::{is_quiet, set_quiet, test_lock};

#[test]
fn setter_and_getter_share_the_same_atomic_state() {
    let _guard = test_lock();
    set_quiet(false);
    assert!(!is_quiet());
    set_quiet(true);
    assert!(is_quiet());
    set_quiet(false);
    assert!(!is_quiet());
}
