// Python package-manager and build isolation tests.
// Test package manager và build isolation Python.

use super::*;

#[test]
fn package_manager_modules_and_ensurepip_are_denied_for_both_launchers() {
    for tool in ["python", "py"] {
        assert!(denial_reason(tool, &["-m".into(), "pip._internal".into()]).is_some());
        assert!(denial_reason(tool, &["-m".into(), "ensurepip".into()]).is_some());
    }
}

#[test]
fn python_build_frontend_requires_no_isolation() {
    assert!(denial_reason("python", &["-m".into(), "build".into()]).is_some());
    assert!(denial_reason("python", &["-m".into(), "build".into(), "--no-isolation".into()]).is_none());
    assert!(denial_reason("py", &["-m".into(), "build".into()]).is_none());
}
