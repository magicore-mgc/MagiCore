//! Tests for process runner isolation — kiểm tra biên cô lập của process runner.

#[cfg(windows)]
#[test]
fn windows_bare_npm_probe_authenticates_image_and_rejects_mutations() {
    use std::ffi::OsString;
    let root = tempfile::tempdir().expect("create authenticated alias fixture");
    let executable_target = root.path().join("mgc.exe");
    let image = root.path().join("npm.exe");
    std::fs::copy(
        std::env::current_exe().expect("current executable"),
        &executable_target,
    )
    .expect("copy trusted executable fixture");
    if std::fs::hard_link(&executable_target, &image).is_err() {
        std::fs::copy(&executable_target, &image).expect("copy npm alias fixture");
    }
    let shadow = super::ShadowPath {
        dir: root.path().to_path_buf(),
    };
    let mut process = super::WindowsProcessSnapshot {
        pid: 101,
        parent_pid: Some(100),
        creation_time: Some(101),
        image_name: "npm.exe".to_owned(),
        command: vec![OsString::from("npm"), OsString::from("--version")],
    };
    assert!(super::is_shadow_windows_npm_version_probe(
        &process,
        &executable_target,
        &shadow
    ));
    assert!(super::is_shadow_windows_npm_version_probe(
        &process, &image, &shadow
    ));
    process.command[1] = OsString::from("install");
    assert!(!super::is_shadow_windows_npm_version_probe(
        &process,
        &executable_target,
        &shadow
    ));
    process.command[1] = OsString::from("--version");
    process.command.pop();
    assert!(super::is_shadow_windows_npm_version_probe(
        &process,
        &executable_target,
        &shadow
    ));
    process.command.clear();
    assert!(!super::is_shadow_windows_npm_version_probe(
        &process,
        &executable_target,
        &shadow
    ));
    std::fs::write(&executable_target, b"spoofed binary").expect("replace image fixture");
    assert!(!super::is_shadow_windows_npm_version_probe(
        &process,
        &executable_target,
        &shadow
    ));
}

#[test]
fn windows_probe_argv0_resolves_only_npm_aliases() {
    use std::ffi::OsStr;
    let root = tempfile::tempdir().expect("probe alias directory");
    let shadow = root.path().join("shadow");
    std::fs::create_dir(&shadow).expect("create private shadow directory");
    for alias in [OsStr::new("npm"), OsStr::new("NPM.EXE")] {
        assert_eq!(
            super::windows_npm_probe_argv0_path(alias, &shadow),
            Some(shadow.join("npm.exe"))
        );
    }
    let absolute_alias = root.path().join("npm.exe");
    assert_eq!(
        super::windows_npm_probe_argv0_path(absolute_alias.as_os_str(), &shadow),
        Some(absolute_alias)
    );
    assert!(super::windows_npm_probe_argv0_path(OsStr::new("other.exe"), &shadow).is_none());
}

#[test]
fn windows_tool_cwd_preserves_drive_unc_unicode_and_other_namespaces() {
    for (input, expected) in [
        (
            r"\\?\C:\Users\runneradmin\project",
            r"C:\Users\runneradmin\project",
        ),
        (r"\\?\UNC\server\share\project", r"\\server\share\project"),
        (r"\\?\C:\ứng dụng\项目", r"C:\ứng dụng\项目"),
        (r"C:\project", r"C:\project"),
        (
            r"\\?\Volume{identity}\project",
            r"\\?\Volume{identity}\project",
        ),
    ] {
        let input = input.encode_utf16().collect::<Vec<_>>();
        let expected = expected.encode_utf16().collect::<Vec<_>>();
        assert_eq!(super::windows_tool_path_units(&input), expected);
    }
    let mut units = r"\\?\C:\project\".encode_utf16().collect::<Vec<_>>();
    units.push(0xD800);
    assert_eq!(super::windows_tool_path_units(&units), units[4..]);
}

#[test]
fn windows_job_denial_preserves_safe_invocation_evidence() {
    use std::ffi::OsString;
    let process = super::WindowsProcessSnapshot {
        pid: 101,
        parent_pid: Some(100),
        creation_time: Some(101),
        image_name: "npm.exe".to_owned(),
        command: ["npm.exe", "install", "private-package-secret"]
            .into_iter()
            .map(OsString::from)
            .collect(),
    };
    let inspection =
        super::inspect_windows_job_processes_with_shadow(100, &[101], &[process], &[], None)
            .expect("inspect forbidden child");
    let super::WindowsProcessScan::Forbidden(found) =
        super::windows_job_scan_from_inspection(inspection)
    else {
        panic!("expected a forbidden process");
    };
    assert_eq!(found.image_name.as_deref(), Some("npm.exe"));
    assert!(found.matched_operation.contains("dependency-mutation"));
    assert!(!found.matched_operation.contains("private-package-secret"));
}

#[cfg(unix)]
use super::{
    ShadowPath, find_forbidden_descendant_with_program, inspect_forbidden_process_table,
    is_shadow_npm_version_probe, npm_probe_command_line_matches_shim_path,
};
use super::{
    ensure_process_inspection_available, is_path_env_key, process_inspection_unavailable_error,
    reject_external_dependency_resolution,
};
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[test]
fn path_environment_key_uses_host_case_semantics() {
    assert!(is_path_env_key("PATH"));
    #[cfg(windows)]
    assert!(is_path_env_key("Path"));
    #[cfg(not(windows))]
    assert!(!is_path_env_key("Path"));
}

#[test]
fn native_windows_path_lookup_prefers_pe_before_shims_across_path_entries()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let local_bin = root.path().join("local-bin");
    let system_bin = root.path().join("system-bin");
    std::fs::create_dir_all(&local_bin)?;
    std::fs::create_dir_all(&system_bin)?;
    std::fs::write(local_bin.join("tool.cmd"), "local shim")?;
    std::fs::write(local_bin.join("tool"), "extensionless script")?;
    std::fs::write(system_bin.join("tool.exe"), "PE executable")?;
    let search_path = std::env::join_paths([&local_bin, &system_bin])?;

    let resolved = super::find_windows_command("tool", Some(&search_path));

    assert_eq!(resolved, Some(system_bin.join("tool.exe").into_os_string()));
    Ok(())
}

#[test]
fn native_windows_path_lookup_prefers_cmd_over_bat_and_extensionless_script()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    std::fs::write(root.path().join("tool"), "extensionless script")?;
    std::fs::write(root.path().join("tool.bat"), "batch shim")?;
    std::fs::write(root.path().join("tool.cmd"), "command shim")?;
    let search_path = std::env::join_paths([root.path()])?;

    let resolved = super::find_windows_command("tool", Some(&search_path));

    assert_eq!(
        resolved,
        Some(root.path().join("tool.cmd").into_os_string())
    );
    Ok(())
}

#[test]
fn native_windows_path_lookup_does_not_search_a_different_path_entry()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let unselected_bin = root.path().join("unselected-bin");
    let selected_bin = root.path().join("selected-bin");
    std::fs::create_dir_all(&unselected_bin)?;
    std::fs::create_dir_all(&selected_bin)?;
    std::fs::write(unselected_bin.join("tool.exe"), "unselected executable")?;
    let search_path = std::env::join_paths([&selected_bin])?;

    let resolved = super::find_windows_command("tool", Some(&search_path));

    assert_eq!(resolved, None);
    Ok(())
}

#[test]
fn windows_batch_invocation_accepts_regular_paths_and_arguments() {
    // Spaces stay usable for common Windows installs and CLI values.
    // Vẫn cho phép khoảng trắng thường gặp trong đường dẫn và giá trị CLI.
    let args = vec!["--output".to_string(), "release bundle.zip".to_string()];
    assert!(
        super::validate_windows_batch_invocation(r"C:\Program Files\MagiCore\tool.cmd", &args)
            .is_ok()
    );
}

#[test]
fn windows_batch_invocation_rejects_cmd_expansion_and_operators() {
    // Every cmd.exe expansion/operator character must be rejected in either input.
    // Mọi ký tự expansion/toán tử của cmd.exe phải bị chặn ở cả hai đầu vào.
    for character in ['"', '%', '!', '&', '|', '<', '>', '^', '(', ')', '\n', '\r'] {
        let argument = format!("value{character}payload");
        assert!(
            super::validate_windows_batch_invocation(
                r"C:\tools\tool.cmd",
                std::slice::from_ref(&argument)
            )
            .is_err(),
            "argument containing {character:?} must be rejected"
        );

        let path = format!(r"C:\tools\bad{character}name.cmd");
        assert!(
            super::validate_windows_batch_invocation(&path, &[]).is_err(),
            "script path containing {character:?} must be rejected"
        );
    }
}

#[test]
#[cfg(unix)]
fn shadow_directory_creation_refuses_a_preexisting_symlink()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let target = root.path().join("attacker-owned");
    fs::create_dir(&target)?;
    let protected_file = target.join("npm");
    fs::write(&protected_file, "do not overwrite")?;
    let link = root.path().join("shadow-link");
    std::os::unix::fs::symlink(&target, &link)?;

    let result = ShadowPath::create_at(link, &[]);

    assert!(
        result.is_err(),
        "existing symlink must not be adopted as a shim directory"
    );
    assert_eq!(fs::read_to_string(protected_file)?, "do not overwrite");
    Ok(())
}

#[test]
fn dependency_guard_blocks_uncovered_package_manager_subcommands() {
    let cases = [
        ("cargo", vec!["package"]),
        ("cargo", vec!["publish"]),
        ("cargo", vec!["tree"]),
        ("cargo", vec!["info", "serde"]),
        ("cargo", vec!["generate-lockfile"]),
        ("cargo", vec!["upgrade"]),
        ("go", vec!["list", "-m", "all"]),
        ("dotnet", vec!["tool", "install", "example.tool"]),
        ("dotnet", vec!["workload", "install", "wasm-tools"]),
        ("dotnet", vec!["nuget", "push", "package.nupkg"]),
        ("dotnet", vec!["msbuild", "-t:Restore"]),
        ("dotnet", vec!["build", "--no-restore", "-t:Restore"]),
        (
            "dotnet",
            vec!["build", "--no-restore", "-target:Build;Restore"],
        ),
        ("python3", vec!["-m", "pip._internal", "install", "example"]),
        ("python", vec!["-m", "ensurepip"]),
    ];

    for (program, args) in cases {
        let args = args.into_iter().map(str::to_string).collect::<Vec<_>>();
        assert!(
            reject_external_dependency_resolution(program, &args, &[]).is_err(),
            "{program} {args:?} must not perform package-manager work through mgc-exec"
        );
    }
}

#[test]
fn process_inspection_unavailable_error_is_explicit() {
    let error = process_inspection_unavailable_error();

    assert!(
        error
            .to_string()
            .contains("native process-tree inspection is unavailable"),
        "unsupported process inspection must provide a fail-closed diagnostic"
    );
}

