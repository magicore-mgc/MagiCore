use super::*;

#[test]
fn registry_base_urls_are_wellformed() {
    let p = SwiftRegistryProtocol::with_registry("http://127.0.0.1:1/");
    assert_eq!(
        p.registry_url_for("scope/lib", "1.0.0.zip"),
        "http://127.0.0.1:1/scope/lib/1.0.0.zip"
    );
}
