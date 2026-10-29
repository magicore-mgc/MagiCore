use super::*;
use std::path::PathBuf;

#[test]
fn provider_generation_plan_keeps_existing_paths_and_template_kinds() {
    let cases = [
        (
            mgc_cicd_adapter::CicdProvider::GithubActions,
            PathBuf::from(".github").join("workflows").join("ci.yml"),
            true,
            "name: CI",
        ),
        (
            mgc_cicd_adapter::CicdProvider::Gitlab,
            PathBuf::from(".gitlab-ci.yml"),
            false,
            "stages:",
        ),
        (
            mgc_cicd_adapter::CicdProvider::CircleCi,
            PathBuf::from(".circleci").join("config.yml"),
            true,
            "version: 2.1",
        ),
        (
            mgc_cicd_adapter::CicdProvider::Jenkins,
            PathBuf::from("Jenkinsfile"),
            false,
            "pipeline {",
        ),
        (
            mgc_cicd_adapter::CicdProvider::AzurePipelines,
            PathBuf::from("azure-pipelines.yml"),
            false,
            "vmImage: ubuntu-latest",
        ),
        (
            mgc_cicd_adapter::CicdProvider::BitbucketPipelines,
            PathBuf::from("bitbucket-pipelines.yml"),
            false,
            "pipelines:",
        ),
    ];

    for (provider, expected_path, creates_parent, marker) in cases {
        let (path, content, actual_parent_policy, _) = generation_plan(provider).unwrap();
        assert_eq!(path, expected_path);
        assert_eq!(actual_parent_policy, creates_parent);
        assert!(content.contains(marker));
    }
}

/// Every generated pipeline must carry the immutable installer pin.
///
/// A template that forgot `{installer_sha}` would ship a pipeline installing
/// MagiCore from mutable `main` — the exact supply-chain regression P0-E closed.
/// (Mọi pipeline sinh ra phải chứa pin installer bất biến. Template quên
/// `{installer_sha}` sẽ ship pipeline cài từ `main` mutable.)
#[test]
fn every_generated_template_pins_the_installer_commit() {
    let providers = [
        mgc_cicd_adapter::CicdProvider::GithubActions,
        mgc_cicd_adapter::CicdProvider::Gitlab,
        mgc_cicd_adapter::CicdProvider::CircleCi,
        mgc_cicd_adapter::CicdProvider::Jenkins,
        mgc_cicd_adapter::CicdProvider::AzurePipelines,
        mgc_cicd_adapter::CicdProvider::BitbucketPipelines,
    ];
    for provider in providers {
        let (_, content, _, _) = generation_plan(provider).unwrap();
        assert!(
            content.contains(super::super::super::templates::MGC_INSTALLER_SHA),
            "{provider:?} does not pin the installer commit"
        );
        assert!(
            !content.contains("{installer_sha}") && !content.contains("{tag}"),
            "{provider:?} still carries an unrendered placeholder"
        );
    }
}

/// New providers must be reachable by the names a user would write in mgc.toml.
///
/// The user path is `[cicd] provider = "<name>"`, so the assertion drives the
/// public `detect_provider` over a real temp project instead of a private parser.
/// (Provider mới phải truy cập được bằng tên người dùng viết trong mgc.toml.
/// Đường thật là `[cicd] provider` nên test gọi `detect_provider` công khai trên
/// project tạm, không gọi parser nội bộ.)
#[test]
fn new_providers_are_reachable_from_project_config() {
    for (name, expected) in [
        ("jenkins", mgc_cicd_adapter::CicdProvider::Jenkins),
        ("azure", mgc_cicd_adapter::CicdProvider::AzurePipelines),
        (
            "azure-pipelines",
            mgc_cicd_adapter::CicdProvider::AzurePipelines,
        ),
        (
            "bitbucket",
            mgc_cicd_adapter::CicdProvider::BitbucketPipelines,
        ),
        (
            "bitbucket-pipelines",
            mgc_cicd_adapter::CicdProvider::BitbucketPipelines,
        ),
    ] {
        let dir = tempfile::tempdir().expect("temp project");
        std::fs::write(
            dir.path().join("mgc.toml"),
            format!("[cicd]\nprovider = \"{name}\"\n"),
        )
        .expect("write mgc.toml");
        let detected = mgc_cicd_adapter::detect_provider(dir.path());
        assert_eq!(
            detected,
            Some(expected),
            "provider name '{name}' not recognized"
        );
    }
}

/// File-marker detection must find the new providers without any config.
/// (Nhận diện qua file marker phải tìm được provider mới mà không cần config.)
#[test]
fn new_providers_are_detected_from_marker_files() {
    for (marker, expected) in [
        ("Jenkinsfile", mgc_cicd_adapter::CicdProvider::Jenkins),
        (
            "azure-pipelines.yml",
            mgc_cicd_adapter::CicdProvider::AzurePipelines,
        ),
        (
            "bitbucket-pipelines.yml",
            mgc_cicd_adapter::CicdProvider::BitbucketPipelines,
        ),
    ] {
        let dir = tempfile::tempdir().expect("temp project");
        std::fs::write(dir.path().join(marker), "").expect("write marker");
        let detected = mgc_cicd_adapter::detect_provider(dir.path());
        assert_eq!(detected, Some(expected), "marker '{marker}' not detected");
    }
}

#[test]
fn unsupported_provider_has_no_generation_plan() {
    assert!(generation_plan(mgc_cicd_adapter::CicdProvider::Aws).is_err());
}