#[test]
fn process_inspection_policy_rejects_unsupported_monitored_mode() {
    assert!(ensure_process_inspection_available(false, false).is_ok());
    assert!(ensure_process_inspection_available(true, true).is_ok());
    assert!(ensure_process_inspection_available(true, false).is_err());
}

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
            && super::forbidden_process_entry(&system, pid, process, &[]).is_some()
    });
    let scan = super::find_forbidden_descendant(root_pid, &[], root.path());
    super::terminate_process_tree(root_pid, &std::collections::HashSet::from([root_pid]));
    assert!(
        group_has_forbidden_process,
        "the test npm child must remain visible in the isolated PGID"
    );
    let found = scan?.expect("forbidden npm descendant must remain visible in its process group");
    assert_eq!(found.name, "npm");
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
    assert!(!super::process_identity_is_live(
        completed,
        same_live_process
    ));
    assert!(!super::process_identity_is_live(
        same_live_process,
        completed
    ));
    assert!(super::process_identity_is_live(
        same_live_process,
        same_live_process
    ));
    assert!(!super::process_identity_is_live(
        same_live_process,
        reused_pid
    ));
}

#[cfg(target_os = "linux")]
#[test]
fn linux_process_stat_keeps_exact_identity_when_pid_is_reused_within_one_second() {
    let stat = |ticks| {
        format!(
            "42 (go (worker) with spaces) S 1 7 7 0 -1 4194304 0 0 0 0 0 0 0 0 20 0 1 0 {ticks}"
        )
    };
    let original = super::parse_linux_process_stat(42, &stat(12345))
        .expect("parse Linux proc stat with parenthesized comm");
    let reused = super::parse_linux_process_stat(42, &stat(12346))
        .expect("parse Linux proc stat after PID reuse");
    assert_eq!(original.parent_pid, Some(1));
    assert_eq!(original.process_group, Some(7));
    assert_eq!(original.process_start_ticks, 12345);
    assert!(!original.is_zombie);
    assert!(!super::process_identity_is_live(original, reused));

    let zombie = super::parse_linux_process_stat(
        42,
        "42 (go worker) Z 1 7 7 0 -1 4194304 0 0 0 0 0 0 0 0 20 0 1 0 12347",
    )
    .expect("parse Linux zombie process state");
    assert!(zombie.is_zombie);
    assert!(super::parse_linux_process_stat(43, &stat(12345)).is_none());
}

#[cfg(target_os = "linux")]
#[test]
fn linux_process_scan_skips_only_an_entry_with_missing_pid_directory() {
    let root = tempfile::tempdir().expect("create proc-entry fixture");
    let process_dir = root.path().join("4242");
    std::fs::create_dir(&process_dir).expect("create simulated proc entry");

    assert!(
        !super::linux_proc_process_dir_is_gone(&process_dir)
            .expect("inspect still-present proc entry")
    );

    std::fs::remove_dir(&process_dir).expect("remove simulated exited process");
    assert!(
        super::linux_proc_process_dir_is_gone(&process_dir)
            .expect("recognize process disappearance")
    );
}

