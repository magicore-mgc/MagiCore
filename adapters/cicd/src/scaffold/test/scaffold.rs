//! Offline CI/CD scaffold tests — kiểm scaffold CI/CD offline.
//!
//! Every case writes into a fresh directory and asserts the pipeline bytes,
//! not just that a file appeared: the contract is that `create-cicd` and
//! `mgc ci-generate` share ONE generator, so a scaffolded pipeline must equal a
//! regenerated one.
//! Mỗi ca ghi vào thư mục mới và kiểm bytes pipeline, không chỉ kiểm có file:
//! contract là `create-cicd` và `mgc ci-generate` dùng CHUNG một generator, nên
//! pipeline scaffold phải bằng pipeline sinh lại.

use std::path::{Path, PathBuf};

use super::super::{
    GENERATED_PROVIDERS, WORKFLOW_TEMPLATE, can_generate, ensure_release_pin, generation_plan,
    render_ci_template, scaffold_project,
};
use crate::provider::{CicdProvider, provider_from_str};

#[test]
fn github_actions_writes_the_shared_workflow_bytes() {
    let dir = target("github-actions");
    run(CicdProvider::GithubActions, "ci-demo", &dir);

    let workflow = std::fs::read_to_string(dir.join(".github/workflows/ci.yml"))
        .expect("generated GitHub Actions workflow");
    assert_eq!(
        workflow,
        render_ci_template(WORKFLOW_TEMPLATE, "ci-demo"),
        "the scaffolded workflow must be the exact ci-generate rendering"
    );
    assert!(
        workflow.starts_with("name: ci-demo\n"),
        "the workflow names the project: {workflow}"
    );
    assert!(dir.join("README.md").is_file(), "README.md missing");
    assert!(dir.join(".gitignore").is_file(), ".gitignore missing");
    cleanup(&dir);
}

#[test]
fn gitlab_and_circleci_write_their_pipeline_paths() {
    let gitlab = target("gitlab");
    run(CicdProvider::Gitlab, "ci-demo", &gitlab);
    assert!(
        gitlab.join(".gitlab-ci.yml").is_file(),
        ".gitlab-ci.yml missing"
    );

    let circle = target("circleci");
    run(CicdProvider::CircleCi, "ci-demo", &circle);
    assert!(
        circle.join(".circleci/config.yml").is_file(),
        ".circleci/config.yml missing"
    );

    cleanup(&gitlab);
    cleanup(&circle);
}

#[test]
fn generation_plan_names_are_byte_identical_for_every_released_provider() {
    // A provider listed as generatable must really produce a plan; the reverse
    // would be the hollow scaffold claim PAR-02 removed.
    // (Provider được liệt kê sinh được phải thật sự có plan; ngược lại là claim
    // scaffold rỗng mà PAR-02 đã loại bỏ.)
    for name in GENERATED_PROVIDERS {
        let provider = provider_from_str(name).expect("released provider parses");
        assert!(can_generate(provider), "{name} must be generatable");
        let plan = generation_plan(provider, "demo").expect("plan");
        assert!(!plan.is_empty(), "{name} must emit at least one file");
    }
}

#[test]
fn provider_without_template_is_refused_not_faked() {
    // Jenkins has no released template in this build: writing nothing and
    // reporting success is exactly what the generator must never do.
    // (Jenkins chưa có template phát hành trong build này: không ghi gì mà báo
    // thành công là điều generator không được phép làm.)
    assert!(!can_generate(CicdProvider::Jenkins));
    let error = generation_plan(CicdProvider::Jenkins, "demo")
        .expect_err("a provider without a template must be refused");
    assert!(
        error.to_string().contains("no offline pipeline template"),
        "refusal must name the missing template: {error}"
    );
}

#[test]
fn release_pin_is_present_in_this_build() {
    ensure_release_pin().expect("released builds pin the installer");
}

#[test]
fn existing_destination_is_refused() {
    let dir = target("existing");
    std::fs::create_dir_all(&dir).expect("pre-create destination");
    let outcome = runtime().block_on(scaffold_project(
        CicdProvider::GithubActions,
        "ci-demo",
        &dir,
    ));
    assert!(outcome.is_err(), "an existing destination must fail closed");
    cleanup(&dir);
}

/// Run the generator for a fresh destination.
/// Chạy generator cho một đích mới.
fn run(provider: CicdProvider, name: &str, dir: &Path) {
    runtime()
        .block_on(scaffold_project(provider, name, dir))
        .unwrap_or_else(|error| panic!("cicd scaffold failed: {error}"));
}

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

/// A unique, not-yet-existing destination directory.
/// Thư mục đích duy nhất, chưa tồn tại.
fn target(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "mgc-cicd-scaffold-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}

fn cleanup(dir: &Path) {
    std::fs::remove_dir_all(dir).ok();
}
