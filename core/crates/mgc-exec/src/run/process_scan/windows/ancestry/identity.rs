//! Track Windows process instances and fail closed on ambiguous PID ancestry.
//! Theo dõi instance process Windows và từ chối khi ancestry PID mơ hồ.

#[cfg(test)]
use super::super::WindowsProcessSnapshot;
#[cfg(test)]
use anyhow::Result;

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WindowsProcessIdentity {
    pub(super) creation_time: u64,
    pub(super) image_name: String,
}

#[cfg(test)]
impl WindowsProcessSnapshot {
    pub(super) fn identity(&self, creation_time: u64) -> WindowsProcessIdentity {
        WindowsProcessIdentity {
            creation_time,
            image_name: self.image_name.clone(),
        }
    }
}

#[cfg(test)]
#[derive(Default)]
pub(in crate::run::implementation::process_scan::windows) struct WindowsProcessAncestry {
    pub(super) known_descendant_instances: std::collections::HashMap<u32, WindowsProcessIdentity>,
    pub(super) retired_pids: std::collections::HashSet<u32>,
}

#[cfg(test)]
pub(super) fn current_windows_process_creation_time(
    process: &WindowsProcessSnapshot,
    query_creation_time: &mut impl FnMut(u32) -> Result<Option<u64>>,
    cached_identities: &mut std::collections::HashMap<u32, Option<u64>>,
) -> Result<Option<u64>> {
    if let Some(creation_time) = process.creation_time {
        return Ok(Some(creation_time));
    }
    if let Some(creation_time) = cached_identities.get(&process.pid) {
        return Ok(*creation_time);
    }
    let creation_time = query_creation_time(process.pid)?;
    cached_identities.insert(process.pid, creation_time);
    Ok(creation_time)
}