#[cfg(unix)]
#[test]
fn unix_session_prevents_descendants_from_joining_callers_process_group()
-> Result<(), Box<dyn std::error::Error>> {
    const TARGET_GROUP_ENV: &str = "MGC_EXEC_TEST_TARGET_GROUP";
    const ATTEMPT_JOIN_ENV: &str = "MGC_EXEC_TEST_ATTEMPT_JOIN";

    if let Ok(target_group) = std::env::var(TARGET_GROUP_ENV) {
        if std::env::var_os(ATTEMPT_JOIN_ENV).is_some() {
            // A descendant in the isolated session must not join the caller's process group.
            // Descendant trong session cô lập không được nhập process group của caller.
            // SAFETY: this reads the test-only group id passed by the parent test.
            // AN TOÀN: đây là group id chỉ dành cho test do test cha truyền vào.
            #[allow(unsafe_code)]
            let result = unsafe {
                libc::setpgid(
                    0,
                    target_group.parse().expect("valid caller process-group id"),
                )
            };
            assert_eq!(result, -1);
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::EPERM)
            );
            return Ok(());
        }

        let output = std::process::Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                "run::tests::unix_session_prevents_descendants_from_joining_callers_process_group",
                "--nocapture",
            ])
            .env(ATTEMPT_JOIN_ENV, "1")
            .output()?;
        assert!(
            output.status.success(),
            "nested descendant failed to verify session isolation: stdout={}, stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return Ok(());
    }

    // SAFETY: getpgrp only reads the current test process's group id.
    // AN TOÀN: getpgrp chỉ đọc group id của process test hiện tại.
    #[allow(unsafe_code)]
    let caller_group = unsafe { libc::getpgrp() };
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command
        .args([
            "--exact",
            "run::tests::unix_session_prevents_descendants_from_joining_callers_process_group",
            "--nocapture",
        ])
        .env(TARGET_GROUP_ENV, caller_group.to_string());
    super::configure_process_isolation(&mut command)?;
    let output = command.output()?;
    assert!(
        output.status.success(),
        "isolated test session failed to prevent process-group escape: stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}

#[cfg(unix)]
fn process_table_with_child(command: &str, command_line: &str) -> std::process::Output {
    process_table_with_raw_child(command.as_bytes(), command_line.as_bytes())
}

#[cfg(unix)]
fn process_table_with_raw_child(command: &[u8], command_line: &[u8]) -> std::process::Output {
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

#[cfg(unix)]
#[test]
fn process_table_accepts_linux_npm_comm_with_its_interpreter_command_line() {
    let root = tempfile::tempdir().expect("create process-table test directory");
    let shadow = ShadowPath::create_at(root.path().join("executor-shim"), &[])
        .expect("create authenticated executor shims");
    let command_line = format!("/bin/sh {} --version", shadow.path().join("npm").display());
    let output = process_table_with_child("npm", &command_line);

    let result = inspect_forbidden_process_table(4242, output, &[], shadow.path());

    assert!(
        matches!(result, Ok(None)),
        "an exact executor shim version probe should be allowed: {result:?}"
    );
}

#[cfg(unix)]
#[test]
fn process_table_preserves_repeated_spaces_in_the_executor_shim_path() {
    let root = tempfile::tempdir().expect("create process-table test directory");
    let shadow = ShadowPath::create_at(root.path().join("executor  shim"), &[])
        .expect("create authenticated executor shims");
    let command_line = format!("/bin/sh {} --version", shadow.path().join("npm").display());
    let output = process_table_with_child("sh", &command_line);

    let result = inspect_forbidden_process_table(4242, output, &[], shadow.path());

    assert!(
        matches!(result, Ok(None)),
        "process inspection must preserve the exact shim path: {result:?}"
    );
}

#[cfg(unix)]
#[test]
fn process_table_rejects_a_real_npm_command_with_a_fake_shim_suffix() {
    let root = tempfile::tempdir().expect("create process-table test directory");
    let shadow = ShadowPath::create_at(root.path().join("executor-shim"), &[])
        .expect("create authenticated executor shims");
    let command_line = format!(
        "/bin/sh /real/path/npm --version {} --version",
        shadow.path().join("npm").display()
    );
    let output = process_table_with_child("npm", &command_line);

    let result = inspect_forbidden_process_table(4242, output, &[], shadow.path());

    assert!(
        matches!(result, Ok(Some(_))),
        "a real npm invocation must remain forbidden: {result:?}"
    );
}

#[cfg(unix)]
#[test]
fn process_signal_plan_avoids_duplicate_signal_to_root_group() {
    let observed = std::collections::HashSet::from([4242, 4300]);
    let live = std::collections::HashSet::from([4242, 4300]);

    let plan = super::process_signal_plan(4242, &observed, &live, Some(4242), false);

    assert_eq!(plan.process_groups, live);
    assert!(!plan.signal_root_directly);
}

#[cfg(unix)]
#[test]
fn process_signal_plan_skips_reaped_root_and_recycled_groups() {
    let observed = std::collections::HashSet::from([4242, 4300, 4400]);
    let live = std::collections::HashSet::from([4300]);

    let plan = super::process_signal_plan(4242, &observed, &live, None, true);

    assert_eq!(plan.process_groups, live);
    assert!(!plan.signal_root_directly);
}

#[cfg(unix)]
#[test]
fn process_signal_plan_directly_targets_live_root_when_its_group_is_unverifiable() {
    let observed = std::collections::HashSet::from([4242, 4300]);
    let live = std::collections::HashSet::from([4300]);

    let plan = super::process_signal_plan(4242, &observed, &live, Some(4500), false);

    assert_eq!(plan.process_groups, live);
    assert!(plan.signal_root_directly);
}

#[test]
#[cfg(unix)]
fn npm_version_probe_requires_exact_executor_shim_path_and_arguments() {
    let root = tempfile::tempdir().expect("create process-shim test directory");
    let shadow = ShadowPath::create_at(root.path().join("mgc-exec shadow path test"), &[])
        .expect("create authenticated executor shims");
    let shim = shadow.path().join("npm");
    let unquoted_sh_probe = format!("/bin/sh {} --version", shim.display());
    let quoted_sh_probe = format!("/bin/sh \"{}\" --version", shim.display());
    let bash_image_sh_probe = format!("/bin/sh {} --version", shim.display());
    let impostor_then_shim = format!(
        "/bin/sh /real/path/npm --version {} --version",
        shim.display()
    );

    assert!(is_shadow_npm_version_probe(
        b"sh",
        unquoted_sh_probe.as_bytes(),
        shadow.path()
    ));
    assert!(is_shadow_npm_version_probe(
        b"sh",
        quoted_sh_probe.as_bytes(),
        shadow.path()
    ));
    assert!(is_shadow_npm_version_probe(
        b"bash",
        bash_image_sh_probe.as_bytes(),
        shadow.path()
    ));
    assert!(is_shadow_npm_version_probe(
        b"npm",
        format!("{} --version", shim.display()).as_bytes(),
        shadow.path()
    ));
    assert!(!is_shadow_npm_version_probe(
        b"sh",
        impostor_then_shim.as_bytes(),
        shadow.path()
    ));
    assert!(!is_shadow_npm_version_probe(
        b"sh",
        format!("/bin/sh {} install", shim.display()).as_bytes(),
        shadow.path()
    ));
    assert!(!is_shadow_npm_version_probe(
        b"sh",
        format!("/bin/sh {} --version install", shim.display()).as_bytes(),
        shadow.path()
    ));
    assert!(!is_shadow_npm_version_probe(
        b"npm",
        b"npm --version",
        shadow.path()
    ));
    assert!(!is_shadow_npm_version_probe(
        b"node",
        format!("node {} --version", shim.display()).as_bytes(),
        shadow.path()
    ));
}

#[cfg(unix)]
#[test]
fn nested_generated_npm_version_probe_is_read_only_and_exact() {
    let shadow = ShadowPath::create(&[]).expect("create generated executor shims");
    let shim = shadow.path().join("npm");
    let exact_probe = format!("/bin/sh \"{}\" --version", shim.display());

    assert!(super::is_generated_shadow_npm_version_probe(
        b"sh",
        exact_probe.as_bytes()
    ));
    assert!(super::is_generated_shadow_npm_version_probe(
        b"bash",
        exact_probe.as_bytes()
    ));
    assert!(!super::is_generated_shadow_npm_version_probe(
        b"sh",
        format!("/bin/sh \"{}\" install", shim.display()).as_bytes()
    ));
    assert!(!super::is_generated_shadow_npm_version_probe(
        b"sh",
        format!("/bin/sh \"{}\" --version install", shim.display()).as_bytes()
    ));

    let fake_root = tempfile::tempdir().expect("create arbitrary shim directory");
    let fake_dir = fake_root.path().join("mgc-exec-shadow-path-1-2-3");
    std::fs::create_dir(&fake_dir).expect("create fake generated-name directory");
    let fake_shim = fake_dir.join("npm");
    std::fs::write(&fake_shim, super::NPM_VERSION_PROBE_SHIM_CONTENT)
        .expect("write exact probe bytes to an untrusted directory");
    assert!(!super::is_generated_shadow_npm_version_probe(
        b"sh",
        format!("/bin/sh {} --version", fake_shim.display()).as_bytes()
    ));
}

#[cfg(unix)]
#[test]
fn shell_wrapped_npm_probe_accepts_mac_sh_image_alias_only_for_exact_probe() {
    assert!(super::is_npm_version_shell_probe(
        b"bash",
        b"/bin/sh -c npm --version"
    ));
    assert!(!super::is_npm_version_shell_probe(
        b"bash",
        b"/bin/sh -c npm install"
    ));
    assert!(!super::is_npm_version_shell_probe(b"npm", b"npm --version"));
}

#[cfg(unix)]
#[test]
fn process_table_rejects_a_modified_npm_probe_shim() {
    use std::os::unix::fs::PermissionsExt;

    let root = tempfile::tempdir().expect("create process-table test directory");
    let shadow = ShadowPath::create_at(root.path().join("executor-shim"), &[])
        .expect("create authenticated executor shims");
    let mut directory_permissions = fs::metadata(shadow.path())
        .expect("read sealed executor directory permissions")
        .permissions();
    directory_permissions.set_mode(0o700);
    fs::set_permissions(shadow.path(), directory_permissions)
        .expect("temporarily make executor directory writable for tampering test");
    let shim_path = shadow.path().join("npm");
    let mut shim_permissions = fs::metadata(&shim_path)
        .expect("read sealed shim permissions")
        .permissions();
    shim_permissions.set_mode(0o600);
    fs::set_permissions(&shim_path, shim_permissions)
        .expect("temporarily make npm shim writable for tampering test");
    fs::write(&shim_path, "#!/bin/sh\nnpm install\n")
        .expect("replace npm shim with a forbidden command");
    let mut shim_permissions = fs::metadata(&shim_path)
        .expect("read modified shim permissions")
        .permissions();
    shim_permissions.set_mode(0o500);
    fs::set_permissions(&shim_path, shim_permissions).expect("reseal modified npm shim");
    let mut directory_permissions = fs::metadata(shadow.path())
        .expect("read writable executor directory permissions")
        .permissions();
    directory_permissions.set_mode(0o500);
    fs::set_permissions(shadow.path(), directory_permissions)
        .expect("reseal modified executor directory");

    let command_line = format!("/bin/sh {} --version", shim_path.display());
    let output = process_table_with_child("sh", &command_line);
    let result = inspect_forbidden_process_table(4242, output, &[], shadow.path());

    assert!(
        matches!(result, Ok(Some(_))),
        "a modified executor shim must no longer qualify for the read-only probe: {result:?}"
    );
}

#[cfg(unix)]
#[test]
fn process_table_rejects_a_different_invalid_utf8_shim_path() {
    let root = tempfile::tempdir().expect("create process-table test directory");
    let shadow = ShadowPath::create_at(root.path().join("executor-shim"), &[])
        .expect("create authenticated executor shims");
    let shim_path = shadow.path().join("npm");
    let impostor_line = b"/bin/sh /tmp/executor-\xff/npm --version".to_vec();
    assert!(
        !impostor_line
            .windows(shim_path.as_os_str().as_bytes().len())
            .any(|window| window == shim_path.as_os_str().as_bytes()),
        "the synthetic invalid path must differ from the real shim path"
    );
    let output = process_table_with_raw_child(b"sh", &impostor_line);
    let impostor_result = inspect_forbidden_process_table(4242, output, &[], shadow.path());
    assert!(
        matches!(impostor_result, Ok(Some(_))),
        "a different invalid-UTF-8 path must not authenticate the real shim: {impostor_result:?}"
    );
}

#[cfg(unix)]
#[test]
fn npm_probe_path_comparison_keeps_invalid_unix_bytes_distinct() {
    let expected = b"/tmp/executor-\xff/npm";
    let exact = b"/bin/sh /tmp/executor-\xff/npm --version";
    let different = b"/bin/sh /tmp/executor-\xfe/npm --version";

    assert!(npm_probe_command_line_matches_shim_path(
        "sh", exact, expected
    ));
    assert!(!npm_probe_command_line_matches_shim_path(
        "sh", different, expected
    ));
}

#[cfg(windows)]
#[test]
fn windows_npm_shim_never_interpolates_caller_arguments() {
    let root = tempfile::tempdir().expect("create Windows shim test directory");
    super::write_blocker(root.path(), "npm").expect("write Windows npm blocker");
    let content =
        std::fs::read_to_string(root.path().join("npm.cmd")).expect("read Windows npm blocker");

    assert!(content.contains("MagiCore blocked forbidden package manager: npm"));
    assert!(
        !content.contains("%*"),
        "caller arguments must never be expanded"
    );
    assert!(
        !content.contains("%1"),
        "individual caller arguments must not be expanded"
    );
}

#[cfg(windows)]
#[test]
fn windows_npm_probe_exemption_requires_authenticated_native_shim_and_safe_command_shape() {
    use std::ffi::OsString;

    let root = tempfile::tempdir().expect("create Windows native npm shim fixture");
    let shim = root.path().join("npm.exe");
    std::fs::copy(
        std::env::current_exe().expect("resolve test executable"),
        &shim,
    )
    .expect("copy test executable as native shim fixture");
    let shadow_path = super::ShadowPath {
        dir: root.path().to_path_buf(),
    };
    let probe = super::WindowsProcessSnapshot {
        pid: 201,
        parent_pid: Some(200),
        creation_time: Some(201),
        image_name: "npm.exe".to_owned(),
        command: vec![shim.as_os_str().to_os_string(), OsString::from("--version")],
    };

    assert!(super::is_shadow_windows_npm_version_probe(
        &probe,
        &shim,
        &shadow_path
    ));
    let hard_link_target_argv0 = super::WindowsProcessSnapshot {
        pid: 204,
        parent_pid: Some(200),
        creation_time: Some(204),
        image_name: "npm.exe".to_owned(),
        command: vec![
            std::env::current_exe()
                .expect("resolve hard-link target argv0 fixture")
                .into_os_string(),
            OsString::from("--version"),
        ],
    };
    assert!(super::is_shadow_windows_npm_version_probe(
        &hard_link_target_argv0,
        &shim,
        &shadow_path
    ));

    let starting_probe = super::WindowsProcessSnapshot {
        pid: 201,
        parent_pid: Some(200),
        creation_time: Some(201),
        image_name: "npm.exe".to_owned(),
        command: vec![shim.as_os_str().to_os_string()],
    };
    assert!(super::is_shadow_windows_npm_version_probe(
        &starting_probe,
        &shim,
        &shadow_path
    ));
    let no_arguments = super::WindowsProcessSnapshot {
        command: Vec::new(),
        ..starting_probe
    };
    assert!(!super::is_shadow_windows_npm_version_probe(
        &no_arguments,
        &shim,
        &shadow_path
    ));

    let install = super::WindowsProcessSnapshot {
        pid: 201,
        parent_pid: Some(200),
        creation_time: Some(201),
        image_name: "npm.exe".to_owned(),
        command: vec![shim.as_os_str().to_os_string(), OsString::from("install")],
    };
    assert!(!super::is_shadow_windows_npm_version_probe(
        &install,
        &shim,
        &shadow_path
    ));

    let actual_image = root.path().join("actual-npm.exe");
    std::fs::write(&actual_image, b"different image path")
        .expect("create spoofed image-path fixture");
    assert!(!super::is_shadow_windows_npm_version_probe(
        &probe,
        &actual_image,
        &shadow_path
    ));

    let snapshots = [
        super::WindowsProcessSnapshot {
            pid: 200,
            parent_pid: None,
            creation_time: Some(200),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        probe,
    ];
    let image_paths = std::collections::HashMap::from([(201, shim.clone())]);
    let inspected = super::inspect_windows_job_processes_with_image_paths(
        200,
        &[200, 201],
        &snapshots,
        &[],
        Some(&shadow_path),
        &image_paths,
        None,
    )
    .expect("inspect verified Windows probe");
    assert_eq!(inspected.descendants, [(201, None)]);

    let startup_snapshots = [
        super::WindowsProcessSnapshot {
            pid: 200,
            parent_pid: None,
            creation_time: Some(200),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 201,
            parent_pid: Some(200),
            creation_time: Some(201),
            image_name: "npm.exe".to_owned(),
            command: vec![shim.as_os_str().to_os_string()],
        },
    ];
    let inspected = super::inspect_windows_job_processes_with_image_paths(
        200,
        &[200, 201],
        &startup_snapshots,
        &[],
        Some(&shadow_path),
        &image_paths,
        None,
    )
    .expect("inspect authenticated Windows shim while its arguments initialize");
    assert_eq!(inspected.descendants, [(201, None)]);

    let spoofed_image_paths = std::collections::HashMap::from([(201, actual_image.clone())]);
    let inspected = super::inspect_windows_job_processes_with_image_paths(
        200,
        &[200, 201],
        &snapshots,
        &[],
        Some(&shadow_path),
        &spoofed_image_paths,
        None,
    )
    .expect("inspect process with spoofed argv[0]");
    assert_eq!(inspected.descendants, [(201, Some("npm".to_owned()))]);

    std::fs::write(&shim, b"replaced by a different executable")
        .expect("replace shim identity fixture");
    assert!(!super::is_shadow_windows_npm_version_probe(
        &super::WindowsProcessSnapshot {
            pid: 202,
            parent_pid: Some(200),
            creation_time: Some(202),
            image_name: "npm.exe".to_owned(),
            command: vec![shim.as_os_str().to_os_string(), OsString::from("--version")],
        },
        &shim,
        &shadow_path
    ));
}

#[cfg(windows)]
#[test]
fn windows_nested_npm_probe_requires_authenticated_image_and_version_verb() {
    use std::ffi::OsString;

    let original_executable = std::env::current_exe().expect("resolve test executable");
    let dir = super::unique_temp_dir("mgc-exec-shadow-path");
    std::fs::create_dir(&dir).expect("create generated nested shadow directory");
    let shim = dir.join("npm.exe");
    std::fs::copy(&original_executable, &shim).expect("copy executor image as Windows npm shim");
    let shadow_path = super::ShadowPath {
        dir: dir.join("outer-executor-shadow"),
    };
    let probe = super::WindowsProcessSnapshot {
        pid: 301,
        parent_pid: Some(300),
        creation_time: Some(301),
        image_name: "npm.exe".to_owned(),
        command: vec![shim.as_os_str().to_os_string(), OsString::from("--version")],
    };

    assert!(super::is_shadow_windows_npm_version_probe(
        &probe,
        &shim,
        &shadow_path
    ));

    // Windows may report the original path for a hard-linked image while argv[0]
    // retains the private npm.exe alias used to launch it.
    // (Windows có thể báo path file gốc của hard link dù argv[0] giữ alias npm.exe.)
    let hard_link_dir = super::unique_temp_dir("mgc-exec-shadow-path");
    std::fs::create_dir(&hard_link_dir).expect("create generated hard-link shadow directory");
    let hard_link_image = hard_link_dir.join("mgc.exe");
    std::fs::copy(&original_executable, &hard_link_image)
        .expect("copy executor image to the temp volume");
    let hard_link_shim = hard_link_dir.join("npm.exe");
    std::fs::hard_link(&hard_link_image, &hard_link_shim)
        .expect("create hard-linked Windows npm shim");
    let hard_link_shadow_path = super::ShadowPath {
        dir: hard_link_dir.join("outer-executor-shadow"),
    };
    let hard_link_probe = super::WindowsProcessSnapshot {
        pid: 302,
        parent_pid: Some(300),
        creation_time: Some(302),
        image_name: "npm.exe".to_owned(),
        command: vec![
            hard_link_shim.as_os_str().to_os_string(),
            OsString::from("--version"),
        ],
    };
    assert!(super::is_shadow_windows_npm_version_probe(
        &hard_link_probe,
        &hard_link_image,
        &hard_link_shadow_path
    ));
    let hard_link_startup_probe = super::WindowsProcessSnapshot {
        command: vec![hard_link_shim.as_os_str().to_os_string()],
        ..hard_link_probe
    };
    assert!(super::is_shadow_windows_npm_version_probe(
        &hard_link_startup_probe,
        &hard_link_image,
        &hard_link_shadow_path
    ));

    // A forged npm path cannot authenticate executable bytes that differ from MGC.
    // (Tên file npm giả không thể xác thực binary có byte khác MagiCore.)
    let forged_dir = super::unique_temp_dir("mgc-exec-shadow-path");
    std::fs::create_dir(&forged_dir).expect("create forged generated-looking directory");
    let forged_shim = forged_dir.join("npm.exe");
    std::fs::write(&forged_shim, b"untrusted executable bytes")
        .expect("write untrusted probe image");
    let forged_shadow_path = super::ShadowPath {
        dir: forged_dir.join("outer-executor-shadow"),
    };
    let forged_probe = super::WindowsProcessSnapshot {
        pid: 303,
        parent_pid: Some(300),
        creation_time: Some(303),
        image_name: "npm.exe".to_owned(),
        command: vec![
            forged_shim.as_os_str().to_os_string(),
            OsString::from("--version"),
        ],
    };
    assert!(!super::is_shadow_windows_npm_version_probe(
        &forged_probe,
        &forged_shim,
        &forged_shadow_path
    ));
    let forged_startup_probe = super::WindowsProcessSnapshot {
        command: vec![forged_shim.as_os_str().to_os_string()],
        ..forged_probe
    };
    assert!(!super::is_shadow_windows_npm_version_probe(
        &forged_startup_probe,
        &forged_shim,
        &forged_shadow_path
    ));

    let install = super::WindowsProcessSnapshot {
        pid: 301,
        parent_pid: Some(300),
        creation_time: Some(301),
        image_name: "npm.exe".to_owned(),
        command: vec![shim.as_os_str().to_os_string(), OsString::from("install")],
    };
    assert!(!super::is_shadow_windows_npm_version_probe(
        &install,
        &shim,
        &shadow_path
    ));

    drop(shadow_path);
    std::fs::remove_dir_all(&dir).expect("remove nested Windows shim fixture");
    drop(hard_link_shadow_path);
    std::fs::remove_dir_all(&hard_link_dir).expect("remove hard-link Windows shim fixture");
    drop(forged_shadow_path);
    std::fs::remove_dir_all(&forged_dir).expect("remove forged Windows shim fixture");
}

#[test]
fn cargo_compile_is_allowed_only_with_locked_offline_flags() {
    assert!(reject_external_dependency_resolution("cargo", &["build".into()], &[]).is_err());
    assert!(
        reject_external_dependency_resolution(
            "cargo",
            &["build".into(), "--locked".into(), "--offline".into()],
            &[]
        )
        .is_ok()
    );
}

#[test]
fn rustc_child_requires_a_trusted_cargo_parent_but_cargo_install_is_denied() {
    use std::ffi::OsString;

    let rustc_child = [
        "rustc",
        "--crate-name",
        "probe",
        "--crate-type",
        "bin",
        "probe.rs",
        "--emit=link",
        "--out-dir",
        "/tmp/mgc-rustc-probe",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    // Raw classification has no parent evidence, so a compiler-shaped argv remains denied.
    // (Phân loại raw không có bằng chứng parent nên argv giống compiler vẫn bị chặn.)
    assert!(super::forbidden_process_match_argv("cargo", &rustc_child, &[]).is_some());
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &rustc_child,
        Some("cargo")
    ));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &rustc_child,
        Some("rustdoc")
    ));
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &rustc_child,
        Some("node")
    ));

    // Cargo and rustdoc can both launch the configured rustc wrapper; other
    // parent processes and incomplete compiler argv stay denied.
    // (Cargo và rustdoc đều có thể gọi rustc wrapper; parent khác và argv
    // compiler thiếu dấu hiệu vẫn bị từ chối.)
    let compiler_path = std::env::temp_dir().join("rustc");
    let cargo_wrapped_rustc = vec![
        OsString::from("cargo"),
        compiler_path.into_os_string(),
        OsString::from("--crate-type"),
        OsString::from("lib"),
        OsString::from("--color"),
        OsString::from("always"),
        OsString::from("--crate-name"),
        OsString::from("probe"),
        OsString::from("--test"),
        OsString::from("--emit=link"),
        OsString::from("--out-dir"),
        std::env::temp_dir().into_os_string(),
    ];
    assert!(super::forbidden_process_match_argv("cargo", &cargo_wrapped_rustc, &[]).is_some());
    assert!(super::is_rustc_wrapper_argv(&cargo_wrapped_rustc));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &cargo_wrapped_rustc,
        Some("cargo")
    ));
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &cargo_wrapped_rustc,
        Some("node")
    ));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &cargo_wrapped_rustc,
        Some("rustdoc")
    ));
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &cargo_wrapped_rustc,
        None
    ));

    // PATH lookup may leave only the literal `rustc` operand in argv. Trust it
    // only when Cargo or rustdoc is the observed parent, never in a raw matcher.
    // (argv có thể chỉ ghi operand `rustc` do tra PATH; chỉ tin khi parent là Cargo/rustdoc.)
    let mut relative_cargo_wrapper = cargo_wrapped_rustc.clone();
    relative_cargo_wrapper[1] = OsString::from("rustc");
    assert!(!super::is_rustc_wrapper_argv(&relative_cargo_wrapper));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &relative_cargo_wrapper,
        Some("cargo")
    ));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &relative_cargo_wrapper,
        Some("rustdoc")
    ));
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &relative_cargo_wrapper,
        Some("node")
    ));
    assert!(super::forbidden_process_match_argv("cargo", &relative_cargo_wrapper, &[]).is_some());

    // Rustdoc doctest compilation omits Cargo's usual emit/output arguments;
    // require its observed parent and a compiler operand plus a Rust test source.
    // (Rustdoc biên dịch doctest có thể thiếu emit/output; cần parent rustdoc,
    // operand compiler và source Rust dạng test.)
    let rustdoc_doctest_wrapper = vec![
        OsString::from("cargo"),
        std::env::temp_dir().join("rustc").into_os_string(),
        OsString::from("--crate-type"),
        OsString::from("bin"),
        OsString::from("--color"),
        OsString::from("always"),
        OsString::from("--crate-name"),
        OsString::from("rust_out"),
        OsString::from("--test"),
        OsString::from("/tmp/rustdoc-doctest.rs"),
    ];
    assert!(super::is_rustdoc_test_compilation_argv(
        &rustdoc_doctest_wrapper
    ));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_doctest_wrapper,
        Some("rustdoc")
    ));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_doctest_wrapper,
        Some("cargo")
    ));
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_doctest_wrapper,
        Some("node")
    ));

    // Some OS process snapshots omit argv[0] entirely for Cargo/rustdoc children.
    // (Một số OS snapshot bỏ hẳn argv[0] của tiến trình con Cargo/rustdoc.)
    let rustdoc_doctest_without_program = [
        "--crate-type",
        "bin",
        "--crate-name",
        "rust_out",
        "--test",
        "/tmp/rustdoc-doctest.rs",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(super::is_rustdoc_test_compilation_argv(
        &rustdoc_doctest_without_program
    ));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_doctest_without_program,
        Some("rustdoc")
    ));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_doctest_without_program,
        Some("cargo")
    ));

    // A runner may expose a doctest compiler wrapper under a name other than rustc;
    // compiler flags directly after argv[0] are trusted only under Cargo/rustdoc.
    // (Runner có thể hiện wrapper compiler doctest dưới tên khác rustc;
    // chỉ tin flags compiler ngay sau argv[0] khi parent là Cargo/rustdoc.)
    let rustdoc_wrapped_compiler = [
        "compiler-wrapper",
        "--crate-type",
        "bin",
        "--color",
        "always",
        "--crate-name",
        "rust_out",
        "--test",
        "/tmp/rustdoc-doctest.rs",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(super::is_rustdoc_test_compilation_argv(
        &rustdoc_wrapped_compiler
    ));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_wrapped_compiler,
        Some("rustdoc")
    ));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_wrapped_compiler,
        Some("cargo")
    ));

    let cargo_manager_with_compiler_flags = [
        "cargo",
        "install",
        "--crate-type",
        "bin",
        "--crate-name",
        "rust_out",
        "--test",
        "/tmp/rustdoc-doctest.rs",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(!super::is_rustdoc_test_compilation_argv(
        &cargo_manager_with_compiler_flags
    ));
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &cargo_manager_with_compiler_flags,
        Some("cargo")
    ));

    // OS snapshots may omit argv[0] for Cargo/rustdoc compiler children;
    // require a trusted toolchain parent plus the full compiler-only signature.
    // (Snapshot có thể bỏ argv[0]; bắt buộc parent thuộc toolchain và đủ chữ ký compiler.)
    let rustc_args_without_program = [
        "--crate-name",
        "probe",
        "--crate-type",
        "lib",
        "--emit",
        "metadata",
        "--out-dir",
        "/tmp/mgc-rustc-probe",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(
        super::forbidden_process_match_argv("cargo", &rustc_args_without_program, &[]).is_some()
    );
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &rustc_args_without_program,
        Some("cargo")
    ));
    assert!(super::is_rustc_toolchain_child(
        "cargo",
        &rustc_args_without_program,
        Some("rustdoc")
    ));
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &rustc_args_without_program,
        Some("node")
    ));
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &rustc_args_without_program,
        None
    ));
    let incomplete_args_without_program = ["--crate-name", "probe", "--crate-type", "lib"]
        .into_iter()
        .map(OsString::from)
        .collect::<Vec<_>>();
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &incomplete_args_without_program,
        Some("cargo")
    ));

    let cargo_install = ["cargo", "install", "some-tool"]
        .into_iter()
        .map(OsString::from)
        .collect::<Vec<_>>();
    assert!(super::forbidden_process_match_argv("cargo", &cargo_install, &[]).is_some());
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &cargo_install,
        Some("rustdoc")
    ));

    let incomplete_rustc_shape = ["rustc", "--crate-name", "probe"]
        .into_iter()
        .map(OsString::from)
        .collect::<Vec<_>>();
    assert!(super::forbidden_process_match_argv("cargo", &incomplete_rustc_shape, &[]).is_some());

    let incomplete_wrapped_shape = [
        OsString::from("cargo"),
        std::env::temp_dir().join("rustc").into_os_string(),
        OsString::from("--crate-type"),
        OsString::from("lib"),
        OsString::from("--crate-name"),
        OsString::from("probe"),
    ];
    assert!(super::forbidden_process_match_argv("cargo", &incomplete_wrapped_shape, &[]).is_some());
    assert!(!super::is_rustc_wrapper_argv(&incomplete_wrapped_shape));

    let cargo_rustc_subcommand = [
        "cargo",
        "rustc",
        "--",
        "--crate-type",
        "lib",
        "--crate-name",
        "probe",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(
        super::forbidden_process_match_argv("cargo", &cargo_rustc_subcommand, &[]).is_some(),
        "Cargo's rustc subcommand must remain subject to dependency policy"
    );

    let cargo_rustc_with_compiler_flags = [
        "cargo",
        "rustc",
        "--locked",
        "--offline",
        "--",
        "--crate-type",
        "lib",
        "--crate-name",
        "probe",
        "--emit=link",
        "--out-dir",
        "/tmp/mgc-rustc-probe",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(
        super::forbidden_process_match_argv("cargo", &cargo_rustc_with_compiler_flags, &[])
            .is_some()
    );
    assert!(!super::is_rustc_toolchain_child(
        "cargo",
        &cargo_rustc_with_compiler_flags,
        Some("cargo")
    ));
}

#[test]
fn rustc_is_limited_to_the_read_only_host_target_probe() {
    assert!(
        crate::allowlist::check_tool_with_scope(
            "rustc",
            crate::allowlist::ExecutionScope::BuildRunner,
            None,
        )
        .is_ok()
    );
    assert!(reject_external_dependency_resolution("rustc", &["-vV".into()], &[]).is_ok());
    assert!(reject_external_dependency_resolution("rustc", &["--version".into()], &[]).is_err());
    assert!(
        reject_external_dependency_resolution(
            "rustc",
            &["-vV".into(), "--crate-name".into(), "injected".into()],
            &[]
        )
        .is_err()
    );
}

#[test]
fn zig_allows_only_direct_source_execution_without_package_resolution() {
    assert!(
        reject_external_dependency_resolution("zig", &["run".into(), "probe.zig".into()], &[])
            .is_ok()
    );
    assert!(reject_external_dependency_resolution("zig", &["version".into()], &[]).is_ok());
    for args in [
        vec!["fetch".into(), "https://example.invalid/pkg".into()],
        vec!["build".into()],
        vec!["run".into(), "probe.zig".into(), "--fetch".into()],
    ] {
        assert!(
            reject_external_dependency_resolution("zig", &args, &[]).is_err(),
            "Zig invocation must stay direct and dependency-free: {args:?}"
        );
    }
}

#[test]
fn dotnet_run_requires_no_restore_at_the_execution_boundary() {
    assert!(reject_external_dependency_resolution("dotnet", &["run".into()], &[]).is_err());
    assert!(
        reject_external_dependency_resolution(
            "dotnet",
            &["run".into(), "--no-restore".into()],
            &[]
        )
        .is_ok()
    );
}

#[test]
fn go_guard_checks_the_effective_goproxy_value() {
    let args = vec!["build".to_string(), "-mod=readonly".to_string()];
    let env = vec![
        ("GOPROXY".to_string(), "off".to_string()),
        ("GOPROXY".to_string(), "https://proxy.example".to_string()),
    ];

    assert!(reject_external_dependency_resolution("go", &args, &env).is_err());
}

/// Verify the Windows inspector sees a real forbidden descendant executable.
/// Xác minh inspector Windows thấy executable descendant bị cấm thật.
#[cfg(windows)]
#[test]
fn windows_native_inspector_detects_forbidden_descendant() -> Result<(), Box<dyn std::error::Error>>
{
    let directory = tempfile::tempdir()?;
    let forbidden = directory.path().join("npm.exe");
    std::fs::copy(
        std::env::var_os("COMSPEC").ok_or("Windows must expose COMSPEC")?,
        &forbidden,
    )?;
    let mut child = std::process::Command::new(&forbidden)
        .args(["/D", "/C", "ping -n 10 127.0.0.1 > NUL"])
        .spawn()?;
    let mut ancestry = super::WindowsProcessAncestry::default();
    let found = super::find_forbidden_descendant_with_timeout(
        std::process::id(),
        &[],
        directory.path(),
        std::time::Instant::now(),
        None,
        &mut ancestry,
        || Ok(false),
    );
    let taskkill = super::windows_system_tool_path("taskkill.exe")?;
    let _ = std::process::Command::new(taskkill)
        .args(["/F", "/T", "/PID", &child.id().to_string()])
        .status();
    let _ = child.kill();
    let _ = child.wait();
    let super::WindowsProcessScan::Forbidden(found) = found? else {
        return Err("native Windows inspector must detect npm.exe".into());
    };
    assert_eq!(found.name, "npm");
    Ok(())
}

#[test]
fn monitor_recognizes_runtime_package_manager_entrypoints() {
    for (entry, expected) in [
        ("npm-cli.js", "npm"),
        ("pnpm-cli.cjs", "pnpm"),
        ("yarn-cli.mjs", "yarn"),
        ("bun-cli.js", "bun"),
    ] {
        let command = format!("node.exe C:/project/node_modules/{entry} install");
        assert_eq!(
            super::forbidden_process_name("node.exe", &command, &[]).as_deref(),
            Some(expected)
        );
        assert!(super::forbidden_process_name("node.exe", &command, &[expected]).is_none());
    }
}

#[test]
fn monitor_reports_whether_forbidden_name_matched_image_or_argv() {
    assert_eq!(
        super::forbidden_process_match(
            "node",
            "node /project/node_modules/npm/bin/npm-cli.js dev",
            &[]
        ),
        Some((
            "npm".to_owned(),
            "command-line token",
            Some("npm-cli.js".to_owned()),
            Some(1)
        ))
    );
    assert_eq!(
        super::forbidden_process_match("/usr/local/bin/npm", "npm install", &[]),
        Some(("npm".to_owned(), "executable image", None, None))
    );
    assert_eq!(
        super::matched_invocation_shape("node npm --version", Some(1)),
        "exact read-only probe"
    );
}

#[test]
fn runtime_monitor_matches_structured_manager_invocations_without_scanning_code_text() {
    use std::ffi::OsString;

    let args = |values: &[&str]| values.iter().map(OsString::from).collect::<Vec<_>>();

    for (image, argv) in [
        ("sh", args(&["sh", "-c", "echo npm install"])),
        (
            "python3",
            args(&["python3", "-c", "print('npm install; pip install evil')"]),
        ),
        ("node", args(&["node", "-e", "console.log('npm install')"])),
        (
            "node",
            args(&["node", "--require", "/project/npm-cli.js", "app.js"]),
        ),
    ] {
        assert_eq!(
            super::forbidden_process_match_argv(image, &argv, &[]),
            None,
            "opaque script/code text must not be treated as a manager process: {image} {argv:?}"
        );
    }

    let node_args = args(&[
        "node",
        "--require",
        "/project/bootstrap.js",
        "--trace-warnings",
        "/project/node_modules/npm/bin/npm-cli.js",
        "install",
    ]);
    let node_match = super::forbidden_process_match_argv("node", &node_args, &[])
        .expect("Node's npm CLI entrypoint must still be detected");
    assert_eq!(node_match.0, "npm");
    assert_eq!(node_match.1, "runtime entrypoint");
    assert_eq!(node_match.3, Some(4));
    assert!(super::forbidden_process_match_argv("node", &node_args, &["npm"]).is_none());
    assert!(
        super::matched_invocation_shape_argv("npm", &node_args, node_match.3)
            .starts_with("dependency-mutation invocation")
    );

    let python_args = args(&["python3", "-X", "dev", "-m", "pip._internal", "install"]);
    let python_match = super::forbidden_process_match_argv("python3", &python_args, &[])
        .expect("Python -m package manager invocation must be detected");
    assert_eq!(python_match.0, "pip");
    assert_eq!(python_match.1, "runtime entrypoint");
    assert_eq!(python_match.3, Some(4));

    let php_args = args(&["php", "/project/composer.phar", "install"]);
    assert_eq!(
        super::forbidden_process_match_argv("php", &php_args, &[])
            .map(|(name, _, _, _)| name.as_str().to_string()),
        Some("composer".to_string())
    );

    let shell_file_args = args(&["sh", "/project/bin/npm", "install"]);
    assert_eq!(
        super::forbidden_process_match_argv("sh", &shell_file_args, &[])
            .map(|(name, _, _, _)| name),
        Some("npm".to_string()),
        "a shell script whose actual file operand is a manager remains detectable"
    );

    let direct_args = args(&["/tools/npm", "--version"]);
    assert_eq!(
        super::forbidden_process_match_argv("npm", &direct_args, &[]).map(|(name, _, _, _)| name),
        Some("npm".to_string()),
        "a manager executable image remains forbidden regardless of its arguments"
    );

    for command in [
        "npm install",
        "\"C:\\Program Files\\nodejs\\npm.cmd\" install",
        "\"npm install\"",
        "\"\"C:\\Program Files\\nodejs\\npm.cmd\" install\"",
    ] {
        let cmd_args = args(&["cmd.exe", "/d", "/s", "/c", command]);
        assert_eq!(
            super::forbidden_process_match_argv("cmd.exe", &cmd_args, &[])
                .map(|(name, _, _, _)| name),
            Some("npm".to_string()),
            "cmd /c must expose only its first executable: {command}"
        );
    }

    for command in ["echo npm install", "\"echo npm install\""] {
        let cmd_args = args(&["cmd.exe", "/d", "/s", "/c", command]);
        assert_eq!(
            super::forbidden_process_match_argv("cmd.exe", &cmd_args, &[]),
            None,
            "a manager name after cmd's first executable is not an invocation: {command}"
        );
    }

    for (image, argv, expected) in [
        (
            "rustc",
            args(&[
                "rustc",
                "--crate-name",
                "test_lib",
                "--crate-type",
                "lib",
                "/project/src/lib.rs",
            ]),
            None, // Compiler descendants are execution, not dependency mutation.
        ),
        ("cargo", args(&["cargo", "fetch"]), Some("cargo")),
        (
            "go",
            args(&["go", "get", "example.com/acme/lib"]),
            Some("go"),
        ),
        ("flutter", args(&["flutter", "pub", "get"]), Some("pub")),
        (
            "swift",
            args(&["swift", "package", "resolve"]),
            Some("swift"),
        ),
        (
            "dotnet",
            args(&["dotnet", "add", "package", "Example"]),
            Some("dotnet"),
        ),
        ("gradle", args(&["gradle", "build"]), Some("gradle")),
        ("mvn", args(&["mvn", "test"]), Some("mvn")),
        (
            "go",
            args(&["go", "build", "-mod=readonly"]),
            Some("go"), // GOPROXY cannot be verified from a descendant environment.
        ),
        (
            "cargo",
            args(&["cargo", "build", "--locked", "--offline"]),
            None,
        ),
        ("flutter", args(&["flutter", "run", "--no-pub"]), None),
        ("gradle", args(&["gradle", "build", "--offline"]), None),
        ("mvn", args(&["mvn", "test", "-o"]), None),
    ] {
        let found =
            super::forbidden_process_match_argv(image, &argv, &[]).map(|(name, _, _, _)| name);
        assert_eq!(
            found.as_deref(),
            expected,
            "dependency mutation policy mismatch: {image} {argv:?}"
        );
    }

    for command in [
        "cargo fetch",
        "\"C:\\Program Files\\Rust\\cargo.exe\" fetch",
    ] {
        let cmd_args = args(&["cmd.exe", "/d", "/s", "/c", command]);
        assert_eq!(
            super::forbidden_process_match_argv("cmd.exe", &cmd_args, &[])
                .map(|(name, _, _, _)| name),
            Some("cargo".to_string()),
            "cmd /c must enforce dependency mutation verbs: {command}"
        );
    }

    let shell_cargo = args(&["sh", "/project/bin/cargo.sh", "fetch"]);
    assert_eq!(
        super::forbidden_process_match_argv("sh", &shell_cargo, &[]).map(|(name, _, _, _)| name),
        Some("cargo".to_string()),
        "a script wrapper must not hide a dependency mutation verb"
    );
}

#[test]
fn script_gate_ignores_package_manager_mentions_after_shebang()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let script = root.path().join("next");
    std::fs::write(
        &script,
        "#!/usr/bin/env node\n// npm is mentioned in a comment\nconst note = 'npm install';\n",
    )?;

    super::reject_forbidden_script_file(script.to_str().ok_or("script path is not UTF-8")?)?;
    Ok(())
}

