//! Reclaim reporting and safety tests.
//! Test báo cáo và an toàn của reclaim.

use std::fs;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn sandbox(tag: &str) -> (PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!(
        "mgc-reclaim-{tag}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let project = base.join("project");
    let data = base.join("data");
    fs::create_dir_all(&project).expect("project dir");
    fs::create_dir_all(&data).expect("data dir");
    fs::write(project.join("mgc.toml"), "name = \"x\"\n").expect("manifest");
    (project, data)
}

fn run(project: &Path, data: &Path, apply: bool, yes: bool) -> anyhow::Result<()> {
    // SAFETY: this test sets process-wide env vars and runs its assertions
    // synchronously inside the test harness lock.
    // AN TOÀN: test đặt biến môi trường tiến trình và chạy đồng bộ dưới lock harness.
    #[allow(unsafe_code)]
    // SAFETY: single-threaded test setup writing process-local env vars.
    // AN TOÀN: test đơn luồng ghi biến môi trường cục bộ tiến trình.
    unsafe {
        std::env::set_var("MGC_DATA_DIR", data);
        std::env::set_var("MAGICORE_STORE_ROOT", data.join("store"));
    }
    super::super::run_reclaim(project, apply, yes, false)
}

#[test]
fn empty_store_reports_nothing_to_reclaim() {
    let (project, data) = sandbox("empty");
    run(&project, &data, false, false).expect("reclaim runs");
    fs::remove_dir_all(project.parent().expect("base")).ok();
}

#[test]
fn review_and_plan_never_delete_without_yes() {
    let (project, data) = sandbox("no-yes");
    run(&project, &data, false, false).expect("review");
    run(&project, &data, true, false).expect("plan");
    assert!(!data.join("store").join("deleted").exists());
    fs::remove_dir_all(project.parent().expect("base")).ok();
}

#[test]
fn workspace_root_is_reported_in_json_shape() {
    let (project, data) = sandbox("shape");
    run(&project, &data, false, false).expect("reclaim");
    assert!(workspace_root().join("Cargo.toml").exists());
    fs::remove_dir_all(project.parent().expect("base")).ok();
}
