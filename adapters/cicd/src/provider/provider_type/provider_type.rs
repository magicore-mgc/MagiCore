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

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
