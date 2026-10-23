//! CI scaffold honesty tests (D-A1..A6, D-P1..P6).
//! Test tính trung thực của scaffold CI.

use super::super::write;
use mgc_config::project::{CiConfig, ProjectConfig};

fn config_with(ci: Option<CiConfig>) -> ProjectConfig {
    let mut config = ProjectConfig::new("sample", "web");
    config.frameworks = vec!["django".to_string()];
    config.ci = ci;
    config
}

fn dir(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mgc-ci-{tag}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}

#[test]
fn refuses_without_build_and_test() {
    let target = dir("missing");
    let error = write(&target, &config_with(None)).expect_err("must refuse");
    let message = error.to_string();
    assert!(message.contains("ci.build and ci.test"), "{message}");
    assert!(message.contains("No workflow was written"), "{message}");
    assert!(!target.join(".github/workflows/ci.yml").exists());
    std::fs::remove_dir_all(&target).ok();
}

#[test]
fn refuses_when_only_build_is_declared() {
    let target = dir("build-only");
    let ci = CiConfig {
        build: "python -m build".into(),
        ..CiConfig::default()
    };
    let error = write(&target, &config_with(Some(ci))).expect_err("must refuse");
    assert!(error.to_string().contains("ci.test"), "{error}");
    assert!(!target.join(".github/workflows/ci.yml").exists());
    std::fs::remove_dir_all(&target).ok();
}

#[test]
fn never_emits_placeholder_echo_and_uses_declared_commands() {
    let target = dir("declared");
    let ci = CiConfig {
        build: "python -m build --wheel".into(),
        test: "pytest -q".into(),
        ..CiConfig::default()
    };
    write(&target, &config_with(Some(ci))).expect("write");

    let yaml = std::fs::read_to_string(target.join(".github/workflows/ci.yml")).expect("yaml");
    assert!(
        !yaml.contains("echo \"MagiCore CI scaffold\""),
        "no echo test"
    );
    assert!(yaml.contains("mgc install --frozen"));
    assert!(yaml.contains("python -m build --wheel"));
    assert!(yaml.contains("pytest -q"));
    // No production block and no publish job without a Dockerfile.
    assert!(!yaml.contains("stage-artifact"));
    assert!(!yaml.contains("publish:"));
    std::fs::remove_dir_all(&target).ok();
}

#[test]
fn production_artifact_is_staged_checked_and_promoted_by_digest() {
    let target = dir("production");
    let ci = CiConfig {
        build: "python -m build --wheel".into(),
        test: "pytest -q".into(),
        smoke: "curl -fsS http://localhost:4315/api/health".into(),
        production: "Dockerfile".into(),
        runtime_image: "ghcr.io/acme/app:staging".into(),
    };
    write(&target, &config_with(Some(ci))).expect("write");

    let yaml = std::fs::read_to_string(target.join(".github/workflows/ci.yml")).expect("yaml");
    assert!(yaml.contains("stage-artifact"));
    assert!(yaml.contains("docker run -d --name smoke -p 4315:4315"));
    assert!(yaml.contains("curl -fsS http://localhost:4315/api/health"));
    assert!(yaml.contains("needs: [stage-artifact]"));
    assert!(yaml.contains("--artifact-digest \"$VERIFIED_DIGEST\""));
    // Staging must never rebuild at publish time.
    let publish = yaml.split("publish:").nth(1).expect("publish job");
    assert!(!publish.contains("docker build"));
    assert!(publish.contains("id-token: write"));
    assert!(
        yaml.contains("never production secrets") || yaml.contains("không dùng secret production")
    );
    std::fs::remove_dir_all(&target).ok();
}

#[test]
fn missing_smoke_fails_the_workflow_instead_of_claiming_success() {
    let target = dir("no-smoke");
    let ci = CiConfig {
        build: "python -m build --wheel".into(),
        test: "pytest -q".into(),
        production: "Dockerfile".into(),
        ..CiConfig::default()
    };
    write(&target, &config_with(Some(ci))).expect("write");

    let yaml = std::fs::read_to_string(target.join(".github/workflows/ci.yml")).expect("yaml");
    assert!(yaml.contains("ci.smoke is required"));
    assert!(yaml.contains("exit 1"));
    std::fs::remove_dir_all(&target).ok();
}
