use super::super::run_tool_with_env;
use anyhow::Result;
use std::path::Path;

fn swift_package_args() -> Vec<String> {
    ["run", "--skip-update", "--disable-automatic-resolution"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

/// iOS simulator dev requires macOS + Xcode; a SwiftPM CLI package can use
/// `swift run` on macOS without an Xcode project or simulator.
/// (Dev iOS Simulator cần macOS + Xcode; SwiftPM CLI có thể chạy `swift run`
/// trên macOS mà không cần Xcode project hay simulator.)
pub(in crate::commands::core::dev) fn run_swift_package(
    root: &Path,
    dry_run: bool,
    env: Option<Vec<(String, String)>>,
) -> Result<()> {
    if dry_run {
        mgc_ui::info("[dry-run] would run: swift run");
        return Ok(());
    }
    mgc_ui::info("App dev (Swift): swift run");
    run_tool_with_env(root, "swift", &swift_package_args(), env)
}

#[cfg(test)]
#[path = "test/swift_package.rs"]
mod tests;
