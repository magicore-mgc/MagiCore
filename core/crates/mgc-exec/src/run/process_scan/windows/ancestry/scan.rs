//! Track Windows process instances and fail closed on ambiguous PID ancestry.
//! Theo dõi instance process Windows và từ chối khi ancestry PID mơ hồ.

#[cfg(test)]
use super::super::{WindowsProcessScan, WindowsProcessSnapshot};
#[cfg(test)]
use super::identity::{WindowsProcessAncestry, current_windows_process_creation_time};
#[cfg(test)]
use anyhow::{Result, bail};
#[cfg(test)]
pub(in crate::run::implementation) fn scan_windows_process_snapshot(
    root_pid: u32,
    ancestry: &mut WindowsProcessAncestry,
    processes: &[WindowsProcessSnapshot],
    exempt: &[&str],
    mut query_creation_time: impl FnMut(u32) -> Result<Option<u64>>,
) -> Result<WindowsProcessScan> {
    use std::collections::HashMap;

    let mut children_by_parent: HashMap<u32, Vec<&WindowsProcessSnapshot>> = HashMap::new();
    let mut processes_by_pid = HashMap::new();
    for process in processes {
        processes_by_pid.insert(process.pid, process);
        if let Some(parent_pid) = process.parent_pid {
            children_by_parent
                .entry(parent_pid)
                .or_default()
                .push(process);
        }
    }
    let Some(root_process) = processes_by_pid.get(&root_pid) else {
        bail!("process table did not contain monitored root PID {root_pid}");
    };
    let mut current_identities = HashMap::new();
    let Some(root_creation_time) = current_windows_process_creation_time(
        root_process,
        &mut query_creation_time,
        &mut current_identities,
    )?
    else {
        return Ok(WindowsProcessScan::MissingProcessIdentity { pid: root_pid });
    };
    let root_identity = root_process.identity(root_creation_time);
    if ancestry
        .known_descendant_instances
        .get(&root_pid)
        .is_some_and(|known| known != &root_identity)
    {
        bail!("monitored root PID {root_pid} changed process identity");
    }
    ancestry
        .known_descendant_instances
        .insert(root_pid, root_identity);
    ancestry.retired_pids.remove(&root_pid);

    // Match each remembered PID to its process instance before traversing its current children.
    // So danh tính instance cho PID đã lưu trước khi duyệt các tiến trình con hiện tại.
    let mut frontier = Vec::new();
    let mut verified_current_pids = std::collections::HashSet::new();
    for (&pid, known_identity) in &ancestry.known_descendant_instances {
        if pid == root_pid {
            frontier.push(pid);
            verified_current_pids.insert(pid);
            continue;
        }
        match processes_by_pid.get(&pid) {
            Some(current) if current.image_name != known_identity.image_name => {
                ancestry.retired_pids.insert(pid);
            }
            Some(current) => {
                let Some(current_creation_time) = current_windows_process_creation_time(
                    current,
                    &mut query_creation_time,
                    &mut current_identities,
                )?
                else {
                    return Ok(WindowsProcessScan::MissingProcessIdentity { pid });
                };
                if current.identity(current_creation_time) == *known_identity {
                    ancestry.retired_pids.remove(&pid);
                    frontier.push(pid);
                    verified_current_pids.insert(pid);
                } else {
                    ancestry.retired_pids.insert(pid);
                }
            }
            None if !ancestry.retired_pids.contains(&pid) => frontier.push(pid),
            None => {}
        }
    }

    let mut seen = std::collections::HashSet::new();
    while let Some(parent_pid) = frontier.pop() {
        if !seen.insert(parent_pid) {
            continue;
        }
        for process in children_by_parent.get(&parent_pid).into_iter().flatten() {
            if process.pid != root_pid {
                let Some(creation_time) = current_windows_process_creation_time(
                    process,
                    &mut query_creation_time,
                    &mut current_identities,
                )?
                else {
                    return Ok(WindowsProcessScan::MissingProcessIdentity { pid: process.pid });
                };
                ancestry
                    .known_descendant_instances
                    .insert(process.pid, process.identity(creation_time));
                ancestry.retired_pids.remove(&process.pid);
                verified_current_pids.insert(process.pid);
                frontier.push(process.pid);
            }
        }
    }

    // A child under a PID known to belong to a different instance has ambiguous ancestry; fail closed.
    // Con dưới PID đã xác định bị thay instance có nguồn gốc mơ hồ; từ chối an toàn.
    for &parent_pid in &ancestry.retired_pids {
        if let Some(child) = children_by_parent.get(&parent_pid).and_then(|children| {
            children
                .iter()
                .find(|child| !verified_current_pids.contains(&child.pid))
        }) {
            return Ok(WindowsProcessScan::UnverifiableAncestry {
                parent_pid,
                child_pid: child.pid,
            });
        }
    }

    super::command_scan::scan_verified_descendant_commands(
        root_pid,
        processes,
        &verified_current_pids,
        exempt,
    )
}
