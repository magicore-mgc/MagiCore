//! `create-cicd` offline scaffold end-to-end — scaffold CI/CD offline qua CLI thật.
//!
//! The cicd core publishes no registry layer, so `mgc create-cicd` could only
//! fail with "no scaffold source". These cases drive the real binary with a
//! hermetic HOME/template root: the create must succeed offline, produce a
//! project the other cicd verbs recognise, and leave a pipeline that
//! `mgc ci-generate` rewrites byte-for-byte.
//! Core cicd chưa có registry layer nên `mgc create-cicd` từng chỉ báo "no
//! scaffold source". Các ca này chạy binary thật với HOME/template root cô lập:
//! create phải thành công offline, tạo project mà các lệnh cicd khác nhận ra, và
//! để lại pipeline mà `mgc ci-generate` ghi lại đúng từng byte.

#![allow(clippy::unwrap_used)]

use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::TempDir;

fn mgc_binary() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_mgc"))
}

/// A CLI invocation with an isolated home, cache and template root.
/// Lệnh CLI với home, cache và template root cô lập.
fn hermetic(home: &Path, cwd: &Path) -> Command {
    let mut command = Command::new(mgc_binary());
    command
        .current_dir(cwd)
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env("MGC_DATA_DIR", home.join("data"))
        .env("MGC_CACHE_DIR", home.join("cache"))
        .env("MGC_TEMPLATES_DIR", home.join("templates"))
        .env("MAGICORE_STORE_ROOT", home.join("store"))
        .env_remove("MAGICORE_TEMPLATE_DIR")
        .env_remove("MGC_NPM_REGISTRY");
    command
}

fn run(command: &mut Command) -> (bool, String, String) {
    let output = command.output().expect("spawn mgc");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn create_cicd_github_actions_is_offline_and_regenerable() {
    let home = TempDir::new().unwrap();
    let workspace = TempDir::new().unwrap();
    let (ok, stdout, stderr) = run(hermetic(home.path(), workspace.path()).args([
        "create-cicd",
        "github-actions",
        "ci-demo",
        "--yes",
    ]));
    assert!(ok, "create-cicd failed: {stderr}\n{stdout}");

    let project = workspace.path().join("ci-demo");
    let workflow_path = project.join(".github/workflows/ci.yml");
    assert!(workflow_path.is_file(), "workflow file missing");
    assert!(project.join("README.md").is_file(), "README.md missing");
    assert!(project.join(".gitignore").is_file(), ".gitignore missing");

    // MagiCore-owned metadata must claim the core and record the provider.
    // (Metadata MagiCore phải giữ core và ghi provider.)
    assert_eq!(
        fs::read_to_string(project.join(".mgc.core"))
            .unwrap()
            .trim(),
        "cicd"
    );
    let manifest = fs::read_to_string(project.join("mgc.toml")).unwrap();
    assert!(manifest.contains("ecosystem = \"cicd\""), "{manifest}");
    assert!(
        manifest.contains("provider = \"github-actions\""),
        "{manifest}"
    );

    let scaffolded = fs::read_to_string(&workflow_path).unwrap();
    let (ok, _, stderr) = run(hermetic(home.path(), &project).arg("ci-generate"));
    assert!(ok, "ci-generate inside the new project failed: {stderr}");

    let regenerated = fs::read_to_string(&workflow_path).unwrap();
    // The `name:` line is a generator-time label: the create names the workflow
    // after the project, while `ci-generate` names it after the directory it was
    // invoked from. Everything else must be byte-identical, which is what proves
    // both lanes use ONE emitter.
    // (Dòng `name:` là nhãn lúc sinh: create đặt tên workflow theo project, còn
    // `ci-generate` theo thư mục gọi lệnh. Phần còn lại phải giống từng byte —
    // đó là bằng chứng hai lane dùng CHUNG một emitter.)
    assert_eq!(
        scaffolded.lines().next(),
        Some("name: ci-demo"),
        "the scaffolded workflow must name the project"
    );
    assert_eq!(
        pipeline_body(&scaffolded),
        pipeline_body(&regenerated),
        "ci-generate must rewrite the identical pipeline body"
    );
}

#[test]
fn create_cicd_gitlab_and_circleci_scaffold_offline() {
    for (framework, pipeline) in [
        ("gitlab", ".gitlab-ci.yml"),
        ("circleci", ".circleci/config.yml"),
    ] {
        let home = TempDir::new().unwrap();
        let workspace = TempDir::new().unwrap();
        let (ok, stdout, stderr) = run(hermetic(home.path(), workspace.path()).args([
            "create-cicd",
            framework,
            "ci-demo",
            "--yes",
        ]));
        assert!(ok, "create-cicd {framework} failed: {stderr}\n{stdout}");
        let project = workspace.path().join("ci-demo");
        assert!(
            project.join(pipeline).is_file(),
            "{framework}: {pipeline} missing"
        );
        let (ok, _, stderr) = run(hermetic(home.path(), &project).arg("ci-generate"));
        assert!(ok, "{framework}: ci-generate failed: {stderr}");
    }
}

#[test]
fn unknown_cicd_framework_still_fails_closed() {
    // The offline generator must not turn a typo into a project.
    // (Generator offline không được biến lỗi gõ thành một project.)
    let home = TempDir::new().unwrap();
    let workspace = TempDir::new().unwrap();
    let (ok, stdout, stderr) = run(hermetic(home.path(), workspace.path()).args([
        "create-cicd",
        "not-a-provider",
        "ci-demo",
        "--yes",
    ]));
    assert!(!ok, "an unknown framework must fail: {stdout}{stderr}");
    assert!(
        !workspace.path().join("ci-demo").exists(),
        "nothing may be created for an unknown framework"
    );
}

/// Everything after the generator-time `name:` label.
/// Toàn bộ nội dung sau nhãn `name:` do generator đặt.
fn pipeline_body(workflow: &str) -> String {
    workflow.lines().skip(1).collect::<Vec<_>>().join("\n")
}
