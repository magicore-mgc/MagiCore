//! Focused execute package regression tests.
//! Test hồi quy tập trung cho execute package.

use super::*;

#[test]
fn native_package_path_puts_verified_bin_before_host_path() {
    let bin = std::path::PathBuf::from("/mgc/project/node_modules/.bin");
    let host = std::env::join_paths(["/host/bin", "/usr/bin"]).expect("host path");
    let actual = runner_path(&bin, &host, false).expect("construct native bin path");
    assert_eq!(
        std::env::split_paths(&actual).collect::<Vec<_>>(),
        [
            bin,
            std::path::PathBuf::from("/host/bin"),
            std::path::PathBuf::from("/usr/bin")
        ]
    );
}

#[test]
fn node_path_preserves_host_entries_before_project_bin() {
    let bin = std::path::PathBuf::from("/mgc/project/node_modules/.bin");
    let host = std::env::join_paths(["/host/bin", "/usr/bin"]).expect("host path");
    let actual = runner_path(&bin, &host, true).expect("construct Node path");
    assert_eq!(
        std::env::split_paths(&actual).collect::<Vec<_>>(),
        [
            std::path::PathBuf::from("/host/bin"),
            std::path::PathBuf::from("/usr/bin"),
            bin
        ]
    );
}
