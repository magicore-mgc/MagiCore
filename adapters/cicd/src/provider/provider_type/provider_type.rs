//! CI/CD provider variants and stable identifiers.
//! Variant provider CI/CD và định danh ổn định.

/// Provider identities recognized by the CI/CD adapter.
/// Định danh provider mà adapter CI/CD hiện nhận diện.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CicdProvider {
    GithubActions,
    Gitlab,
    CircleCi,
    Cloudflare,
    Aws,
    Gcp,
    Argocd,
}

impl CicdProvider {
    /// Return the provider's existing canonical identifier.
    /// Trả định danh chuẩn hiện dùng của provider.
    pub fn as_str(&self) -> &'static str {
        match self {
            CicdProvider::GithubActions => "github-actions",
            CicdProvider::Gitlab => "gitlab",
            CicdProvider::CircleCi => "circleci",
            CicdProvider::Cloudflare => "cloudflare",
            CicdProvider::Aws => "aws",
            CicdProvider::Gcp => "gcp",
            CicdProvider::Argocd => "argocd",
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
