use super::*;

#[cfg(windows)]
#[test]
fn root_exit_completion_inspects_and_terminates_a_forbidden_job_child()
-> Result<(), Box<dyn std::error::Error>> {
    use super::super::super::super::ShadowPath;
    use super::super::super::super::output::ExecOutcome;
    use super::super::super::ProcessTreeGuard;
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
    let shadow_path = ShadowPath {
        dir: temp.path().to_path_buf(),
        cleaned: true,
        cleanup_attempted: true,
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
        crate::allowlist::ExecutionScope::Install,
        |child, _| {
            Ok((
                ExecOutcome {
                    status: child
                        .wait()
                        .expect("root process status must remain readable"),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                },
                true,
            ))
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
