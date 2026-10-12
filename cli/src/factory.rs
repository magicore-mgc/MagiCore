use std::sync::Arc;

use mgc_plugin::Plugin;
use mgc_types::Ecosystem;
use mgc_types::adapter::PackageAdapter;

/// Available cores in this build (for init menu filtering)
#[allow(clippy::vec_init_then_push)]
pub fn available_cores() -> Vec<(&'static str, &'static str)> {
    let mut cores = Vec::new();
    #[cfg(feature = "web")]
    cores.push(("web", "🌐  Web application"));
    #[cfg(feature = "lib")]
    cores.push(("lib", "📚  Library (ts / rust / python)"));
    #[cfg(feature = "ai")]
    cores.push(("ai", "🤖  AI agent / model runtime"));
    #[cfg(feature = "app")]
    cores.push(("app", "📱  App (flutter / kotlin / swift)"));
    #[cfg(feature = "game")]
    cores.push(("game", "🎮  Game (bevy / godot / unity / unreal)"));
    #[cfg(feature = "iot")]
    cores.push(("iot", "📡  IoT (esp32-rust / platformio / zephyr)"));
    #[cfg(feature = "clo")]
    cores.push(("clo", "☁️  Cloud infrastructure"));
    #[cfg(feature = "cicd")]
    cores.push(("cicd", "🚦  CI/CD pipeline"));
    #[cfg(feature = "hardware")]
    cores.push((
        "hardware",
        "⚙️  Hardware (optimizer/bench — CPU/RAM tuning)",
    ));
    cores
}

/// Bind native adapters to each project; explicit global plugins remain overrides.
/// Adapter native gắn registry/ngôn ngữ của project, bridge không làm cache toàn core.
pub fn create_adapter(
    ecosystem: &Ecosystem,
    registry_url: Option<&str>,
    token: Option<&str>,
) -> anyhow::Result<Arc<dyn PackageAdapter>> {
    create_adapter_for(
        &std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?,
        ecosystem,
        registry_url,
        token,
        &[],
    )
}

/// Tạo adapter gắn với project root rõ ràng (mix core: workspace target).
/// `fallbacks`: (url, token) chain — chỉ dùng khi primary 404/network/5xx.
pub fn create_adapter_for(
    root: &std::path::Path,
    ecosystem: &Ecosystem,
    registry_url: Option<&str>,
    token: Option<&str>,
    fallbacks: &[(String, Option<String>)],
) -> anyhow::Result<Arc<dyn PackageAdapter>> {
    let _ = root;
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(plugin) = mgc_plugin::global().get(*ecosystem)
        && !plugin.is_context_scoped()
        && let Some(adapter) = plugin.as_adapter()
    {
        return Ok(adapter);
    }

    let adapter: Arc<dyn PackageAdapter> = match ecosystem {
        #[cfg(feature = "web")]
        // P0-4 (2026-09-15): WebAdapter constructors are fallible (typed
        // registry-URL guard, fail-closed kept) — `?` surfaces the error.
        // (P0-4: constructor WebAdapter có thể lỗi (guard URL typed, vẫn
        // fail-closed) — `?` đưa lỗi lên.)
        Ecosystem::Web => Arc::new(match (registry_url, token) {
            (Some(url), _) => mgc_web_adapter::WebAdapter::with_registry_chain(
                url.to_string(),
                token.map(str::to_string),
                fallbacks.to_vec(),
            )?,
            _ => mgc_web_adapter::WebAdapter::new()?,
        }),
        #[cfg(not(feature = "web"))]
        Ecosystem::Web => return Err(crate::error::core_not_in_build("web")),
        #[cfg(feature = "game")]
        Ecosystem::Game => Arc::new(
            mgc_game_adapter::adapter_for(root)
                .ok_or_else(|| crate::error::detect_core_failed("game"))?,
        ),
        #[cfg(not(feature = "game"))]
        Ecosystem::Game => return Err(crate::error::core_not_in_build("game")),
        #[cfg(feature = "ai")]
        Ecosystem::Ai => Arc::new(
            mgc_ai_adapter::adapter_for(root)
                .ok_or_else(|| crate::error::detect_core_failed("ai"))?,
        ),
        #[cfg(not(feature = "ai"))]
        Ecosystem::Ai => return Err(crate::error::core_not_in_build("ai")),
        #[cfg(feature = "clo")]
        Ecosystem::Cloud => Arc::new(
            // Result<Option<_>> now (P0-4): `?` first for the typed
            // registry-URL error, then Ok(None) → detect failure.
            // (Giờ là Result<Option<_>> (P0-4): `?` trước cho lỗi
            // registry-URL typed, rồi Ok(None) → lỗi detect.)
            mgc_cloud_adapter::adapter_for(root)?
                .ok_or_else(|| crate::error::detect_core_failed("clo"))?,
        ),
        #[cfg(not(feature = "clo"))]
        Ecosystem::Cloud => return Err(crate::error::core_not_in_build("clo")),
        #[cfg(feature = "cicd")]
        Ecosystem::Cicd => Arc::new(
            mgc_cicd_adapter::adapter_for(root)
                .ok_or_else(|| crate::error::detect_core_failed("cicd"))?,
        ),
        #[cfg(not(feature = "cicd"))]
        Ecosystem::Cicd => return Err(crate::error::core_not_in_build("cicd")),
        #[cfg(feature = "iot")]
        Ecosystem::Iot => Arc::new(
            mgc_iot_adapter::adapter_for(root)
                .ok_or_else(|| crate::error::detect_core_failed("iot"))?,
        ),
        #[cfg(not(feature = "iot"))]
        Ecosystem::Iot => return Err(crate::error::core_not_in_build("iot")),
        #[cfg(feature = "app")]
        Ecosystem::App => Arc::new(
            mgc_app_adapter::adapter_for(root)
                .ok_or_else(|| crate::error::detect_core_failed("app"))?,
        ),
        #[cfg(not(feature = "app"))]
        Ecosystem::App => return Err(crate::error::core_not_in_build("app")),
        #[cfg(feature = "hardware")]
        Ecosystem::Hardware => Arc::new(
            mgc_hardware_adapter::adapter_for(root)
                .ok_or_else(|| crate::error::detect_core_failed("hardware"))?,
        ),
        #[cfg(not(feature = "hardware"))]
        Ecosystem::Hardware => return Err(crate::error::core_not_in_build("hardware")),
        #[cfg(feature = "lib")]
        Ecosystem::Lib => Arc::new(
            // Result<Option<_>> now (P0-4): `?` for the typed registry-URL
            // error, then Ok(None) → detect failure.
            // (Giờ là Result<Option<_>> (P0-4): `?` cho lỗi registry-URL
            // typed, rồi Ok(None) → lỗi detect.)
            mgc_lib_adapter::adapter_for_with_chain(
                root,
                registry_url.map(str::to_string),
                token.map(str::to_string),
                fallbacks,
            )?
            .ok_or_else(|| crate::error::detect_core_failed("lib"))?,
        ),
        #[cfg(not(feature = "lib"))]
        Ecosystem::Lib => return Err(crate::error::core_not_in_build("lib")),
    };

    // Register a context-scoped native bridge; do not reuse its project-bound adapter.
    // Registry giữ bridge native, factory vẫn bind lại đúng root/config mỗi lượt.
    let _ = mgc_plugin::register(Plugin::from_project_adapter(adapter.clone()));
    Ok(adapter)
}

