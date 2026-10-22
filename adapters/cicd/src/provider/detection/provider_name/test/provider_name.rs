//! Tests for canonical CI/CD provider names.
//! Test tên provider CI/CD chuẩn.

use super::super::provider_from_str;
use crate::provider::CicdProvider;

#[test]
fn maps_all_existing_provider_names() {
    assert_eq!(
        provider_from_str("github-actions"),
        Some(CicdProvider::GithubActions)
    );
    assert_eq!(provider_from_str("gitlab"), Some(CicdProvider::Gitlab));
    assert_eq!(provider_from_str("circleci"), Some(CicdProvider::CircleCi));
    assert_eq!(
        provider_from_str("cloudflare"),
        Some(CicdProvider::Cloudflare)
    );
    assert_eq!(provider_from_str("aws"), Some(CicdProvider::Aws));
    assert_eq!(provider_from_str("gcp"), Some(CicdProvider::Gcp));
    assert_eq!(provider_from_str("argocd"), Some(CicdProvider::Argocd));
}

#[test]
fn unknown_provider_name_is_not_normalized_or_accepted() {
    assert_eq!(provider_from_str("GitHub-Actions"), None);
    assert_eq!(provider_from_str("jenkins"), None);
}
