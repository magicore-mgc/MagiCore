//! Test Flutter executable lookup behavior without executing it.
//! Test cách dò executable Flutter mà không chạy executable.

#[test]
fn flutter_sdk_path_lookup_is_filesystem_only_and_honors_pathext() {
    let dir = tempfile::tempdir().unwrap();
    let executable = dir.path().join("flutter.CMD");
    std::fs::write(&executable, "@echo off\r\n").unwrap();

    let found = crate::manifest::flutter::find_flutter_executable_in_path(
        dir.path().as_os_str(),
        Some(std::ffi::OsStr::new(".COM;.EXE;.CMD;.BAT")),
        true,
    );

    assert_eq!(found, Some(executable.canonicalize().unwrap()));
}

#[test]
fn flutter_sdk_path_lookup_does_not_accept_missing_or_directory_candidates() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("flutter")).unwrap();

    let found = crate::manifest::flutter::find_flutter_executable_in_path(
        dir.path().as_os_str(),
        None,
        false,
    );

    assert_eq!(found, None);
}
