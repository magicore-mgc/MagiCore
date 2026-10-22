//! Verify root/group targeting policy without signaling processes.
//! Kiểm tra policy chọn root/group mà không gửi signal process.

#[cfg(unix)]
#[test]
fn process_signal_plan_avoids_duplicate_signal_to_root_group() {
    let observed = std::collections::HashSet::from([4242, 4300]);
    let live = std::collections::HashSet::from([4242, 4300]);

    let plan = super::super::process_signal_plan(4242, &observed, &live, Some(4242), false);

    assert_eq!(plan.process_groups, live);
    assert!(!plan.signal_root_directly);
}

#[cfg(unix)]
#[test]
fn process_signal_plan_skips_reaped_root_and_recycled_groups() {
    let observed = std::collections::HashSet::from([4242, 4300, 4400]);
    let live = std::collections::HashSet::from([4300]);

    let plan = super::super::process_signal_plan(4242, &observed, &live, None, true);

    assert_eq!(plan.process_groups, live);
    assert!(!plan.signal_root_directly);
}

#[cfg(unix)]
#[test]
fn process_signal_plan_directly_targets_live_root_when_its_group_is_unverifiable() {
    let observed = std::collections::HashSet::from([4242, 4300]);
    let live = std::collections::HashSet::from([4300]);

    let plan = super::super::process_signal_plan(4242, &observed, &live, Some(4500), false);

    assert_eq!(plan.process_groups, live);
    assert!(plan.signal_root_directly);
}
