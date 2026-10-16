use crate::commands::run::{resolve_mgc_toml_script, resolve_package_json_script};
use std::path::PathBuf;

fn write(path: PathBuf, content: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

#[test]
fn mgc_toml_script_resolves_by_name() {
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("mgc.toml"),
        "[scripts]\nstart = \"mgc dev --port 3000\"\ntest = \"cargo test\"\n",
    );
    let resolved = resolve_mgc_toml_script(&dir.path().join("mgc.toml"), "test").unwrap();
    assert_eq!(resolved.as_deref(), Some("cargo test"));
    assert_eq!(
        resolve_mgc_toml_script(&dir.path().join("mgc.toml"), "missing").unwrap(),
        None
    );
}

#[test]
fn package_json_script_resolves_by_name() {
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts": {"build": "vite build", "dev": "vite --port 5173"}}"#,
    );
    let resolved = resolve_package_json_script(&dir.path().join("package.json"), "build").unwrap();
    assert_eq!(resolved.as_deref(), Some("vite build"));
    assert_eq!(
        resolve_package_json_script(&dir.path().join("package.json"), "nope").unwrap(),
        None
    );
}

#[test]
fn mgc_toml_script_has_priority_source() {
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("mgc.toml"),
        "[scripts]\nrun = \"echo mgc-toml-wins\"\n",
    );
    write(
        dir.path().join("package.json"),
        r#"{"scripts": {"run": "echo package-json-loses"}}"#,
    );
    let mgc = resolve_mgc_toml_script(&dir.path().join("mgc.toml"), "run").unwrap();
    let pkg = resolve_package_json_script(&dir.path().join("package.json"), "run").unwrap();
    assert_eq!(mgc.as_deref(), Some("echo mgc-toml-wins"));
    assert_eq!(pkg.as_deref(), Some("echo package-json-loses"));
}

#[test]
fn task_script_source_precedence_mgc_toml_wins() {
    use crate::commands::run::resolve_task_script;
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("mgc.toml"),
        "[scripts]\ngo = \"cargo --version\"\n",
    );
    write(
        dir.path().join("package.json"),
        r#"{"scripts": {"go": "cargo --bad-flag-xyz"}}"#,
    );
    let (cmd, from_mgc_toml) = resolve_task_script(dir.path(), "go").unwrap().unwrap();
    assert_eq!(cmd, "cargo --version");
    assert!(from_mgc_toml);
}

#[test]
fn pre_hook_failure_aborts_chain() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_with_hooks;
    // pre fails (`cargo --bad-flag-xyz` exits non-zero) while main would
    // succeed — Err proves main never ran.
    // (pre hỏng trong khi main sẽ thành công — Err chứng minh main không chạy.)
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts": {"prego": "cargo --bad-flag-xyz", "go": "cargo --version"}}"#,
    );
    let compat = CompatMode::Native;
    let err = run_with_hooks(dir.path(), "go", &[], &compat, None, "run").unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[test]
fn post_hook_runs_on_success() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_with_hooks;
    // Main succeeds, post fails — Err proves post ran.
    // (Main thành công, post hỏng — Err chứng minh post đã chạy.)
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts": {"go": "cargo --version", "postgo": "cargo --bad-flag-xyz"}}"#,
    );
    let compat = CompatMode::Native;
    let err = run_with_hooks(dir.path(), "go", &[], &compat, None, "run").unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[test]
fn main_failure_wins_over_post_failure() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_with_hooks;
    // Both main and post fail — the reported error must be main's
    // (post failure is only warned). Distinctive commands tell them apart.
    // (Cả main và post đều hỏng — lỗi báo phải là của main (lỗi post chỉ
    // cảnh báo). Lệnh khác nhau phân biệt chúng.)
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts": {"go": "npm install left-pad-marker", "postgo": "cargo --bad-post-xyz"}}"#,
    );
    let compat = CompatMode::Native;
    let err = run_with_hooks(dir.path(), "go", &[], &compat, None, "run").unwrap_err();
    assert!(
        err.to_string().contains("left-pad-marker"),
        "expected main failure, got: {err}"
    );
}

#[cfg(unix)]
#[test]
fn explicit_compat_runner_reaches_executor_from_package_script() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_task_script;
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("node_modules/.bin");
    std::fs::create_dir_all(&bin).unwrap();
    let npm = bin.join("npm");
    std::fs::write(&npm, "#!/bin/sh\nexit 0\n").unwrap();
    let mut permissions = std::fs::metadata(&npm).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&npm, permissions).unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts":{"dev":"npm run dev"}}"#,
    );

    run_task_script(
        dir.path(),
        "dev",
        &[],
        &CompatMode::Native,
        Some("npm"),
        "run",
    )
    .unwrap();
}

#[test]
fn native_script_still_rejects_package_manager_without_compat() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_task_script;

    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts":{"dev":"npm run dev"}}"#,
    );
    let err =
        run_task_script(dir.path(), "dev", &[], &CompatMode::Native, None, "run").unwrap_err();
    assert!(
        err.to_string()
            .contains("delegates to forbidden package manager"),
        "got: {err}"
    );
}
