//! CI/CD provider variants and stable identifiers.
//! Variant provider CI/CD và định danh ổn định.

/// Provider identities recognized by the CI/CD adapter.
/// Định danh provider mà adapter CI/CD hiện nhận diện.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CicdProvider {
    GithubActions,
    Gitlab,
    CircleCi,
    /// Jenkins pipeline (`Jenkinsfile`) — pipeline sinh `Jenkinsfile` khai báo.
    /// (Jenkins dùng `Jenkinsfile`; pipeline sinh file này.)
    Jenkins,
    /// Azure Pipelines (`azure-pipelines.yml`).
    /// (Azure Pipelines dùng `azure-pipelines.yml`.)
    AzurePipelines,
    /// Bitbucket Pipelines (`bitbucket-pipelines.yml`).
    /// (Bitbucket Pipelines dùng `bitbucket-pipelines.yml`.)
    BitbucketPipelines,
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
            CicdProvider::Jenkins => "jenkins",
            CicdProvider::AzurePipelines => "azure-pipelines",
            CicdProvider::BitbucketPipelines => "bitbucket-pipelines",
            CicdProvider::Cloudflare => "cloudflare",
            CicdProvider::Aws => "aws",
            CicdProvider::Gcp => "gcp",
            CicdProvider::Argocd => "argocd",
        }
    }
}

/// Map a recognized canonical provider name.
/// Ánh xạ tên provider chuẩn đã được nhận diện.
///
/// This is the single name table: config detection, the offline generator and
/// any future lane resolve `github-actions`, `gitlab`, ... through it, so a new
/// spelling cannot be accepted in one lane and rejected in another.
/// Đây là bảng tên duy nhất: nhận diện config, generator offline và mọi lane
/// sau này resolve `github-actions`, `gitlab`, ... qua nó, nên cách viết mới
/// không thể được nhận ở lane này mà bị từ chối ở lane khác.
pub fn provider_from_str(name: &str) -> Option<CicdProvider> {
    match name {
        "github-actions" => Some(CicdProvider::GithubActions),
        "gitlab" => Some(CicdProvider::Gitlab),
        "circleci" => Some(CicdProvider::CircleCi),
        "jenkins" => Some(CicdProvider::Jenkins),
        "azure-pipelines" | "azure" => Some(CicdProvider::AzurePipelines),
        "bitbucket-pipelines" | "bitbucket" => Some(CicdProvider::BitbucketPipelines),
        "cloudflare" => Some(CicdProvider::Cloudflare),
        "aws" => Some(CicdProvider::Aws),
        "gcp" => Some(CicdProvider::Gcp),
        "argocd" => Some(CicdProvider::Argocd),
        _ => None,
    }
}

impl std::str::FromStr for CicdProvider {
    type Err = ();

    /// Parse a canonical provider id — parse id provider chuẩn.
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        provider_from_str(name).ok_or(())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
