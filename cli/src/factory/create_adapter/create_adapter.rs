use crate::factory::create_adapter_for;
use mgc_types::Ecosystem;
use mgc_types::adapter::PackageAdapter;
use std::sync::Arc;

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
