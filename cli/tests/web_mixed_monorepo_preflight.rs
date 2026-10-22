//! Verify mixed web scaffolds fail before mutating a supported frontend lane.
//! (Kiểm tra scaffold web hỗn hợp bị từ chối trước khi sửa lane frontend.)

use std::process::{Command, Output};
use std::time::Duration;
use tempfile::TempDir;

fn mgc_binary() -> String {
    std::env::var("CARGO_BIN_EXE_mgc").expect("CARGO_BIN_EXE_mgc must be set by cargo test")
}

fn run_mgc(args: &[&str], cwd: &std::path::Path, isolated: &TempDir) -> Output {
    let data = isolated.path().join("system-data");
    let cache = isolated.path().join("cache");
    let templates = isolated.path().join("templates");
    std::fs::create_dir_all(&data).expect("create isolated system-data directory");
    std::fs::create_dir_all(&cache).expect("create isolated cache directory");
    std::fs::create_dir_all(&templates).expect("create isolated templates directory");

    Command::new(mgc_binary())
        .args(args)
        .current_dir(cwd)
        .env("HOME", isolated.path())
        .env("USERPROFILE", isolated.path())
        .env("MGC_DATA_DIR", data)
        .env("MGC_CACHE_DIR", cache)
        .env("MGC_TEMPLATES_DIR", templates)
        .output()
        .expect("run mgc command")
}

fn combined_output(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn mixed_web_install_rejects_unsupported_backend_before_frontend_mutation() {
    let temp = TempDir::new().expect("create temporary mixed-stack project");
    let project = temp.path().join("mixed-react-spring");
    let frontend = project.join("apps/frontend");
    let backend = project.join("apps/backend");
    std::fs::create_dir_all(&frontend).expect("create frontend workspace member");
    std::fs::create_dir_all(&backend).expect("create backend workspace member");
    std::fs::write(
        project.join("mgc.toml"),
        "name = \"mixed-react-spring\"\nversion = \"0.1.0\"\necosystem = \"web\"\n",
    )
    .expect("write root project identity");
    std::fs::write(
        project.join("magicore.workspace.toml"),
        "mode = \"monorepo\"\n[layout]\napps_dir = \"apps\"\npackages_dir = \"packages\"\n",
    )
    .expect("write mixed workspace layout");
    std::fs::write(
        frontend.join("package.json"),
        r#"{"name":"frontend","version":"0.1.0","dependencies":{}}"#,
    )
    .expect("write empty JavaScript frontend manifest");
    std::fs::write(
        backend.join("pom.xml"),
        "<project><modelVersion>4.0.0</modelVersion></project>\n",
    )
    .expect("write Java backend manifest");

    let install = run_mgc(&["--core", "web", "install"], &project, &temp);
    let output = combined_output(&install);
    assert!(
        !install.status.success(),
        "unsupported mixed install succeeded"
    );
    assert!(
        output.contains("web monorepo member` install for `java` is unavailable"),
        "install should explain the unsupported backend:\n{output}"
    );
    assert!(!project.join("node_modules").exists());
    assert!(!project.join("mgc.lock").exists());
    assert!(
        !project
            .join(".magicore")
            .join("cache")
            .join("web")
            .join("store.db")
            .exists()
    );
}

#[test]
fn monorepo_install_acquires_each_target_writer_lock_before_empty_graph_return() {
    let isolated = TempDir::new().expect("create temporary workspace");
    let root = isolated.path().join("workspace");
    let frontend = root.join("apps/frontend");
    std::fs::create_dir_all(&frontend).expect("create frontend workspace target");
    std::fs::write(
        root.join("mgc.toml"),
        "name = \"workspace\"\nversion = \"0.1.0\"\necosystem = \"web\"\n",
    )
    .expect("write root project identity");
    std::fs::write(
        root.join("magicore.workspace.toml"),
        "mode = \"monorepo\"\n[layout]\napps_dir = \"apps\"\npackages_dir = \"packages\"\n",
    )
    .expect("write workspace layout");
    std::fs::write(
        frontend.join("mgc.toml"),
        "name = \"frontend\"\nversion = \"0.1.0\"\necosystem = \"web\"\n[lock]\nacquire_timeout_ms = 100\n",
    )
    .expect("write target lock timeout");
    std::fs::write(
        frontend.join("package.json"),
        r#"{"name":"frontend","version":"0.1.0","dependencies":{}}"#,
    )
    .expect("write empty target manifest");

    let held_lock =
        mgc_lockfile::project_lock::ProjectWriteLock::acquire(&frontend, Duration::from_secs(1))
            .expect("hold the target project writer lock");
    let install = run_mgc(&["--core", "web", "install"], &root, &isolated);
    let output = combined_output(&install);
    assert!(
        !install.status.success(),
        "monorepo install bypassed the held target writer lock:\n{output}"
    );
    assert!(
        output.contains("cannot acquire the project writer lock"),
        "expected the project writer lock refusal before install preparation:\n{output}"
    );
    assert!(
        !frontend.join("node_modules").exists(),
        "install created a target directory despite the held writer lock"
    );
    drop(held_lock);
}
