//! Lifecycle behavior tests for the matching production owner.
//! Test hành vi lifecycle cho owner production tương ứng.

use super::super::LifecycleRunner;
use super::helpers::write_package_script;

#[test]
#[cfg(unix)]
fn lifecycle_core_identity_tampering_fails_the_install_path() {
    let _environment = super::helpers::environment_test_guard();
    // A lifecycle process can write project files; it must not turn a
    // successful install into a false success after changing core ownership.
    // (Lifecycle có thể ghi file project; thay core phải làm install fail.)
    let project = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::new("web-owner", "web")
        .save(project.path())
        .unwrap();
    let package = tempfile::tempdir().unwrap();
    let marker = project
        .path()
        .join(mgc_config::project::ProjectConfig::CORE_MARKER_FILE);
    let script = format!(
        "python3 -c \"from pathlib import Path; Path('{}').write_text('ai')\"",
        marker.display()
    );
    write_package_script(package.path(), &script);

    let error = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();
    assert!(
        error.to_string().contains("identity") && error.to_string().contains("lifecycle scripts"),
        "identity tampering must fail closed, got: {error}"
    );
    assert_eq!(
        std::fs::read_to_string(&marker).expect("read restored core marker"),
        "web\n",
        "failed lifecycle must restore the pre-install core identity marker"
    );
    assert_eq!(
        mgc_config::project::ProjectConfig::detect_core(project.path())
            .expect("detect core after identity rollback")
            .as_deref(),
        Some("web"),
        "restored project identity must remain usable after install abort"
    );
}

#[test]
#[cfg(unix)]
fn lifecycle_identity_tampering_stops_later_hooks_before_execution() {
    let _environment = super::helpers::environment_test_guard();
    let project = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::new("web-owner", "web")
        .save(project.path())
        .unwrap();
    let package = tempfile::tempdir().unwrap();
    let marker = project
        .path()
        .join(mgc_config::project::ProjectConfig::CORE_MARKER_FILE);
    let later_hook_sentinel = project.path().join("later-hook-ran.txt");
    let preinstall = format!(
        "python3 -c \"from pathlib import Path; Path('{}').write_text('ai')\"",
        marker.display()
    );
    let install = format!(
        "python3 -c \"from pathlib import Path; Path('{}').write_text('ran')\"",
        later_hook_sentinel.display()
    );
    let manifest = serde_json::json!({
        "scripts": {
            "preinstall": preinstall,
            "install": install,
        }
    });
    std::fs::write(package.path().join("package.json"), manifest.to_string()).unwrap();

    let error = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();

    assert!(error.to_string().contains("identity"));
    assert!(
        !later_hook_sentinel.exists(),
        "later lifecycle hooks must not run"
    );
    assert_eq!(std::fs::read_to_string(marker).unwrap(), "web\n");
}

#[test]
#[cfg(unix)]
fn lifecycle_cannot_delete_core_marker_and_rely_on_config_fallback() {
    let _environment = super::helpers::environment_test_guard();
    let project = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::new("web-owner", "web")
        .save(project.path())
        .unwrap();
    let package = tempfile::tempdir().unwrap();
    let marker = project
        .path()
        .join(mgc_config::project::ProjectConfig::CORE_MARKER_FILE);
    let script = format!(
        "python3 -c \"from pathlib import Path; Path('{}').unlink()\"",
        marker.display()
    );
    write_package_script(package.path(), &script);

    let error = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();

    assert!(error.to_string().contains("identity"));
    assert_eq!(
        std::fs::read_to_string(&marker).unwrap(),
        "web\n",
        "a removed ownership marker must be restored even when mgc.toml still identifies Web"
    );
}
