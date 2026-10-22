use anyhow::Result;

/// Reject a configured build step for CI/CD projects, which produce pipeline configuration rather than a build artifact.
/// Từ chối bước build được cấu hình cho project CI/CD vì đầu ra là pipeline config, không phải build artifact.
pub(in crate::commands::core::dev::cicd::verify) fn ensure_build_step_supported(
    core: &str,
) -> Result<()> {
    if core == "cicd" {
        return Err(crate::error::build_not_supported(
            core,
            "CI/CD projects have no build artifact; remove `build` from `[cicd].verify` or define a project-specific build command",
        ));
    }
    Ok(())
}
