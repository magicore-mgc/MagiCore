//! Verify order-independent equality of selected groups.
//! Kiểm tra equality không phụ thuộc thứ tự group.

#[test]
fn process_signal_plan_compares_targets_and_direct_root_policy() {
    let plan = super::super::ProcessSignalPlan {
        process_groups: std::collections::HashSet::from([4242, 4300]),
        signal_root_directly: true,
    };
    let same = super::super::ProcessSignalPlan {
        process_groups: std::collections::HashSet::from([4300, 4242]),
        signal_root_directly: true,
    };

    assert_eq!(plan, same);
}