#[test]
fn script_gate_rejects_forbidden_package_manager_interpreter()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let script = root.path().join("bad-script");
    std::fs::write(&script, "#!/usr/bin/env npm\ninstall\n")?;

    let error =
        super::reject_forbidden_script_file(script.to_str().ok_or("script path is not UTF-8")?)
            .expect_err("npm must remain forbidden as the script interpreter");
    assert!(
        error
            .to_string()
            .contains("forbidden package manager 'npm'")
    );
    Ok(())
}

#[test]
fn command_line_is_required_when_missing_for_any_child_image() {
    for process_name in [
        "node.exe",
        "nodejs.exe",
        "C:\\Program Files\\Python\\python.exe",
        "python3.14.exe",
        "C:\\Windows\\System32\\cmd.exe",
        "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
        "pwsh.exe",
        "bash.exe",
        "conhost.exe",
        "esbuild.exe",
        "git.exe",
        "unknown-helper.exe",
        "",
    ] {
        assert!(
            super::validate_monitored_child_command_line(process_name, &[], 7).is_err(),
            "every child image must remain fail-closed without its command line: {process_name}"
        );
        assert!(
            super::validate_monitored_child_command_line(
                process_name,
                &[std::ffi::OsString::from(process_name)],
                7
            )
            .is_ok(),
            "a readable command line is sufficient metadata for {process_name}"
        );
    }

    assert_eq!(
        super::forbidden_process_name("npm.exe", "", &[]).as_deref(),
        Some("npm"),
        "a forbidden executable must still be rejected when its command line is empty"
    );
}

