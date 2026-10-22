use std::collections::HashMap;

use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

/// Refresh processes with argv enabled and snapshot their parent links.
/// Refresh process kèm argv và chụp các liên kết parent.
pub(in crate::spawn_profile) fn refresh_parent_map(system: &mut System) -> HashMap<Pid, Pid> {
    system.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_cmd(UpdateKind::Always),
    );
    system
        .processes()
        .iter()
        .filter_map(|(pid, process)| process.parent().map(|parent| (*pid, parent)))
        .collect()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
