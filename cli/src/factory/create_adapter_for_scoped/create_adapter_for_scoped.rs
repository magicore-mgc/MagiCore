use crate::factory::create_adapter_for;
use mgc_plugin::Plugin;
use mgc_types::Ecosystem;
use mgc_types::adapter::PackageAdapter;
use std::sync::Arc;

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
