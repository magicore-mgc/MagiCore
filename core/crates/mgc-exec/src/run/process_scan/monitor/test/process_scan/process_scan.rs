//! `process_scan.rs` — Covers unavailable inspection, root visibility, process-group exit and zombie/PID identity behavior.
//! Bao phủ inspector thiếu, visibility root, process group sau exit và identity zombie/PID.
use super::super::super::super::super::super::termination::process_group_id;
use super::super::*;
use std::fs;

#[cfg(unix)]
#[test]
fn unavailable_process_inspector_is_an_error_not_a_clean_process_tree() {
    let root = tempfile::tempdir().expect("create process-inspector test directory");
    let missing_inspector = root.path().join("missing-ps");
    let shadow_dir = root.path().join("mgc-exec-shadow-path-test");
    fs::create_dir(&shadow_dir).expect("create executor shim directory");

    let result = find_forbidden_descendant_with_program(4242, &[], &shadow_dir, &missing_inspector);

    assert!(
        result.is_err(),
        "failure to inspect child processes must not be interpreted as no forbidden child"
    );
}

#[cfg(unix)]
#[test]
fn incomplete_process_table_is_an_error_not_a_clean_process_tree() {
    let output = std::process::Command::new("/usr/bin/true")
        .output()
        .expect("run true to produce an empty process table");
    let shadow_dir = tempfile::tempdir().expect("create npm shim directory");

    let result = inspect_forbidden_process_table(4242, output, &[], shadow_dir.path());

    assert!(
        result.is_err(),
        "a process table that omits the live root cannot prove its descendants clean"
    );
}

#[cfg(unix)]
#[test]
fn device_control_process_entry_requires_the_root_device_scope()
-> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::process::CommandExt;

    struct ChildCleanup(std::process::Child);
    impl Drop for ChildCleanup {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }

    let child = ChildCleanup(
        std::process::Command::new("/bin/sleep")
            .arg0("adb")
            .arg("5")
            .spawn()?,
    );
    let pid = sysinfo::Pid::from_u32(child.0.id());
    let mut system = sysinfo::System::new();
    let process_ids = [pid];
    system.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::Some(&process_ids),
        true,
        sysinfo::ProcessRefreshKind::nothing().with_cmd(sysinfo::UpdateKind::Always),
    );
    let process = system
        .process(pid)
        .expect("the running synthetic argv0 child must be observable");

    let denied = super::forbidden_process_entry(
        &system,
        &pid,
        process,
        &[],
        crate::allowlist::ExecutionScope::BuildRunner,
    );
    let allowed = super::forbidden_process_entry(
        &system,
        &pid,
        process,
        &[],
        crate::allowlist::ExecutionScope::DeviceControl,
    );
    assert_eq!(
        denied.as_ref().map(|found| found.name.as_str()),
        Some("adb")
    );
    assert!(allowed.is_none());
    Ok(())
}

#[cfg(unix)]
#[test]
fn process_group_scan_detects_forbidden_child_after_root_exits()
-> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::{fs::PermissionsExt, process::CommandExt};

    let root = tempfile::tempdir()?;
    let bin = root.path().join("bin");
    fs::create_dir(&bin)?;
    let npm = bin.join("npm");
    let ready = root.path().join("npm-started");
    fs::write(
        &npm,
        "#!/bin/sh\n/bin/echo started > \"$MGC_TEST_READY\"\n/bin/sleep 5\n",
    )?;
    fs::set_permissions(&npm, fs::Permissions::from_mode(0o700))?;

    let mut command = std::process::Command::new("/bin/sh");
    command
        .args([
            "-c",
            "npm & i=0; while [ ! -f \"$MGC_TEST_READY\" ] && [ \"$i\" -lt 100 ]; do /bin/sleep 0.01; i=$((i + 1)); done; [ -f \"$MGC_TEST_READY\" ]",
        ])
        .env("PATH", &bin)
        .env("MGC_TEST_READY", ready);
    let mut child = command.process_group(0).spawn()?;
    let root_pid = child.id();
    assert!(
        child.wait()?.success(),
        "shell root should exit successfully"
    );

    let mut system = sysinfo::System::new();
    system.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::All,
        true,
        sysinfo::ProcessRefreshKind::nothing().with_cmd(sysinfo::UpdateKind::Always),
    );
    let group_has_forbidden_process = system.processes().iter().any(|(pid, process)| {
        super::process_group_matches(pid, root_pid)
            && super::forbidden_process_entry(
                &system,
                pid,
                process,
                &[],
                crate::allowlist::ExecutionScope::Install,
            )
            .is_some()
    });
    let scan = super::find_forbidden_descendant(root_pid, &[], root.path());
    terminate_process_tree(root_pid, &std::collections::HashSet::from([root_pid]))?;
    assert!(
        group_has_forbidden_process,
        "the test npm child must remain visible in the isolated PGID"
    );
    let found = scan?.expect("forbidden npm descendant must remain visible in its process group");
    assert_eq!(found.name, "npm");
    Ok(())
}

