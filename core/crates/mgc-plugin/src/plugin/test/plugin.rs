// Public Plugin API compatibility tests.
// Test tương thích API Plugin công khai.
use crate::Plugin;
use mgc_types::Ecosystem;
use mgc_types::adapter::PackageAdapter;
use std::sync::Arc;

#[test]
fn public_plugin_api_signatures_are_stable() {
    let _: fn(Arc<dyn PackageAdapter>) -> Plugin = Plugin::from_adapter;
    let _: fn(Arc<dyn PackageAdapter>) -> Plugin = Plugin::from_project_adapter;
    let _: fn(&Plugin) -> bool = Plugin::is_context_scoped;
    let _: fn(&Plugin) -> Ecosystem = Plugin::ecosystem;
    let _: fn(&Plugin) -> Option<Arc<dyn PackageAdapter>> = Plugin::as_adapter;
}
