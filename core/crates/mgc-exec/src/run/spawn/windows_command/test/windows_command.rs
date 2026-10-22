// Windows command lookup, path and batch validation tests.
// Test dò lệnh Windows, chuẩn hóa path và kiểm tra batch.

use crate::run::implementation::spawn::windows_command::error::WindowsCommandError;
use super::*;

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
        assert_eq!(windows_tool_path_units(&input), expected);
    }
    let mut units = r"\\?\C:\project\".encode_utf16().collect::<Vec<_>>();
    units.push(0xD800);
    assert_eq!(windows_tool_path_units(&units), units[4..]);
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

    assert_eq!(
        find_windows_command("tool", Some(&search_path)),
        Some(system_bin.join("tool.exe").into_os_string())
    );
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

    assert_eq!(
        find_windows_command("tool", Some(&search_path)),
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

    assert_eq!(find_windows_command("tool", Some(&search_path)), None);
    Ok(())
}

#[test]
fn windows_batch_invocation_accepts_regular_paths_and_arguments() {
    let args = vec!["--output".to_string(), "release bundle.zip".to_string()];
    assert!(validate_windows_batch_invocation(r"C:\Program Files\MagiCore\tool.cmd", &args).is_ok());
}

#[test]
fn windows_batch_invocation_rejects_cmd_expansion_and_operators() {
    for character in ['"', '%', '!', '&', '|', '<', '>', '^', '(', ')', '\n', '\r'] {
        let argument = format!("value{character}payload");
        assert_eq!(
            validate_windows_batch_invocation(
                r"C:\tools\tool.cmd",
                std::slice::from_ref(&argument)
            ),
            Err(WindowsCommandError::UnsupportedBatchSyntax),
            "argument containing {character:?} must be rejected"
        );

        let path = format!(r"C:\tools\bad{character}name.cmd");
        assert_eq!(
            validate_windows_batch_invocation(&path, &[]),
            Err(WindowsCommandError::UnsupportedBatchSyntax),
            "script path containing {character:?} must be rejected"
        );
    }
}