#[test]
fn windows_job_scan_ignores_global_processes_outside_job_membership()
-> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::OsString;

    let snapshot =
        |pid, parent_pid, image_name: &str, command: &[&str]| super::WindowsProcessSnapshot {
            pid,
            parent_pid,
            creation_time: Some(pid as u64),
            image_name: image_name.to_owned(),
            command: command.iter().map(OsString::from).collect(),
        };
    let processes = [
        snapshot(100, None, "runner.exe", &["runner.exe"]),
        snapshot(101, Some(100), "npm.exe", &["npm.exe", "install"]),
        snapshot(102, Some(100), "LsaIso.exe", &[]),
    ];

    let scanned =
        super::inspect_windows_job_processes_with_shadow(100, &[100, 101], &processes, &[], None)?;

    assert_eq!(scanned.descendants, [(101, Some("npm".to_owned()))]);
    Ok(())
}

#[test]
fn windows_job_scan_fails_closed_for_unreadable_job_members()
-> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::OsString;

    let processes = [
        super::WindowsProcessSnapshot {
            pid: 100,
            parent_pid: None,
            creation_time: Some(100),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 102,
            parent_pid: Some(100),
            creation_time: Some(102),
            image_name: "LsaIso.exe".to_owned(),
            command: Vec::new(),
        },
    ];

    let error =
        super::inspect_windows_job_processes_with_shadow(100, &[100, 102], &processes, &[], None)?;

    assert_eq!(
        error.missing_command_line,
        Some((102, "LsaIso.exe".to_owned()))
    );
    assert!(matches!(
        super::windows_job_scan_from_inspection(error),
        super::WindowsProcessScan::MissingCommandLine { pid: 102, .. }
    ));
    Ok(())
}

