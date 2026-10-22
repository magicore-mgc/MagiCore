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
    ];

    for (provider, expected_path, creates_parent, marker) in cases {
        let (path, content, actual_parent_policy, _) = generation_plan(provider).unwrap();
        assert_eq!(path, expected_path);
        assert_eq!(actual_parent_policy, creates_parent);
        assert!(content.contains(marker));
    }
}

#[test]
fn unsupported_provider_has_no_generation_plan() {
    assert!(generation_plan(mgc_cicd_adapter::CicdProvider::Aws).is_err());
}
