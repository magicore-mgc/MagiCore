//! Existing file-marker provider detection.
//! Nhận diện provider qua các file marker hiện hành.

use std::path::Path;

use crate::provider::CicdProvider;

/// Return the first provider whose existing marker path exists.
/// Trả provider đầu tiên có marker path hiện hành.
pub(in crate::provider::detection) fn provider_from_markers(root: &Path) -> Option<CicdProvider> {
    if root.join("wrangler.toml").exists() {
        return Some(CicdProvider::Cloudflare);
    }
    if root.join("argocd").join("application.yaml").exists() {
        return Some(CicdProvider::Argocd);
    }
    if root.join(".github").join("workflows").exists() {
        return Some(CicdProvider::GithubActions);
    }
    if root.join(".gitlab-ci.yml").exists() {
        return Some(CicdProvider::Gitlab);
    }
    if root.join(".circleci").join("config.yml").exists() {
        return Some(CicdProvider::CircleCi);
    }
    if root.join("main.tf").exists() {
        return Some(CicdProvider::Aws);
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
