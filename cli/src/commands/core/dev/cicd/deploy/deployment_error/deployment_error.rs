pub(super) fn not_available(reason: &str) -> anyhow::Error {
    crate::error::cicd_reason(reason)
}

pub(in crate::commands::core::dev::cicd::deploy) fn provider_deploy_unavailable(
    provider: mgc_cicd_adapter::CicdProvider,
) -> anyhow::Error {
    match provider {
        mgc_cicd_adapter::CicdProvider::Cloudflare => {
            crate::error::deploy_not_implemented("cloudflare")
        }
        mgc_cicd_adapter::CicdProvider::Gcp => crate::error::deploy_not_implemented("gcp"),
        mgc_cicd_adapter::CicdProvider::GithubActions
        | mgc_cicd_adapter::CicdProvider::Gitlab
        | mgc_cicd_adapter::CicdProvider::CircleCi => {
            not_available("is CI-only — push to trigger; no local deploy command.")
        }
        mgc_cicd_adapter::CicdProvider::Aws => crate::error::deploy_not_implemented("aws"),
        mgc_cicd_adapter::CicdProvider::Argocd => not_available(
            "argocd runs server-side (GitOps) — commit + push to trigger sync; no local deploy command.",
        ),
    }
}
