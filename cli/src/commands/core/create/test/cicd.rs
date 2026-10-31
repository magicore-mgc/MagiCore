//! `create-cicd` tests — kiểm thử `create-cicd`.
//!
//! The cicd core has no registry layer, so the offline generator is the only
//! source it has. These cases drive it exactly as `mgc create-cicd` does: the
//! local-generator request, then MagiCore's own metadata write. A green run
//! proves a fresh machine scaffolds a detectable CI/CD project whose pipeline
//! is the byte-exact output of `mgc ci-generate`.
//! Core cicd chưa có registry layer nên generator offline là nguồn duy nhất.
//! Các ca này chạy nó đúng như `mgc create-cicd`: request local generator, rồi
//! ghi metadata của MagiCore. Test xanh chứng minh máy mới scaffold được
//! project CI/CD nhận diện được, với pipeline đúng từng byte như
//! `mgc ci-generate`.

use std::path::{Path, PathBuf};

use crate::wizard::engine::ScaffoldConfig;

#[test]
fn github_actions_scaffolds_offline_and_is_core_detectable() {
    let workspace = workspace("github-actions");
    let target = workspace.join("ci-demo");
    scaffold("github-actions", &target);

    let workflow = std::fs::read_to_string(target.join(".github/workflows/ci.yml"))
        .expect("GitHub Actions workflow");
    assert_eq!(
        workflow,
        mgc_cicd_adapter::scaffold::render_ci_template(
            mgc_cicd_adapter::scaffold::WORKFLOW_TEMPLATE,
            "ci-demo"
        ),
        "the scaffolded workflow must equal what `mgc ci-generate` writes"
    );
    assert!(target.join("README.md").is_file(), "README.md missing");
    assert!(target.join(".gitignore").is_file(), ".gitignore missing");

    // The create must leave a project every other cicd command recognises:
    // `.mgc.core` claims the core, `mgc.toml` records the provider, and the
    // provider is detected from that config on the next run.
    // (Create phải để lại project mọi lệnh cicd khác nhận ra: `.mgc.core` giữ
    // core, `mgc.toml` ghi provider, và provider được nhận diện từ config đó.)
    assert_eq!(
        read_marker(&target.join(".mgc.core")).as_deref(),
        Some("cicd")
    );
    let manifest = std::fs::read_to_string(target.join("mgc.toml")).expect("mgc.toml");
    assert!(
        manifest.contains("[cicd]") && manifest.contains("provider = \"github-actions\""),
        "mgc.toml must record the provider: {manifest}"
    );
    assert!(
        manifest.contains("frameworks = [\"github-actions\"]"),
        "mgc.toml must record the framework: {manifest}"
    );
    assert_eq!(
        mgc_cicd_adapter::detect_provider(&target),
        Some(mgc_cicd_adapter::CicdProvider::GithubActions)
    );
    cleanup(&workspace);
}

#[test]
fn gitlab_and_circleci_scaffold_offline() {
    for (framework, pipeline) in [
        ("gitlab", ".gitlab-ci.yml"),
        ("circleci", ".circleci/config.yml"),
    ] {
        let workspace = workspace(framework);
        let target = workspace.join("ci-demo");
        scaffold(framework, &target);
        assert!(
            target.join(pipeline).is_file(),
            "{framework}: {pipeline} missing"
        );
        assert_eq!(
            read_marker(&target.join(".mgc.core")).as_deref(),
            Some("cicd")
        );
        cleanup(&workspace);
    }
}

#[test]
fn provider_without_a_template_declines_instead_of_faking() {
    // Jenkins is a known provider but has no released offline template, so the
    // generator must decline and leave the caller's registry/fail-closed path in
    // charge — never create an empty project.
    // (Jenkins là provider đã biết nhưng chưa có template offline, nên generator
    // phải từ chối và để caller giữ đường registry/fail-closed — không bao giờ
    // tạo project rỗng.)
    let workspace = workspace("jenkins");
    let target = workspace.join("ci-demo");
    let generated = runtime()
        .block_on(super::super::local_generator::try_local_scaffold(
            &super::super::local_generator::LocalScaffoldRequest {
                core: "cicd",
                framework: "jenkins",
                target: &target,
                board: None,
            },
        ))
        .expect("local scaffold attempt");
    assert!(!generated, "a provider without a template must decline");
    assert!(!target.exists(), "a declined create must write nothing");
    cleanup(&workspace);
}

/// Run the local generator and then MagiCore's metadata write, as `run` does.
/// Chạy local generator rồi ghi metadata MagiCore, đúng như `run` làm.
fn scaffold(framework: &str, target: &Path) {
    let generated = runtime()
        .block_on(super::super::local_generator::try_local_scaffold(
            &super::super::local_generator::LocalScaffoldRequest {
                core: "cicd",
                framework,
                target,
                board: None,
            },
        ))
        .unwrap_or_else(|error| panic!("cicd local scaffold failed: {error}"));
    assert!(generated, "expected '{framework}' to scaffold offline");

    let name = target
        .file_name()
        .and_then(|name| name.to_str())
        .expect("target name");
    let config = ScaffoldConfig {
        project_name: name.to_string(),
        frameworks: vec![framework.to_string()],
        core: "cicd".to_string(),
        ..Default::default()
    };
    super::super::save_scaffold_metadata(target, &config).expect("metadata write");
}

/// First non-empty line of a marker file — dòng đầu không rỗng của file marker.
fn read_marker(path: &Path) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()?
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(str::to_string)
}

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

/// A unique, not-yet-existing directory — thư mục duy nhất, chưa tồn tại.
fn workspace(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "mgc-create-cicd-{label}-{}-{}",
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
