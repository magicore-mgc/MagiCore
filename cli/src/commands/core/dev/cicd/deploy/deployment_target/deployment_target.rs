use super::super::super::DeployTarget;

/// Reject provider CLI pass-through until MagiCore owns a native deploy path.
/// (Từ chối gọi CLI vendor cho tới khi MagiCore có đường deploy native.)
pub(in crate::commands::core::dev::cicd::deploy) fn target_deploy_unavailable(
    target: &DeployTarget,
) -> anyhow::Error {
    match target.provider.as_str() {
        "cloudflare" | "gcp" | "google" => crate::error::deploy_not_implemented(&target.provider),
        "aws" => crate::error::cicd_deploy_target_not_implemented(
            &target.provider,
            &target.stack,
            if target.region.is_empty() {
                "default"
            } else {
                &target.region
            },
        ),
        other => crate::error::deploy_target_unknown(other),
    }
}