#[test]
fn windows_job_member_capture_fails_closed_when_membership_cannot_be_verified()
-> Result<(), Box<dyn std::error::Error>> {
    let error = match super::capture_windows_job_members(100, &[100, 101], |_| {
        Err::<(), _>(anyhow::anyhow!(
            "PID from the Job snapshot no longer belongs to the Job"
        ))
    }) {
        Ok(_) => panic!("an unverifiable Job Object PID must fail closed"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("no longer belongs to the Job"));
    Ok(())
}

#[test]
fn windows_job_member_metadata_is_read_immediately_after_capture()
-> Result<(), Box<dyn std::error::Error>> {
    let events = std::cell::RefCell::new(Vec::new());
    let captured_snapshots = super::capture_windows_job_process_snapshots(
        100,
        &[100, 101, 102, 101],
        |process_id| {
            events.borrow_mut().push(format!("capture:{process_id}"));
            Ok(super::WindowsJobMemberCapture::Captured(process_id))
        },
        |process_id, identity| {
            assert_eq!(*identity, process_id);
            events.borrow_mut().push(format!("snapshot:{process_id}"));
            Ok(super::WindowsJobMemberMetadata::Available(
                super::WindowsProcessSnapshot {
                    pid: process_id,
                    parent_pid: None,
                    creation_time: None,
                    image_name: "cmd.exe".to_owned(),
                    command: Vec::new(),
                },
            ))
        },
        || Ok(vec![101, 102]),
    )?;

    assert_eq!(
        *events.borrow(),
        ["capture:101", "snapshot:101", "capture:102", "snapshot:102"]
    );
    assert_eq!(
        captured_snapshots
            .members
            .iter()
            .map(|(process_id, _)| *process_id)
            .collect::<Vec<_>>(),
        [101, 102]
    );
    assert_eq!(
        captured_snapshots
            .snapshots
            .iter()
            .map(|snapshot| snapshot.pid)
            .collect::<Vec<_>>(),
        [101, 102]
    );
    Ok(())
}

#[test]
fn windows_job_member_without_immediate_metadata_fails_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let error = match super::capture_windows_job_process_snapshots(
        100,
        &[100, 101],
        |process_id| Ok(super::WindowsJobMemberCapture::Captured(process_id)),
        |_process_id, _identity| Ok(super::WindowsJobMemberMetadata::Unavailable),
        || Ok(vec![101]),
    ) {
        Ok(_) => panic!("missing child metadata must fail closed"),
        Err(error) => error,
    };
    assert!(
        error
            .to_string()
            .contains("cannot inspect captured Windows job child PID 101")
    );
    Ok(())
}

#[test]
fn windows_job_member_that_exits_during_metadata_capture_is_skipped()
-> Result<(), Box<dyn std::error::Error>> {
    let captured_snapshots = super::capture_windows_job_process_snapshots(
        100,
        &[100, 101, 102],
        |process_id| Ok(super::WindowsJobMemberCapture::Captured(process_id)),
        |process_id, _identity| {
            if process_id == 101 {
                return Ok(super::WindowsJobMemberMetadata::Exited);
            }
            Ok(super::WindowsJobMemberMetadata::Available(
                super::WindowsProcessSnapshot {
                    pid: process_id,
                    parent_pid: None,
                    creation_time: None,
                    image_name: "node.exe".to_owned(),
                    command: vec![std::ffi::OsString::from("node.exe")],
                },
            ))
        },
        || Ok(vec![102]),
    )?;

    // Only the confirmed-live member is sent to policy inspection.
    // Chỉ thành viên được xác nhận còn sống mới được đưa vào bước kiểm tra chính sách.
    assert_eq!(
        captured_snapshots
            .members
            .iter()
            .map(|(pid, _)| *pid)
            .collect::<Vec<_>>(),
        [102]
    );
    assert_eq!(
        captured_snapshots
            .snapshots
            .iter()
            .map(|snapshot| snapshot.pid)
            .collect::<Vec<_>>(),
        [102]
    );
    Ok(())
}

