//! Existing diagnostic text for Web bundler failures.
//! Diagnostic hiện hành cho lỗi Web bundler.

/// Formats esbuild failures with the existing user-facing prefix.
/// Format lỗi esbuild với prefix hiển thị hiện hành.
pub(in crate::bundler) fn esbuild_failed(messages: &[String]) -> anyhow::Error {
    anyhow::anyhow!("esbuild failed: {}", messages.join("\n"))
}

/// Formats node_modules link failures without changing the legacy diagnostic.
/// Format lỗi link node_modules, giữ nguyên diagnostic legacy.
pub(in crate::bundler) fn link_node_modules_failed(
    source: &std::path::Path,
    target: &std::path::Path,
    error: &dyn std::fmt::Display,
) -> anyhow::Error {
    anyhow::anyhow!(
        "failed to link node_modules for build from {} to {}: {}",
        source.display(),
        target.display(),
        error
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
