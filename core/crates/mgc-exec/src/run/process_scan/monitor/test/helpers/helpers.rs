//! `helpers.rs` — process-table fixtures and read-only inspector harness.
//! Fixture process-table và harness inspector chỉ đọc.
use super::super::super::super::super::super::{
    forbidden_process_match, matched_invocation_shape, process_basename,
};
use super::super::*;
use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;

pub(in crate::run::implementation::process_scan::monitor::implementation::test) fn process_table_with_child(
    command: &str,
    command_line: &str,
) -> std::process::Output {
    process_table_with_raw_child(command.as_bytes(), command_line.as_bytes())
}

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation::test) fn process_table_with_raw_child(
    command: &[u8],
    command_line: &[u8],
) -> std::process::Output {
    use std::os::unix::process::ExitStatusExt;
    let mut output = std::process::Output {
        status: std::process::ExitStatus::from_raw(0),
        stdout: Vec::new(),
        stderr: Vec::new(),
    };
    output.stdout = b"4242 1 cargo /usr/bin/cargo\n4243 4242 ".to_vec();
    output.stdout.extend_from_slice(command);
    output.stdout.push(b' ');
    output.stdout.extend_from_slice(command_line);
    output.stdout.push(b'\n');
    output
}

#[cfg(test)]
pub(in crate::run::implementation::process_scan::monitor::implementation::test) fn find_forbidden_descendant_with_program(
    root_pid: u32,
    exempt: &[&str],
    shadow_dir: &Path,
    inspector: &Path,
) -> Result<Option<ForbiddenProcess>> {
    let output = Command::new(inspector)
        .args(["-axo", "pid=,ppid=,comm=,command="])
        .output()
        .with_context(|| {
            format!(
                "cannot start process-table inspector '{}'",
                inspector.display()
            )
        })?;
    if !output.status.success() {
        bail!(
            "process-table inspector '{}' exited with {}",
            inspector.display(),
            output.status
        );
    }

    inspect_forbidden_process_table(root_pid, output, exempt, shadow_dir)
}

#[cfg(unix)]
#[cfg(test)]
pub(in crate::run::implementation::process_scan::monitor::implementation::test) fn inspect_forbidden_process_table(
    root_pid: u32,
    output: std::process::Output,
    exempt: &[&str],
    shadow_dir: &Path,
) -> Result<Option<ForbiddenProcess>> {
    let mut processes = Vec::new();
    let mut root_seen = false;
    for mut line in output.stdout.split(|byte| *byte == b'\n') {
        if line.last() == Some(&b'\r') {
            line = &line[..line.len() - 1];
        }
        if line.iter().all(u8::is_ascii_whitespace) {
            continue;
        }
        let (pid_field, remainder) = next_process_table_field(line)
            .ok_or_else(|| anyhow::anyhow!("process-table row has no PID"))?;
        let pid = std::str::from_utf8(pid_field)
            .context("process-table row has a non-UTF-8 PID")?
            .parse::<u32>()
            .context("process-table row has an invalid PID")?;
        let (ppid_field, remainder) = next_process_table_field(remainder)
            .ok_or_else(|| anyhow::anyhow!("process-table row has no parent PID"))?;
        let ppid = std::str::from_utf8(ppid_field)
            .context("process-table row has a non-UTF-8 parent PID")?
            .parse::<u32>()
            .context("process-table row has an invalid parent PID")?;
        let (command, command_line) = next_process_table_field(remainder)
            .ok_or_else(|| anyhow::anyhow!("process-table row has no executable name"))?;
        root_seen |= pid == root_pid;
        processes.push((pid, ppid, command.to_vec(), command_line.to_vec()));
    }
    if !root_seen {
        bail!("process table did not contain monitored root PID {root_pid}");
    }

    let mut frontier = vec![root_pid];
    let mut seen = std::collections::HashSet::new();
    while let Some(parent) = frontier.pop() {
        if !seen.insert(parent) {
            continue;
        }

        for (pid, _ppid, command, command_line) in
            processes.iter().filter(|(_, ppid, _, _)| *ppid == parent)
        {
            let command_text = String::from_utf8_lossy(command);
            let command_line_text = String::from_utf8_lossy(command_line);
            if let Some((name, match_source, matched_token, matched_token_index)) =
                forbidden_process_match(&command_text, &command_line_text, exempt)
            {
                if is_allowed_shadow_npm_version_probe(command, command_line, shadow_dir, &name) {
                    frontier.push(*pid);
                    continue;
                }
                return Ok(Some(ForbiddenProcess {
                    pid: *pid,
                    name: name.to_owned(),
                    image_name: Some(process_basename(&command_text)),
                    match_source,
                    matched_token,
                    matched_token_index,
                    matched_operation: matched_invocation_shape(
                        &command_line_text,
                        matched_token_index,
                    ),
                }));
            }
            frontier.push(*pid);
        }
    }

    Ok(None)
}

#[cfg(unix)]
#[cfg(test)]
fn next_process_table_field(line: &[u8]) -> Option<(&[u8], &[u8])> {
    let start = line.iter().position(|byte| !byte.is_ascii_whitespace())?;
    let line = &line[start..];
    let end = line
        .iter()
        .position(u8::is_ascii_whitespace)
        .unwrap_or(line.len());
    if end == 0 {
        return None;
    }

    let remainder = &line[end..];
    let remainder_start = remainder
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(remainder.len());
    Some((&line[..end], &remainder[remainder_start..]))
}
