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

    // TypeScript/JavaScript library: a `package.json` that declares no web
    // framework.
    //
    // Requiring `tsconfig.json` as well excluded real TypeScript packages: a
    // library published with a `types` field and built by a bundler config
    // (`esbuild.config.mjs`, `bun build`) needs no `tsconfig.json`, and a plain
    // JavaScript library never has one — yet both are `lib` projects whose dev
    // loop runs Node. The framework check below is what separates "library" from
    // "web app"; a second build-config file adds no signal.
    // (`package.json` không framework là đủ. Yêu cầu thêm `tsconfig.json` loại
    // trừ package TS thật dùng `types` + bundler config; thư viện JS thuần cũng
    // không có file đó.)
    if let Some(content) = project_text(project_root, "package.json") {
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

    // Java: either build program selects the JVM lane; the adapter writes both
    // MAVEN_OPTS and GRADLE_OPTS so whichever one runs is tuned.
    // (Java: một trong hai chương trình build chọn lane JVM; adapter ghi cả
    // MAVEN_OPTS lẫn GRADLE_OPTS nên bên nào chạy cũng được tuning.)
    if project_root.join("pom.xml").exists()
        || project_root.join("build.gradle").exists()
        || project_root.join("build.gradle.kts").exists()
    {
        runtimes.push(DetectedRuntime::JvmLib);
    }

    // .NET: a project file is named after the project, so match the extension.
    // (.NET: file project đặt tên theo project nên khớp theo extension.)
    if crate::commands::core::shared::project_manifest::declares_dotnet_project(project_root) {
        runtimes.push(DetectedRuntime::DotNetLib);
    }

    // SwiftPM.
    if project_root.join("Package.swift").is_file() {
        runtimes.push(DetectedRuntime::SwiftLib);
    }

    runtimes
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
