use super::super::*;

#[test]
fn layer_identity_rejects_paths_that_escape_the_cache_on_any_platform() {
    for name in [
        "",
        "../layer",
        "/absolute",
        "a/../../b",
        "a//b",
        "C:\\layer",
        "a\\b",
        "a:b",
        "a\nb",
    ] {
        assert!(validate_template_identity("web", name).is_err(), "{name:?}");
    }
    assert!(validate_template_identity("unknown", "layer").is_err());
    for (core, name) in [
        ("web", "web/backend/go/gin"),
        ("ai", "python-agent"),
        ("app", "flutter"),
        ("lib", "rust"),
    ] {
        validate_template_identity(core, name).unwrap();
    }
}

#[test]
fn nested_layer_identity_round_trips_to_the_processor_cache() {
    assert_eq!(
        layer_rel("web", "backend/node/probe"),
        "web/backend/node/probe"
    );
    assert_eq!(
        registry_package_name("web", "web/backend/node/probe"),
        "mgc-create-web-backend-node-probe"
    );
}
