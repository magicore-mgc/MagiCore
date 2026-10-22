//! Web runtime marker recognition.
use super::super::{DetectedRuntime, detect_package_manager, project_text};
use std::path::Path;

pub(in crate::commands::optimizer) fn detect_web_runtime(
    project_root: &Path,
) -> Vec<DetectedRuntime> {
    let mut runtimes = vec![];

    // Check for Deno (deno.json or deno.jsonc) — kiểm tra Deno
    if project_root.join("deno.json").exists() || project_root.join("deno.jsonc").exists() {
        runtimes.push(DetectedRuntime::Deno);
        return runtimes; // Deno is exclusive — Deno là độc quyền
    }

    // Check for Bun (bun.lockb or bunfig.toml) — kiểm tra Bun
    if project_root.join("bun.lockb").exists() || project_root.join("bunfig.toml").exists() {
        runtimes.push(DetectedRuntime::Bun);
        return runtimes; // Bun is exclusive — Bun là độc quyền
    }

    // Check for Node.js (package.json) — kiểm tra Node.js
    if project_text(project_root, "package.json").is_some() {
        let pm = detect_package_manager(project_root);
        runtimes.push(DetectedRuntime::NodeJs {
            package_manager: pm,
        });
    }

    runtimes
}