#[test]
fn windows_job_member_that_exits_before_capture_is_skipped()
-> Result<(), Box<dyn std::error::Error>> {
    let events = std::cell::RefCell::new(Vec::new());
    let captured_snapshots = super::capture_windows_job_process_snapshots(
        100,
        &[100, 101, 102],
        |process_id| {
            events.borrow_mut().push(format!("capture:{process_id}"));
            if process_id == 101 {
                return Ok(super::WindowsJobMemberCapture::Exited);
            }
            Ok(super::WindowsJobMemberCapture::Captured(process_id))
        },
        |process_id, _identity| {
            events.borrow_mut().push(format!("snapshot:{process_id}"));
            Ok(super::WindowsJobMemberMetadata::Available(
                super::WindowsProcessSnapshot {
                    pid: process_id,
                    parent_pid: None,
                    creation_time: None,
                    image_name: "node.exe".to_owned(),
                    command: vec![std::ffi::OsString::from("node.exe")],
                },
            ))
        },
        || Ok(vec![102]),
    )?;

    // A member that left the active Job before opening has no live metadata to inspect.
    // Thành viên đã rời Job trước khi mở không còn metadata tiến trình sống để kiểm tra.
    assert_eq!(
        *events.borrow(),
        ["capture:101", "capture:102", "snapshot:102"]
    );
    assert_eq!(
        captured_snapshots
            .members
            .iter()
            .map(|(pid, _)| *pid)
            .collect::<Vec<_>>(),
        [102]
    );
    Ok(())
}

#[test]
fn windows_job_snapshot_reenumerates_descendants_after_a_member_exits()
-> Result<(), Box<dyn std::error::Error>> {
    let refreshed =
        std::cell::RefCell::new(std::collections::VecDeque::from([vec![102], vec![102]]));
    let captured_snapshots = super::capture_windows_job_process_snapshots(
        100,
        &[100, 101],
        |process_id| {
            if process_id == 101 {
                return Ok(super::WindowsJobMemberCapture::Exited);
            }
            Ok(super::WindowsJobMemberCapture::Captured(process_id))
        },
        |process_id, _identity| {
            Ok(super::WindowsJobMemberMetadata::Available(
                super::WindowsProcessSnapshot {
                    pid: process_id,
                    parent_pid: Some(101),
                    creation_time: None,
                    image_name: "npm.exe".to_owned(),
                    command: vec![
                        std::ffi::OsString::from("npm.exe"),
                        std::ffi::OsString::from("install"),
                    ],
                },
            ))
        },
        || {
            Ok(refreshed
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| vec![102]))
        },
    )?;

    // Re-enumeration finds a new Job member whose parent exited after the first PID snapshot.
    // Liệt kê lại phát hiện thành viên Job mới có parent thoát sau snapshot PID đầu tiên.
    assert_eq!(
        captured_snapshots
            .snapshots
            .iter()
            .map(|snapshot| snapshot.pid)
            .collect::<Vec<_>>(),
        [102]
    );
    assert_eq!(captured_snapshots.snapshots[0].image_name, "npm.exe");
    Ok(())
}

#[test]
fn windows_job_snapshot_reenumerates_members_after_a_clean_first_pass()
-> Result<(), Box<dyn std::error::Error>> {
    let refreshed = std::cell::RefCell::new(std::collections::VecDeque::from([
        vec![101, 102],
        vec![101, 102],
    ]));
    let captured_snapshots = super::capture_windows_job_process_snapshots(
        100,
        &[100, 101],
        |process_id| Ok(super::WindowsJobMemberCapture::Captured(process_id)),
        |process_id, _identity| {
            Ok(super::WindowsJobMemberMetadata::Available(
                super::WindowsProcessSnapshot {
                    pid: process_id,
                    parent_pid: Some(100),
                    creation_time: None,
                    image_name: "node.exe".to_owned(),
                    command: vec![std::ffi::OsString::from("node.exe")],
                },
            ))
        },
        || {
            Ok(refreshed
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| vec![101, 102]))
        },
    )?;

    assert_eq!(
        captured_snapshots
            .snapshots
            .iter()
            .map(|snapshot| snapshot.pid)
            .collect::<Vec<_>>(),
        [101, 102]
    );
    Ok(())
}

#[test]
fn windows_job_snapshot_fails_closed_when_membership_never_stabilizes()
-> Result<(), Box<dyn std::error::Error>> {
    let mut next_pid = 102;
    let error = match super::capture_windows_job_process_snapshots(
        100,
        &[100, 101],
        |_process_id| Ok(super::WindowsJobMemberCapture::<()>::Exited),
        |_process_id, _identity| unreachable!(),
        || {
            let pid = next_pid;
            next_pid += 1;
            Ok(vec![pid])
        },
    ) {
        Ok(_) => panic!("a continuously changing Job must fail closed"),
        Err(error) => error,
    };

    assert!(error.to_string().contains("membership kept changing"));
    Ok(())
}

#[test]
fn windows_job_completion_rejects_a_descendant_seen_after_the_snapshot() {
    assert!(!super::windows_job_has_active_descendants(100, &[100]));
    assert!(super::windows_job_has_active_descendants(100, &[100, 101]));
}

#[test]
fn windows_job_snapshot_scans_a_captured_child_that_exits_after_membership_check()
-> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::OsString;

    let captured = super::capture_windows_job_members(100, &[100, 101], |_| Ok(()))?;
    let captured_pids = captured
        .iter()
        .map(|(process_id, _)| *process_id)
        .collect::<Vec<_>>();
    let process_snapshot = [super::WindowsProcessSnapshot {
        pid: 101,
        parent_pid: Some(100),
        creation_time: Some(101),
        image_name: "npm.exe".to_owned(),
        command: vec![OsString::from("npm.exe"), OsString::from("install")],
    }];

    // Captured identity evidence remains authoritative after the process leaves the job.
    // Bằng chứng danh tính đã chụp vẫn có hiệu lực sau khi process rời Job.
    let inspection = super::inspect_windows_job_processes_with_shadow(
        100,
        &captured_pids,
        &process_snapshot,
        &[],
        None,
    )?;
    assert_eq!(inspection.descendants, [(101, Some("npm".to_owned()))]);
    Ok(())
}

#[test]
fn windows_job_scan_fails_closed_for_captured_child_missing_snapshot()
-> Result<(), Box<dyn std::error::Error>> {
    let captured = super::capture_windows_job_members(100, &[100, 101], |_| Ok(()))?;
    let captured_pids = captured
        .iter()
        .map(|(process_id, _)| *process_id)
        .collect::<Vec<_>>();
    let error =
        match super::inspect_windows_job_processes_with_shadow(100, &captured_pids, &[], &[], None)
        {
            Ok(_) => panic!("a captured Job Object member without metadata must fail closed"),
            Err(error) => error,
        };
    assert!(
        error
            .to_string()
            .contains("cannot inspect captured Windows job child PID 101")
    );
    Ok(())
}

#[test]
fn windows_metadata_retry_rewalks_new_descendants() {
    use std::collections::VecDeque;
    use std::ffi::OsString;

    let snapshot = |pid, parent_pid, creation_time, image_name: &str, command: &[&str]| {
        super::WindowsProcessSnapshot {
            pid,
            parent_pid,
            creation_time: Some(creation_time),
            image_name: image_name.to_owned(),
            command: command.iter().map(OsString::from).collect(),
        }
    };
    let snapshots = VecDeque::from([
        vec![
            snapshot(100, None, 1, "runner.exe", &["runner.exe"]),
            snapshot(101, Some(100), 2, "node.exe", &[]),
        ],
        vec![
            snapshot(100, None, 1, "runner.exe", &["runner.exe"]),
            snapshot(102, Some(101), 3, "npm.exe", &["npm.exe", "install"]),
        ],
    ]);
    let mut snapshots = snapshots;
    let mut inspections = 0;
    let mut ancestry = super::WindowsProcessAncestry::default();

    let found = super::retry_windows_process_scan(
        std::time::Instant::now(),
        None,
        || Ok(false),
        || {
            inspections += 1;
            let snapshot = snapshots.pop_front().expect("retry must inspect again");
            super::scan_windows_process_snapshot(100, &mut ancestry, &snapshot, &[], |_| Ok(None))
        },
    )
    .expect("process snapshot inspection should succeed");

    assert_eq!(inspections, 2, "the retry must refresh and rescan the tree");
    let super::WindowsProcessScan::Forbidden(found) = found else {
        panic!("npm below an exited launcher must still be rejected");
    };
    assert_eq!(found.pid, 102);
    assert_eq!(found.name, "npm");
}

#[test]
fn windows_process_scan_keeps_ancestry_across_clean_snapshots() {
    use std::ffi::OsString;

    let mut ancestry = super::WindowsProcessAncestry::default();
    let first_snapshot = [
        super::WindowsProcessSnapshot {
            pid: 100,
            parent_pid: None,
            creation_time: Some(1),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 101,
            parent_pid: Some(100),
            creation_time: Some(2),
            image_name: "node.exe".to_owned(),
            command: vec![OsString::from("node.exe"), OsString::from("script.js")],
        },
    ];
    assert!(matches!(
        super::scan_windows_process_snapshot(100, &mut ancestry, &first_snapshot, &[], |_| {
            Ok(None)
        })
        .expect("initial tree scan should succeed"),
        super::WindowsProcessScan::Clean
    ));

    let second_snapshot = [
        super::WindowsProcessSnapshot {
            pid: 100,
            parent_pid: None,
            creation_time: Some(1),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 102,
            parent_pid: Some(101),
            creation_time: Some(3),
            image_name: "npm.exe".to_owned(),
            command: vec![OsString::from("npm.exe"), OsString::from("install")],
        },
    ];
    let result =
        super::scan_windows_process_snapshot(100, &mut ancestry, &second_snapshot, &[], |_| {
            Ok(None)
        })
        .expect("detached descendant scan should succeed");

    let super::WindowsProcessScan::Forbidden(found) = result else {
        panic!("npm below a previously observed launcher must still be rejected");
    };
    assert_eq!(found.pid, 102);
    assert_eq!(found.name, "npm");
}

#[test]
fn windows_process_scan_fails_closed_on_child_below_reused_pid() {
    use std::ffi::OsString;

    let snapshot = |pid, parent_pid, creation_time, image_name: &str, command: &[&str]| {
        super::WindowsProcessSnapshot {
            pid,
            parent_pid,
            creation_time: Some(creation_time),
            image_name: image_name.to_owned(),
            command: command.iter().map(OsString::from).collect(),
        }
    };
    let first_snapshot = vec![
        snapshot(100, None, 1, "runner.exe", &["runner.exe"]),
        snapshot(101, Some(100), 2, "node.exe", &["node.exe", "script.js"]),
    ];
    let second_snapshot = vec![
        snapshot(100, None, 1, "runner.exe", &["runner.exe"]),
        snapshot(101, Some(999), 10, "unrelated.exe", &["unrelated.exe"]),
        snapshot(102, Some(101), 11, "npm.exe", &["npm.exe", "install"]),
    ];
    let mut ancestry = super::WindowsProcessAncestry::default();
    assert!(matches!(
        super::scan_windows_process_snapshot(100, &mut ancestry, &first_snapshot, &[], |_| Ok(
            None
        ),)
        .expect("initial ancestry scan should succeed"),
        super::WindowsProcessScan::Clean
    ));

    let result = super::retry_windows_process_scan(
        std::time::Instant::now(),
        None,
        || Ok(false),
        || {
            super::scan_windows_process_snapshot(100, &mut ancestry, &second_snapshot, &[], |_| {
                Ok(None)
            })
        },
    );

    let error = match result {
        Ok(_) => panic!("a child below a reused PID must fail closed"),
        Err(error) => error.to_string(),
    };
    assert!(error.contains("ambiguous after PID reuse"), "{error}");
    assert!(!error.contains("forbidden child process"), "{error}");
}

