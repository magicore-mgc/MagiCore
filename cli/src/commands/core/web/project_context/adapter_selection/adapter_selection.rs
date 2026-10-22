// Select a native backend language and adapter — chọn ngôn ngữ backend và adapter.
// Backend language and adapter selection.
use super::*;

/// Non-JS backend language of a web project, when `package.json` is
/// absent (create-web backend scaffolds: gin/echo/fiber → Go,
/// axum/actix → Rust, fastapi/django/flask → Python, spring/quarkus →
/// Java, dotnet-* → .NET). `None` = JS/TS lane owns it (or nothing
/// detectable — current behavior runs and fails honestly downstream).
/// Detected from manifest markers first, then mgc.toml frameworks[].
/// (Ngôn ngữ backend non-JS của project web khi không có package.json.)
#[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
pub(in crate::commands::core::web::implementation) fn web_backend_language(
    root: &Path,
) -> Option<mgc_lib_adapter::LibLanguage> {
    use mgc_lib_adapter::LibLanguage;
    if root.join("package.json").exists() {
        return None;
    }
    if root.join("go.mod").exists() {
        return Some(LibLanguage::Go);
    }
    if root.join("Cargo.toml").exists() {
        return Some(LibLanguage::Rust);
    }
    if root.join("pyproject.toml").exists()
        || root.join("setup.py").exists()
        || root.join("requirements.txt").exists()
    {
        return Some(LibLanguage::Python);
    }
    if root.join("pom.xml").exists()
        || root.join("build.gradle").exists()
        || root.join("build.gradle.kts").exists()
    {
        return Some(LibLanguage::Java);
    }
    if std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .any(|e| e.path().extension().is_some_and(|ext| ext == "csproj"))
    {
        return Some(LibLanguage::DotNet);
    }
    let mgc_toml =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
            .ok()??;
    let value: toml::Value = toml::from_str(&mgc_toml).ok()?;
    let frameworks: Vec<&str> = value
        .get("frameworks")
        .and_then(|f| f.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();
    frameworks.into_iter().find_map(|framework| {
        let base = framework.split('/').next_back().unwrap_or(framework);
        match base {
            "gin" | "echo" | "fiber" | "go" => Some(LibLanguage::Go),
            "axum" | "actix-web" | "actix" | "rust" => Some(LibLanguage::Rust),
            "fastapi" | "django" | "flask" | "python" => Some(LibLanguage::Python),
            "spring-boot" | "spring" | "quarkus" | "java" => Some(LibLanguage::Java),
            "dotnet-webapi" | "dotnet-minimal" | "dotnet" | "csharp" => Some(LibLanguage::DotNet),
            _ => None,
        }
    })
}
/// Build the native language adapter for a web-owned backend project.
/// Dependency ownership is evaluated by the lib language lane, while the
/// project identity and adapter security policy remain `web`.
/// Dựng adapter ngôn ngữ native cho backend do web sở hữu. Gate dùng lane
/// ngôn ngữ lib; project identity và security policy vẫn thuộc `web`.
#[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
pub(in crate::commands::core::web::implementation) fn web_backend_adapter_for_operation(
    root: &Path,
    language: mgc_lib_adapter::LibLanguage,
    operation: crate::commands::dep_gate::DepOp,
    compat_runtime: Option<&str>,
) -> Result<Arc<dyn PackageAdapter>> {
    let compat = crate::commands::dep_gate::from_dep_flag(compat_runtime)?;
    // Gate the same language and manifest that web_backend_language selected
    // for the adapter; Lib's generic detector has a different probe order.
    // (Gate theo đúng ngôn ngữ/manifest Web đã chọn; detector Lib tổng quát
    // có thứ tự dò khác.)
    let context = crate::commands::dep_gate::DepContext::new(
        "lib",
        Some(language.ecosystem()),
        None,
        None,
        operation,
    )
    .with_manifest_format(mgc_lib_adapter::dependency_manifest_format(root, language));
    crate::commands::dep_gate::gate(
        &context,
        None,
        &compat,
        Some(&root.join(".magicore").join("exec.log")),
    )?;
    let adapter = mgc_lib_adapter::adapter_for_web_backend(language, root)?;
    Ok(Arc::new(adapter))
}
pub(in crate::commands::core::web::implementation) fn web_adapter(
    root: &Path,
) -> Result<Arc<dyn PackageAdapter>> {
    let started_at = std::time::Instant::now();
    let config = mgc_config::project::ProjectConfig::load(root)?;
    let chain = mgc_config::chain::registry_chain_scoped_checked(Some(root), config.as_ref())?;
    let adapter = crate::factory::create_adapter_for_scoped(root, &Ecosystem::Web, chain)?;
    web_command_profile_mark("web_adapter", started_at);
    Ok(adapter)
}
