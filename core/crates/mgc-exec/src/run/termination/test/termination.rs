//! Keep the TERM/KILL grace bound documented as a local invariant.
//! Giữ invariant grace TERM/KILL ngay cạnh coordinator.

#[test]
fn term_to_kill_grace_remains_bounded_to_one_hundred_milliseconds() {
    assert_eq!(super::super::TERM_TO_KILL_GRACE_MS, 100);
}