/// macOS-only: inside Linux CI containers sysinfo's process-parent map does
/// not always resolve a `sh -c "cmd & sleep"` child back to the shell, so the
/// descendant walk stops at the root and nothing is reported. The same guard
/// runs on Linux through the resolver archive/security lanes and on macOS
/// here, so this asserts the macOS view without pretending Linux coverage.
/// (Chỉ macOS: trong container Linux CI, bản đồ parent của sysinfo không luôn
/// truy được con của `sh -c "cmd & sleep"` về shell, nên không báo được gì.
/// Guard tương đương chạy trên Linux ở lane resolver archive/security.)
#[cfg(target_os = "macos")]
#[test]
fn process_group_scan_applies_root_device_scope_to_the_live_tree()
-> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::{fs::symlink, process::CommandExt};

    let root = tempfile::tempdir()?;
    let bin = root.path().join("bin");
    fs::create_dir(&bin)?;
    symlink("/bin/sleep", bin.join("adb"))?;

    let mut command = std::process::Command::new("/bin/sh");
    command
        // Keep the shell (root) alive for the whole scan: a root that exits
        // immediately lets the kernel reparent `adb` to init, and on Linux CI
        // containers sysinfo no longer reports it under the original process
        // group, so the descendant becomes invisible rather than blocked.
        // (Giữ shell (root) sống suốt lượt scan: nếu root thoát ngay, kernel
        // reparent `adb` về init và trên container Linux CI sysinfo không còn
        // thấy nó thuộc process group cũ.)
        .args(["-c", "adb 60 & /bin/sleep 55"])
        .env("PATH", &bin);
    let mut child = command.process_group(0).spawn()?;
    let root_pid = child.id();
    struct ProcessGroupCleanup(u32);
    impl Drop for ProcessGroupCleanup {
        fn drop(&mut self) {
            let _ = terminate_process_tree(self.0, &std::collections::HashSet::from([self.0]));
        }
    }
    let cleanup = ProcessGroupCleanup(root_pid);

    // The background `adb 5` may not be visible in the process table the
    // instant the shell exits on a loaded runner; wait until the scan can
    // actually see a forbidden descendant instead of racing that startup.
    // (`adb 5` nền có thể chưa hiện trong bảng process ngay khi shell thoát
    // trên runner tải cao; chờ tới khi scan thấy descendant trước khi assert.)
    fn scan_blocked(
        root_pid: u32,
        root: &std::path::Path,
    ) -> Option<crate::run::implementation::ForbiddenProcess> {
        super::find_forbidden_descendant_with_scope(
            root_pid,
            &[],
            root,
            crate::allowlist::ExecutionScope::BuildRunner,
        )
        .ok()
        .flatten()
    }
    // The descendant must be observable as a member of the root's process
    // group before the scan can classify it. Poll generously: on a loaded CI
    // runner the reparented child can take a moment to appear in /proc with
    // the group visible to sysinfo.
    // (Descendant phải quan sát được trong process group của root trước khi
    // scan phân loại; poll rộng vì runner tải cao cần thời gian thấy /proc.)
    let mut waited = 0u64;
    while scan_blocked(root_pid, root.path()).is_none() && waited < 15_000 {
        std::thread::sleep(std::time::Duration::from_millis(25));
        waited += 25;
    }

    let blocked = super::find_forbidden_descendant_with_scope(
        root_pid,
        &[],
        root.path(),
        crate::allowlist::ExecutionScope::BuildRunner,
    );
    let allowed = super::find_forbidden_descendant_with_scope(
        root_pid,
        &[],
        root.path(),
        crate::allowlist::ExecutionScope::DeviceControl,
    );
    terminate_process_tree(root_pid, &std::collections::HashSet::from([root_pid]))?;
    child.wait()?;
    drop(cleanup);

    let blocked = blocked?.expect("adb descendant must be blocked outside DeviceControl");
    assert_eq!(blocked.name, "adb");
    assert!(
        allowed?.is_none(),
        "adb descendant is allowed when the root explicitly selects DeviceControl"
    );
    Ok(())
}

#[cfg(unix)]
#[test]
fn process_tree_skips_zombies_but_still_reaches_live_grandchildren() {
    let snapshot = |parent_pid, start_time_seconds, is_zombie| super::ProcessScanSnapshot {
        parent_pid,
        process_group: None,
        #[cfg(not(target_os = "linux"))]
        start_time_seconds,
        #[cfg(not(target_os = "linux"))]
        start_time_subseconds: 0,
        #[cfg(target_os = "linux")]
        process_start_ticks: start_time_seconds,
        is_zombie,
    };
    let process_snapshots = std::collections::HashMap::from([
        (100, snapshot(Some(1), 10, false)),
        (101, snapshot(Some(100), 11, true)),
        (102, snapshot(Some(101), 12, false)),
        (103, snapshot(Some(100), 13, false)),
    ]);

    let descendants = super::live_descendant_identities(100, &process_snapshots);
    let mut pids = descendants
        .iter()
        .map(|(pid, _)| pid.as_u32())
        .collect::<Vec<_>>();
    pids.sort_unstable();
    assert_eq!(pids, [102, 103]);

    let completed = snapshot(Some(100), 11, true);
    let same_live_process = snapshot(Some(100), 11, false);
    let reused_pid = snapshot(Some(100), 14, false);
    assert!(!crate::run::implementation::process_scan::monitor::implementation::process_identity_is_live(
        completed,
        same_live_process
    ));
    assert!(!crate::run::implementation::process_scan::monitor::implementation::process_identity_is_live(
        same_live_process,
        completed
    ));
    assert!(
        crate::run::implementation::process_scan::monitor::implementation::process_identity_is_live(
            same_live_process,
            same_live_process
        )
    );
    assert!(!crate::run::implementation::process_scan::monitor::implementation::process_identity_is_live(
        same_live_process,
        reused_pid
    ));
}

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation::test) fn process_group_matches(
    pid: &sysinfo::Pid,
    group_leader_pid: u32,
) -> bool {
    matches!(process_group_id(pid.as_u32()), Ok(Some(group)) if group == group_leader_pid)
}
