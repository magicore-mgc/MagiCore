//! Library project runtime recognition across supported languages.
use super::super::{DetectedRuntime, project_text};
use std::path::Path;

pub(in crate::commands::optimizer) fn detect_lib_runtime(
    project_root: &Path,
) -> Vec<DetectedRuntime> {
    let mut runtimes = vec![];

    // Cargo treats src/lib.rs as an implicit library target even without [lib].
    // Cargo coi src/lib.rs là library target ngầm ngay cả khi không có [lib].
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(content) = project_text(project_root, "Cargo.toml")
        && (project_root.join("src/lib.rs").is_file()
            || content.contains("[lib]")
            || content.contains("crate-type"))
    {
        runtimes.push(DetectedRuntime::RustLib);
    }

    // Check for Go lib (go.mod) — kiểm tra thư viện Go
    if project_root.join("go.mod").exists() {
        runtimes.push(DetectedRuntime::GoLib);
    }

    // Check for Python lib (pyproject.toml with build-system or setup.py) — kiểm tra thư viện Python
    if project_root.join("pyproject.toml").exists() || project_root.join("setup.py").exists() {
        runtimes.push(DetectedRuntime::PythonLib);
    }

    // Check for TypeScript lib (package.json + tsconfig.json, no framework) — kiểm tra thư viện TypeScript
    if project_text(project_root, "package.json").is_some()
        && project_root.join("tsconfig.json").exists()
        && let Some(content) = project_text(project_root, "package.json")
    {
        // Not a web framework if no "react", "vue", "svelte", "next" etc. — không phải web framework
        if !content.contains("react")
            && !content.contains("vue")
            && !content.contains("svelte")
            && !content.contains("next")
            && !content.contains("vite")
        {
            runtimes.push(DetectedRuntime::TypeScriptLib);
        }
    }

    runtimes
}
