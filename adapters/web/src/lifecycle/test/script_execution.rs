//! Lifecycle behavior tests for the matching production owner.
//! Test hành vi lifecycle cho owner production tương ứng.

use super::super::{LifecycleRunner, load_package_scripts};
use super::helpers::write_package_script;

#[test]
#[cfg(unix)]
fn lifecycle_runs_simple_script_without_shell() {
    let _environment = super::helpers::environment_test_guard();
    let project = tempfile::tempdir().unwrap();
    let package = tempfile::tempdir().unwrap();
    let marker = package.path().join("marker.txt");
    write_package_script(
        package.path(),
        "python3 -c \"from pathlib import Path; Path('marker.txt').write_text('ok')\"",
    );

    LifecycleRunner::run_scripts(package.path(), project.path()).unwrap();
    assert!(marker.exists());
}

#[test]
#[cfg(unix)]
fn lifecycle_accepts_leading_env_assignment() {
    let _environment = super::helpers::environment_test_guard();
    let project = tempfile::tempdir().unwrap();
    let package = tempfile::tempdir().unwrap();
    write_package_script(
        package.path(),
        "MGC_LIFECYCLE_TEST=ok python3 -c \"import os; assert os.environ.get('MGC_LIFECYCLE_TEST') == 'ok'\"",
    );

    LifecycleRunner::run_scripts(package.path(), project.path()).unwrap();
}

#[test]
#[cfg(unix)]
fn lifecycle_execution_uses_the_policy_checked_manifest_snapshot() {
    let _environment = super::helpers::environment_test_guard();
    let project = tempfile::tempdir().unwrap();
    let package = tempfile::tempdir().unwrap();
    let marker = package.path().join("snapshot-marker.txt");
    let script = format!(
        "python3 -c \"from pathlib import Path; Path('{}').write_text('checked')\"",
        marker.display()
    );
    write_package_script(package.path(), &script);

    let checked = load_package_scripts(package.path()).unwrap();
    write_package_script(package.path(), "npm install attacker-controlled");

    LifecycleRunner::run_scripts_with_snapshot(package.path(), project.path(), checked).unwrap();
    assert_eq!(std::fs::read_to_string(marker).unwrap(), "checked");
}

#[test]
#[cfg(unix)]
fn lifecycle_timeout_kills_hung_process() {
    let project = tempfile::tempdir().unwrap();
    let package = tempfile::tempdir().unwrap();
    write_package_script(package.path(), "python3 -c \"import time; time.sleep(2)\"");

    let _timeout =
        super::helpers::ScopedEnvironmentVariable::set("MGC_LIFECYCLE_TIMEOUT_SECS", "1");
    let err = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();
    assert!(err.to_string().contains("timed out"));
}

#[test]
fn lifecycle_failing_script_is_an_error_not_a_warning() {
    let _environment = super::helpers::environment_test_guard();
    // A lifecycle script that RUNS but exits non-zero must fail the
    // caller: install treats this Err as fatal (npm parity — a red
    // postinstall is never a green install).
    // (Script chạy nhưng exit lỗi phải là Err.)
    let project = tempfile::tempdir().unwrap();
    let package = tempfile::tempdir().unwrap();
    write_package_script(package.path(), "node -e \"process.exit(1)\"");
    let err = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();
    assert!(
        err.to_string().contains("failed"),
        "unexpected error: {err}"
    );
}