/// Keep scoped credentials through native npm construction.
/// Caller đã chọn chain; không làm phẳng token trước khi dựng transport.
pub fn create_adapter_for_scoped(
    root: &std::path::Path,
    ecosystem: &Ecosystem,
    entries: Vec<mgc_config::chain::ScopedRegistry>,
) -> anyhow::Result<Arc<dyn PackageAdapter>> {
    if let Some(plugin) = mgc_plugin::global().get(*ecosystem)
        && !plugin.is_context_scoped()
        && let Some(adapter) = plugin.as_adapter()
    {
        return Ok(adapter);
    }
    let adapter: Arc<dyn PackageAdapter> = match ecosystem {
        #[cfg(feature = "web")]
        Ecosystem::Web => Arc::new(mgc_web_adapter::WebAdapter::with_scoped_registry_chain(
            entries,
        )?),
        #[cfg(feature = "lib")]
        Ecosystem::Lib => Arc::new(
            mgc_lib_adapter::adapter_for_with_scoped_chain(root, entries)?
                .ok_or_else(|| crate::error::detect_core_failed("lib"))?,
        ),
        _ => return create_adapter_for(root, ecosystem, None, None, &[]),
    };
    let _ = mgc_plugin::register(Plugin::from_project_adapter(adapter.clone()));
    Ok(adapter)
}

/// Check whether this project actually consumes npm registry credentials.
/// Không đọc npmrc cho core/ngôn ngữ ngoài npm chỉ vì CLI dùng context chung.
pub fn project_uses_npm_registry(root: &std::path::Path, ecosystem: &Ecosystem) -> bool {
    let _ = root;
    match ecosystem {
        #[cfg(feature = "web")]
        Ecosystem::Web => {
            // A JS manifest owns npm even in a polyglot project.
            // Manifest native rõ ràng chỉ tránh npm khi project không có package.json.
            root.join("package.json").exists()
                || mgc_lib_adapter::detect_language(root)
                    .is_none_or(|language| language == mgc_lib_adapter::LibLanguage::Ts)
        }
        #[cfg(feature = "lib")]
        Ecosystem::Lib => {
            mgc_lib_adapter::detect_language(root) == Some(mgc_lib_adapter::LibLanguage::Ts)
        }
        _ => false,
    }
}
