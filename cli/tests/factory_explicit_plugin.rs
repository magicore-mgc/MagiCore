//! Explicit plugin registration remains an intentional global factory override.
//! Giữ contract plugin đăng ký tường minh khi adapter native chuyển sang scope project.

#![allow(clippy::unwrap_used)]

#[cfg(feature = "web")]
#[test]
fn explicitly_registered_plugin_remains_factory_override() {
    use mgc_types::Ecosystem;
    use mgc_types::adapter::PackageAdapter;
    use std::sync::Arc;

    let adapter: Arc<dyn PackageAdapter> = Arc::new(
        mgc_web_adapter::WebAdapter::with_registry_chain(
            "https://explicit.invalid/registry".to_string(),
            None,
            vec![],
        )
        .unwrap(),
    );
    mgc_plugin::register(mgc_plugin::Plugin::from_adapter(adapter.clone())).unwrap();
    let project = tempfile::tempdir().unwrap();
    let selected = mgc::factory::create_adapter_for(
        project.path(),
        &Ecosystem::Web,
        Some("https://project.invalid/registry"),
        None,
        &[],
    )
    .unwrap();
    assert!(Arc::ptr_eq(&adapter, &selected));
}
