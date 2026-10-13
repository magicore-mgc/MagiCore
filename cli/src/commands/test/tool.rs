#![allow(clippy::unwrap_used)]

use super::render_windows_shim_contents;
use std::path::Path;

#[test]
fn windows_node_shim_invokes_the_declared_package_target_directly() {
    let target = Path::new(r"C:\MagiCore\tools\package\bin\cli");
    let rendered = render_windows_shim_contents(target).unwrap();

    assert_eq!(
        rendered,
        "@echo off\r\nnode \"C:\\MagiCore\\tools\\package\\bin\\cli\" %*\r\n"
    );
    assert!(!rendered.contains("node_modules\\.bin"));
    assert!(!rendered.contains(".cmd"));
}

#[test]
fn windows_native_executable_shim_does_not_require_node() {
    let target = Path::new(r"C:\MagiCore\tools\package\bin\cli.exe");
    let rendered = render_windows_shim_contents(target).unwrap();

    assert_eq!(
        rendered,
        "@echo off\r\n\"C:\\MagiCore\\tools\\package\\bin\\cli.exe\" %*\r\n"
    );
}

#[test]
fn windows_shim_rejects_percent_expansion_and_control_characters() {
    assert!(render_windows_shim_contents(Path::new(r"C:\tools\bad%PATH%\cli")).is_err());
    assert!(render_windows_shim_contents(Path::new("C:\\tools\\bad\npath\\cli")).is_err());
}
