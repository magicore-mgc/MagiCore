//! Tests for public CI/CD provider identifiers.
//! Test định danh provider CI/CD công khai.

use super::super::CicdProvider;

#[test]
fn stable_provider_ids_are_preserved() {
    assert_eq!(CicdProvider::GithubActions.as_str(), "github-actions");
    assert_eq!(CicdProvider::Gitlab.as_str(), "gitlab");
    assert_eq!(CicdProvider::CircleCi.as_str(), "circleci");
    assert_eq!(CicdProvider::Cloudflare.as_str(), "cloudflare");
    assert_eq!(CicdProvider::Aws.as_str(), "aws");
    assert_eq!(CicdProvider::Gcp.as_str(), "gcp");
    assert_eq!(CicdProvider::Argocd.as_str(), "argocd");
}

#[test]
fn canonical_parser_round_trips_every_identifier() {
    for provider in [
        crate::provider::CicdProvider::GithubActions,
        crate::provider::CicdProvider::Gitlab,
        crate::provider::CicdProvider::CircleCi,
        crate::provider::CicdProvider::Jenkins,
        crate::provider::CicdProvider::AzurePipelines,
        crate::provider::CicdProvider::BitbucketPipelines,
        crate::provider::CicdProvider::Cloudflare,
        crate::provider::CicdProvider::Aws,
        crate::provider::CicdProvider::Gcp,
        crate::provider::CicdProvider::Argocd,
    ] {
        assert_eq!(
            crate::provider::provider_from_str(provider.as_str()),
            Some(provider),
            "{} must parse back to itself",
            provider.as_str()
        );
    }
    assert!(crate::provider::provider_from_str("not-a-provider").is_none());
}
