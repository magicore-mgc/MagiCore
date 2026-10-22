// Registry empty-state and poisoned-lock behavior tests.
// Kiểm tra registry rỗng và cách xử lý lock bị poison.
use crate::PluginRegistry;
use mgc_types::Ecosystem;
use std::sync::Arc;

#[test]
fn new_registry_is_empty() {
    let registry = PluginRegistry::new();
    assert!(registry.registered().is_empty());
    assert!(registry.get(Ecosystem::Web).is_none());
}

#[test]
fn poisoned_registry_lock_fails_closed() {
    let registry = Arc::new(PluginRegistry::new());
    let other = Arc::clone(&registry);
    let result = std::thread::spawn(move || {
        let _guard = other.by_ecosystem.lock().expect("fresh registry lock");
        panic!("poison the registry lock for this test");
    })
    .join();

    assert!(result.is_err());
    assert!(registry.get(Ecosystem::Web).is_none());
    assert!(registry.registered().is_empty());
}
