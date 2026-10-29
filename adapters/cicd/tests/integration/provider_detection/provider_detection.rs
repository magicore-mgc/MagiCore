//! Provider Detection behavior tests.
//! Test hành vi cho nhóm provider detection.

use crate::integration::support::tmp;
use mgc_cicd_adapter::{CicdProvider, detect_provider};

#[cfg(unix)]
pub fn provider_detection_does_not_follow_external_mgc_config_symlink() {
    let project = tmp("external-mgc-config-link");
    let external = tmp("external-mgc-config-target");
    std::fs::write(external.join("mgc.toml"), "[cicd]\nprovider = 'gitlab'\n").unwrap();
    std::os::unix::fs::symlink(external.join("mgc.toml"), project.join("mgc.toml")).unwrap();

    assert_eq!(detect_provider(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}

pub fn detect_cloudflare_via_wrangler_toml() {
    let dir = tmp("cf");
    std::fs::write(dir.join("wrangler.toml"), "name = \"worker\"\n").unwrap();
    assert_eq!(detect_provider(&dir), Some(CicdProvider::Cloudflare));
}

pub fn detect_github_actions_via_workflows_dir() {
    let dir = tmp("gha");
    std::fs::create_dir_all(dir.join(".github").join("workflows")).unwrap();
    std::fs::write(
        dir.join(".github").join("workflows").join("ci.yml"),
        "name: CI\non: [push]\n",
    )
    .unwrap();
    assert_eq!(detect_provider(&dir), Some(CicdProvider::GithubActions));
}

pub fn detect_gitlab_via_gitlab_ci_yml() {
    let dir = tmp("gl");
    std::fs::write(dir.join(".gitlab-ci.yml"), "stages: [test]\n").unwrap();
    assert_eq!(detect_provider(&dir), Some(CicdProvider::Gitlab));
}

pub fn detect_circleci_via_config_yml() {
    let dir = tmp("cc");
    std::fs::create_dir_all(dir.join(".circleci")).unwrap();
    std::fs::write(dir.join(".circleci").join("config.yml"), "version: 2.1\n").unwrap();
    assert_eq!(detect_provider(&dir), Some(CicdProvider::CircleCi));
}

pub fn detect_argocd_via_application_yaml() {
    let dir = tmp("argo");
    std::fs::create_dir_all(dir.join("argocd")).unwrap();
    std::fs::write(
        dir.join("argocd").join("application.yaml"),
        "kind: Application\n",
    )
    .unwrap();
    assert_eq!(detect_provider(&dir), Some(CicdProvider::Argocd));
}

pub fn detect_aws_via_main_tf() {
    let dir = tmp("aws-tf");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    assert_eq!(detect_provider(&dir), Some(CicdProvider::Aws));
}

pub fn detect_aws_via_mgc_toml_overrides_file_marker() {
    let dir = tmp("mgc-aws");
    // wrangler.toml marks Cloudflare; mgc.toml takes precedence and selects AWS.
    // wrangler.toml → Cloudflare, mgc.toml → Aws (mgc.toml thắng)
    std::fs::write(dir.join("wrangler.toml"), "name = \"worker\"\n").unwrap();
    std::fs::write(dir.join("mgc.toml"), "[cicd]\nprovider = \"aws\"\n").unwrap();
    assert_eq!(detect_provider(&dir), Some(CicdProvider::Aws));
}

pub fn detect_gcp_via_mgc_toml() {
    let dir = tmp("mgc-gcp");
    std::fs::write(dir.join("mgc.toml"), "[cicd]\nprovider = \"gcp\"\n").unwrap();
    assert_eq!(detect_provider(&dir), Some(CicdProvider::Gcp));
}

pub fn detect_returns_none_for_empty_dir() {
    let dir = tmp("empty");
    assert!(detect_provider(&dir).is_none());
}

pub fn detect_returns_none_for_unknown_mgc_toml_provider() {
    let dir = tmp("unknown");
    std::fs::write(dir.join("mgc.toml"), "[cicd]\nprovider = \"travis\"\n").unwrap();
    // Travis is unsupported, so provider detection returns None.
    // "travis" chưa được hỗ trợ → None
    assert!(detect_provider(&dir).is_none());
}
