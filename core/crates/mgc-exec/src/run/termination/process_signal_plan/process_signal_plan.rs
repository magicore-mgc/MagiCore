//! Select safe signal targets from observed/live groups and root state.
//! Chọn đích signal an toàn từ group đã thấy/đang sống và trạng thái root.

#[cfg(unix)]
use super::types::ProcessSignalPlan;

#[cfg(unix)]
pub(in crate::run::implementation::termination) fn process_signal_plan(
    root_pid: u32,
    observed_process_groups: &std::collections::HashSet<u32>,
    live_process_groups: &std::collections::HashSet<u32>,
    root_group_id: Option<u32>,
    root_reaped: bool,
) -> ProcessSignalPlan {
    let mut process_groups = observed_process_groups
        .intersection(live_process_groups)
        .copied()
        .collect::<std::collections::HashSet<_>>();
    if !root_reaped {
        for group in [Some(root_pid), root_group_id].into_iter().flatten() {
            if live_process_groups.contains(&group) {
                process_groups.insert(group);
            }
        }
    }
    ProcessSignalPlan {
        process_groups,
        signal_root_directly: !root_reaped
            && root_group_id.is_none_or(|group| !live_process_groups.contains(&group)),
    }
}

#[cfg(all(unix, test))]
#[path = "test/mod.rs"]
mod tests;
