use std::ffi::{OsStr, OsString};

use sysinfo::{Pid, Process, System};

/// Capture the process image name and argv excluding argv[0].
/// Lưu tên image và argv của process, bỏ argv[0].
pub(in crate::spawn_profile) fn process_observation(
    name: &OsStr,
    command: &[OsString],
) -> (String, Vec<String>) {
    let name = name.to_string_lossy().to_string();
    let args = command
        .iter()
        .skip(1)
        .map(|argument| argument.to_string_lossy().to_string())
        .collect();
    (name, args)
}

pub(in crate::spawn_profile) fn observe_process(
    system: &System,
    pid: Pid,
) -> Option<(String, Vec<String>)> {
    let process: &Process = system.process(pid)?;
    Some(process_observation(process.name(), process.cmd()))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
