//! CI/CD adapter facade with stable public exports.
//! Facade adapter CICD giữ các public export ổn định.

mod adapter;
mod adapter_factory;
mod audit_provider;
mod capability_manifest;
mod core_identity;
mod optimizer_provider;
mod package_adapter;
mod project_detection;
mod provider_name;
mod scaffold_provider;
mod unclaimed_capabilities;

pub use adapter::CicdAdapter;
pub use adapter_factory::adapter_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

/// Providers the generator can actually emit a pipeline for.
/// Provider mà generator thật sự sinh được pipeline.
///
/// Kept next to the adapter because a detected provider with no template is
/// exactly the hollow claim PAR-02 removed.
/// (Đặt cạnh adapter vì provider được nhận diện mà không có template chính là
/// claim rỗng mà PAR-02 loại bỏ.)
pub(crate) fn pipeline_providers() -> &'static [&'static str] {
    &[
        "github-actions",
        "gitlab",
        "circleci",
        "jenkins",
        "azure-pipelines",
        "bitbucket-pipelines",
        "argocd",
    ]
}
