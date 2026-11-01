//! Windows native process-table snapshot integration test.
//! Kiểm thử tích hợp snapshot process table native Windows.

#[cfg(windows)]
use anyhow::Result;
// `WindowsProcessScan` is re-exported by `run::implementation` itself, not by this
// test module's parent, which only forwards the ancestry type and the inspector.
// (`WindowsProcessScan` được re-export ở chính `run::implementation`, không phải
// module cha của test này — module cha chỉ chuyển tiếp type ancestry và inspector.)
#[cfg(windows)]
use crate::run::implementation::WindowsProcessScan;

#[cfg(windows)]
#[test]
fn windows_native_inspector_detects_forbidden_descendant() -> Result<(), Box<dyn std::error::Error>>
{
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let directory = tempfile::tempdir()?;
    let forbidden = directory.path().join("npm.exe");
    // A hard link exposes the real Windows image name and executable bytes.
    // A copied executable would correctly fail image-identity authentication.
    std::fs::hard_link(
        std::env::current_exe().expect("resolve Windows runner image"),
        &forbidden,
    )?;
    let mut child = std::process::Command::new(&forbidden)
        .args(["/D", "/C", "ping -n 10 127.0.0.1 > NUL"])
        .creation_flags(CREATE_NO_WINDOW)
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
    let taskkill = crate::run::implementation::spawn::windows_command::windows_system_tool_path(
        "taskkill.exe",
    )?;
    let _ = std::process::Command::new(taskkill)
        .args(["/F", "/T", "/PID", &child.id().to_string()])
        .status();
    let _ = child.kill();
    let _ = child.wait();
    let WindowsProcessScan::Forbidden(found) = found? else {
        return Err("native Windows inspector must detect npm.exe".into());
    };
    assert_eq!(found.name, "npm");
    Ok(())
}
