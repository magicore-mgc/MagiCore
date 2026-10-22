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
