use anyhow::Result;

/// Run the shared `mgc test` policy for this project root; never report a skipped step as success.
/// Dùng chung chính sách `mgc test` trên project root; không bao giờ báo thành công khi đã skip.
pub(in crate::commands::core::dev::cicd::verify) async fn run_test_step(
    root: &std::path::Path,
    core: &str,
) -> Result<()> {
    let core_override = (!core.is_empty()).then_some(core);
    crate::commands::test::test_at(root, Vec::new(), core_override, None, None)
        .await
        .map_err(|error| crate::error::cicd_test_step_failed(core, &error))
}

#[cfg(test)]
#[path = "test/test_step.rs"]
mod tests;
