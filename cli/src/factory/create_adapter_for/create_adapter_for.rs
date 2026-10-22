use std::sync::Arc;

use mgc_plugin::Plugin;
use mgc_types::Ecosystem;
use mgc_types::adapter::PackageAdapter;

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