#[test]
fn windows_process_scan_ignores_unrelated_process_after_pid_reuse() {
    use std::ffi::OsString;

    let mut ancestry = super::WindowsProcessAncestry::default();
    let first_snapshot = [
        super::WindowsProcessSnapshot {
            pid: 100,
            parent_pid: None,
            creation_time: Some(1),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 101,
            parent_pid: Some(100),
            creation_time: Some(2),
            image_name: "node.exe".to_owned(),
            command: vec![OsString::from("node.exe"), OsString::from("script.js")],
        },
    ];
    super::scan_windows_process_snapshot(100, &mut ancestry, &first_snapshot, &[], |_| Ok(None))
        .expect("initial ancestry scan should succeed");

    let second_snapshot = [
        super::WindowsProcessSnapshot {
            pid: 100,
            parent_pid: None,
            creation_time: Some(1),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 101,
            parent_pid: Some(999),
            creation_time: Some(10),
            image_name: "npm.exe".to_owned(),
            command: vec![OsString::from("npm.exe"), OsString::from("install")],
        },
    ];

    assert!(matches!(
        super::scan_windows_process_snapshot(100, &mut ancestry, &second_snapshot, &[], |_| Ok(
            None
        ),)
        .expect("unrelated PID reuse without descendants should remain clean"),
        super::WindowsProcessScan::Clean
    ));
}

#[test]
fn windows_process_scan_retries_transient_creation_time_unavailability() {
    use std::cell::Cell;
    use std::ffi::OsString;

    let snapshot = [
        super::WindowsProcessSnapshot {
            pid: 100,
            parent_pid: None,
            creation_time: Some(1),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 101,
            parent_pid: Some(100),
            creation_time: None,
            image_name: "node.exe".to_owned(),
            command: vec![OsString::from("node.exe"), OsString::from("script.js")],
        },
    ];
    let mut ancestry = super::WindowsProcessAncestry::default();
    let inspections = Cell::new(0);
    let identity_queries = Cell::new(0);

    let result = super::retry_windows_process_scan(
        std::time::Instant::now(),
        None,
        || Ok(false),
        || {
            inspections.set(inspections.get() + 1);
            super::scan_windows_process_snapshot(100, &mut ancestry, &snapshot, &[], |_| {
                let query = identity_queries.get() + 1;
                identity_queries.set(query);
                Ok((query > 1).then_some(2))
            })
        },
    )
    .expect("transient process creation-time lookup should recover");

    assert!(matches!(result, super::WindowsProcessScan::Clean));
    assert_eq!(
        inspections.get(),
        2,
        "retry must refresh the full process tree"
    );
    assert_eq!(
        identity_queries.get(),
        2,
        "retry must query process identity again"
    );
}

#[test]
fn windows_metadata_retry_stops_when_child_exits() {
    use std::cell::Cell;

    let running_checks = Cell::new(0);
    let inspections = Cell::new(0);
    let result = super::retry_windows_process_scan(
        std::time::Instant::now(),
        None,
        || {
            let check = running_checks.get() + 1;
            running_checks.set(check);
            Ok(check >= 3)
        },
        || {
            inspections.set(inspections.get() + 1);
            Ok(super::WindowsProcessScan::MissingCommandLine {
                pid: 101,
                image_name: "node.exe".to_owned(),
            })
        },
    )
    .expect("finished child should end retries cleanly");

    assert!(matches!(result, super::WindowsProcessScan::ChildExited));
    assert_eq!(inspections.get(), 1, "do not refresh after the child exits");
}

#[test]
fn windows_metadata_retry_observes_child_exit_after_clean_scan() {
    use std::cell::Cell;

    let running_checks = Cell::new(0);
    let result = super::retry_windows_process_scan(
        std::time::Instant::now(),
        None,
        || {
            let check = running_checks.get() + 1;
            running_checks.set(check);
            Ok(check == 2)
        },
        || Ok(super::WindowsProcessScan::Clean),
    )
    .expect("child exit during a clean scan should be observed");

    assert!(matches!(result, super::WindowsProcessScan::ChildExited));
}

#[test]
fn windows_metadata_retry_respects_command_deadline() {
    use std::cell::Cell;

    let inspections = Cell::new(0);
    let result = super::retry_windows_process_scan(
        std::time::Instant::now(),
        Some(std::time::Duration::from_millis(1)),
        || Ok(false),
        || {
            inspections.set(inspections.get() + 1);
            std::thread::sleep(std::time::Duration::from_millis(5));
            Ok(super::WindowsProcessScan::MissingCommandLine {
                pid: 101,
                image_name: "node.exe".to_owned(),
            })
        },
    )
    .expect("slow snapshot must stop retries at the command deadline");

    assert!(matches!(result, super::WindowsProcessScan::DeadlineReached));
    assert_eq!(
        inspections.get(),
        1,
        "do not start another scan after timeout"
    );
}

#[cfg(windows)]
#[test]
fn root_exit_completion_inspects_and_terminates_a_forbidden_job_child()
-> Result<(), Box<dyn std::error::Error>> {
    use super::{ExecOutcome, ProcessTreeGuard, finish_monitored_root_exit};
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    use std::time::Duration;
    use windows_sys::Win32::System::Threading::CREATE_SUSPENDED;

    let temp = tempfile::tempdir()?;
    let comspec = std::env::var_os("COMSPEC").ok_or("Windows must expose COMSPEC")?;
    let forbidden_exe = temp.path().join("npm.exe");
    std::fs::copy(&comspec, &forbidden_exe)?;

    let guard = ProcessTreeGuard::new()?;
    let mut forbidden_command = Command::new(&forbidden_exe);
    forbidden_command
        .args(["/D", "/S", "/C", "ping -n 30 127.0.0.1 > NUL"])
        .creation_flags(CREATE_SUSPENDED);
    let forbidden_child = forbidden_command.spawn()?;
    guard.activate(&forbidden_child)?;
    std::thread::sleep(Duration::from_millis(150));
    let shadow_path = super::ShadowPath {
        dir: temp.path().to_path_buf(),
    };

    let mut root_command = Command::new(comspec);
    root_command
        .args(["/D", "/S", "/C", "exit /b 0"])
        .creation_flags(CREATE_SUSPENDED);
    let mut root = root_command.spawn()?;
    guard.activate(&root)?;
    assert!(
        root.wait()?.success(),
        "test root command must exit successfully"
    );

    let result = finish_monitored_root_exit(
        &mut root,
        guard,
        &[],
        &shadow_path,
        None,
        None,
        |child, _| {
            (
                ExecOutcome {
                    status: child
                        .wait()
                        .expect("root process status must remain readable"),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                },
                true,
            )
        },
    );
    let error = match result {
        Ok(_) => return Err("root-exit completion accepted a forbidden child".into()),
        Err(error) => error.to_string(),
    };
    assert!(
        error.contains("forbidden package manager 'npm' spawned"),
        "root-exit helper did not inspect the surviving job child: {error}"
    );
    Ok(())
}

#[test]
fn observed_go_proxy_is_required_for_readonly_execution_but_never_allows_mutation() {
    use std::ffi::OsString;
    let argv = |args: &[&str]| args.iter().map(OsString::from).collect::<Vec<_>>();
    let off = vec![("GOPROXY".to_string(), "off".to_string())];
    let online = [("GOPROXY".to_string(), "https://proxy.example".to_string())];
    for env in [&[][..], &online[..]] {
        assert!(
            super::forbidden_process_match_argv_with_env(
                "go",
                &argv(&["go", "run", "-mod=readonly", "main.go"]),
                &[],
                env
            )
            .is_some()
        );
    }
    assert!(
        super::forbidden_process_match_argv_with_env(
            "go",
            &argv(&["go", "run", "-mod=readonly", "main.go"]),
            &[],
            &off
        )
        .is_none()
    );
    for args in [
        vec!["go", "get", "example.com/pkg"],
        vec!["go", "run", "main.go"],
        vec!["go", "mod", "download"],
    ] {
        assert!(
            super::forbidden_process_match_argv_with_env("go", &argv(&args), &[], &off).is_some()
        );
    }
    assert!(
        super::forbidden_process_match_argv_with_env("npm", &argv(&["npm", "install"]), &[], &off)
            .is_some()
    );
}

#[test]
fn windows_go_policy_uses_only_observed_member_environment()
-> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    use std::ffi::OsString;
    let snapshot = |pid, image: &str, args: &[&str]| super::WindowsProcessSnapshot {
        pid,
        parent_pid: Some(100),
        creation_time: Some(u64::from(pid)),
        image_name: image.into(),
        command: args.iter().map(OsString::from).collect(),
    };
    let processes = [
        snapshot(
            101,
            "go.exe",
            &["go.exe", "run", "-mod=readonly", "main.go"],
        ),
        snapshot(
            102,
            "go.exe",
            &["go.exe", "run", "-mod=readonly", "main.go"],
        ),
        snapshot(103, "go.exe", &["go.exe", "get", "example.com/evil"]),
        snapshot(104, "npm.exe", &["npm.exe", "install"]),
    ];
    let paths = HashMap::new();
    let off = vec![("GOPROXY".into(), "off".into())];
    for allowed_env in [false, true] {
        let mut environments = HashMap::from([
            (
                102,
                vec![("GOPROXY".into(), "https://proxy.example".into())],
            ),
            (103, off.clone()),
            (104, off.clone()),
            // Root environment must never authorize a child.
            // Môi trường root không được cấp quyền cho child thiếu bằng chứng.
            (100, off.clone()),
        ]);
        if allowed_env {
            environments.insert(101, off.clone());
        }
        let inspection = super::inspect_windows_job_processes_with_policy_metadata(
            100,
            &[100, 101, 102, 103, 104],
            &processes,
            &[],
            None,
            &super::WindowsProcessPolicyMetadata {
                executable_paths: &paths,
                go_policy_env: &environments,
            },
            None,
        )?;
        assert_eq!(
            inspection.descendants,
            vec![
                (101, (!allowed_env).then(|| "go".into())),
                (102, Some("go".into())),
                (103, Some("go".into())),
                (104, Some("npm".into())),
            ]
        );
    }
    Ok(())
}
