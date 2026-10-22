//! Native Windows Job member inspection.
//! Kiểm tra native thành viên Windows Job.

#[cfg(windows)]
use super::super::super::super::super::process_scan::WindowsProcessSnapshot;
#[cfg(windows)]
use super::super::super::super::super::{
    CompatAuditContext, ShadowPath, audit_compat_process, is_npm_version_shell_probe,
    matched_invocation_shape_argv, process_basename, validate_monitored_child_command_line,
};
#[cfg(windows)]
use super::super::super::super::ProcessTreeGuard;
#[cfg(windows)]
use super::super::super::error::InspectionError;
#[cfg(windows)]
use super::super::member_capture::capture_windows_job_process_snapshots;
#[cfg(windows)]
use super::super::policy_metadata::inspect_windows_job_processes_with_policy_metadata;
#[cfg(windows)]
use super::super::{
    CapturedWindowsJobSnapshots, WindowsJobInspection, WindowsJobMemberCapture,
    WindowsJobMemberMetadata, WindowsProcessPolicyMetadata,
};
#[cfg(windows)]
use crate::allowlist::ExecutionScope;
#[cfg(windows)]
use anyhow::Result;
#[cfg(windows)]
use std::path::{Path, PathBuf};

#[cfg(windows)]
pub(in crate::run::implementation::windows_job) fn windows_job_descendants(
    guard: &ProcessTreeGuard,
    root_pid: u32,
    exempt: &[&str],
    shadow_path: &ShadowPath,
    compat_audit: Option<&CompatAuditContext<'_>>,
    execution_scope: ExecutionScope,
) -> std::result::Result<Vec<(u32, Option<String>)>, InspectionError> {
    let inspection = inspect_windows_job(
        guard,
        root_pid,
        exempt,
        Some(shadow_path),
        compat_audit,
        execution_scope,
    )
    .map_err(InspectionError::MemberInspection)?;
    if let Some((process_id, image_name)) = inspection.missing_command_line {
        validate_monitored_child_command_line(&image_name, &[], process_id).map_err(|source| {
            InspectionError::CommandLineValidation {
                process_id,
                image_name,
                source: anyhow::Error::new(source),
            }
        })?;
    }
    Ok(inspection.descendants)
}

/// Inspect process IDs owned by this private Job Object.
/// Chỉ kiểm tra các PID thuộc Job Object riêng của lệnh đang chạy.
#[cfg(windows)]
pub(in crate::run::implementation::windows_job::inspection::implementation) fn inspect_windows_job(
    guard: &ProcessTreeGuard,
    root_pid: u32,
    exempt: &[&str],
    shadow_path: Option<&ShadowPath>,
    compat_audit: Option<&CompatAuditContext<'_>>,
    execution_scope: ExecutionScope,
) -> Result<WindowsJobInspection> {
    use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

    let mut executable_paths = std::collections::HashMap::new();
    let mut go_policy_env = std::collections::HashMap::new();
    let process_ids = guard.active_process_ids()?;
    if process_ids.is_empty() {
        return Ok(WindowsJobInspection::default());
    }
    let mut system = System::new();
    let captured_snapshots = capture_windows_job_process_snapshots(
        root_pid,
        &process_ids,
        |process_id| {
            Ok(match guard.capture_job_member_process(process_id)? {
                Some(process) => WindowsJobMemberCapture::Captured(process),
                None => WindowsJobMemberCapture::Exited,
            })
        },
        |process_id, identity_guard| {
            let process_id = Pid::from_u32(process_id);
            let process_ids = [process_id];
            for _ in 0..2 {
                // Capture each member's command line immediately after verifying its Job
                // membership, before opening handles for other short-lived descendants.
                // Chụp command line ngay sau khi xác minh membership Job, trước khi mở handle
                // cho các tiến trình con ngắn hạn khác.
                system.refresh_processes_specifics(
                    ProcessesToUpdate::Some(&process_ids),
                    true,
                    ProcessRefreshKind::nothing()
                        .with_cmd(UpdateKind::Always)
                        .with_exe(UpdateKind::Always),
                );
                let is_go = system.process(process_id).is_some_and(|process| {
                    process_basename(&process.name().to_string_lossy()) == "go"
                });
                if is_go {
                    // Read this confirmed Job member's environment, never inherit policy from its parent.
                    // Chỉ môi trường của Go thuộc Job được xét; thiếu bằng chứng vẫn deny.
                    system.refresh_processes_specifics(
                        ProcessesToUpdate::Some(&process_ids),
                        true,
                        ProcessRefreshKind::nothing()
                            .with_cmd(UpdateKind::Always)
                            .with_exe(UpdateKind::Always)
                            .with_environ(UpdateKind::Always),
                    );
                }
                if let Some(process) = system.process(process_id) {
                    let image_name = process.name().to_string_lossy().into_owned();
                    if is_go && process_basename(&image_name) == "go" {
                        let observed = process
                            .environ()
                            .iter()
                            .filter_map(|entry| {
                                let (key, value) = entry.to_str()?.split_once('=')?;
                                key.eq_ignore_ascii_case("GOPROXY")
                                    .then(|| ("GOPROXY".to_owned(), value.to_owned()))
                            })
                            .collect::<Vec<_>>();
                        go_policy_env.insert(process_id.as_u32(), observed);
                    }
                    let executable_path = if image_name.eq_ignore_ascii_case("npm.exe") {
                        // Resolve the image through the validated Job handle, not only a process-table snapshot.
                        // Lấy image từ handle Job đã xác thực, không chỉ dựa vào snapshot process-table.
                        Some(identity_guard.full_image_path()?)
                    } else {
                        process.exe().map(Path::to_path_buf)
                    };
                    if let Some(executable) = executable_path {
                        executable_paths.insert(process_id.as_u32(), executable);
                    }
                    return Ok(WindowsJobMemberMetadata::Available(
                        WindowsProcessSnapshot {
                            pid: process_id.as_u32(),
                            #[cfg(test)]
                            parent_pid: None,
                            #[cfg(test)]
                            creation_time: None,
                            image_name,
                            command: process.cmd().to_vec(),
                        },
                    ));
                }
                if identity_guard.has_terminated()? {
                    // Live descendants remain separate members of this Job and are inspected in turn.
                    // Descendant còn sống vẫn là thành viên riêng của Job và sẽ được kiểm tra tiếp.
                    return Ok(WindowsJobMemberMetadata::Exited);
                }
            }
            if identity_guard.has_terminated()? {
                return Ok(WindowsJobMemberMetadata::Exited);
            }
            Ok(WindowsJobMemberMetadata::Unavailable)
        },
        || guard.active_process_ids(),
    )?;
    let captured_job_process_ids = captured_snapshots
        .members
        .iter()
        .map(|(process_id, _)| *process_id)
        .collect::<Vec<_>>();
    inspect_windows_job_processes_with_policy_metadata(
        root_pid,
        &captured_job_process_ids,
        &captured_snapshots.snapshots,
        exempt,
        shadow_path,
        &WindowsProcessPolicyMetadata {
            executable_paths: &executable_paths,
            go_policy_env: &go_policy_env,
        },
        compat_audit,
        execution_scope,
    )
}
