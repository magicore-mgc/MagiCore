//! `observed_groups.rs` — runner-owned process group liveness.
//! Tính sống của process group thuộc runner.

use super::super::super::super::super::termination::{self, process_group_is_live};
use anyhow::Result;

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn refresh_live_observed_process_groups(
    observed_process_groups: &mut std::collections::HashSet<u32>,
) -> Result<()> {
    // Probe only runner-owned groups instead of calling getpgid for every host PID.
    // Chỉ dò các process group của lệnh đang chạy, không gọi getpgid cho mọi PID máy.
    let mut live_observed_process_groups = std::collections::HashSet::new();
    for process_group in observed_process_groups.iter().copied() {
        match process_group_is_live(process_group) {
            Ok(true) => {
                live_observed_process_groups.insert(process_group);
            }
            Ok(false) => {}
            Err(source) => {
                return Err(termination::TerminationError::process_group_probe(
                    process_group,
                    source,
                )
                .into());
            }
        }
    }
    *observed_process_groups = live_observed_process_groups;
    Ok(())
}
