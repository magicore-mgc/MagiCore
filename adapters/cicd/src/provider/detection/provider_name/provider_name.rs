//! Canonical CI/CD provider name parser.
//! Parser tên provider CI/CD chuẩn.

use crate::provider::CicdProvider;

/// Map a recognized canonical provider name.
/// Ánh xạ tên provider chuẩn đã được nhận diện.
pub(in crate::provider::detection) fn provider_from_str(name: &str) -> Option<CicdProvider> {
    match name {
        "github-actions" => Some(CicdProvider::GithubActions),
        "gitlab" => Some(CicdProvider::Gitlab),
        "circleci" => Some(CicdProvider::CircleCi),
        "cloudflare" => Some(CicdProvider::Cloudflare),
        "aws" => Some(CicdProvider::Aws),
        "gcp" => Some(CicdProvider::Gcp),
        "argocd" => Some(CicdProvider::Argocd),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
