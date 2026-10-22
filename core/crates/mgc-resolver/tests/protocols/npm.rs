//! Public integration tests for the npm slot's fail-closed contract.
//! Test integration public cho hợp đồng fail-closed của slot npm.

#![allow(clippy::unwrap_used)]
use mgc_resolver::protocols::{NpmProtocol, RegistryProtocol, ResolvedEntry};

fn assert_unsupported(err: &mgc_types::MgError, core: &str, capability: &str) {
    match err {
        mgc_types::MgError::Unsupported {
            core: actual_core,
            capability: actual_capability,
            ..
        } => {
            assert_eq!(actual_core.to_string(), core);
            assert_eq!(actual_capability.to_string(), capability);
        }
        other => panic!("expected MgError::Unsupported, got: {other:?}"),
    }
}
#[tokio::test]
async fn npm_resolve_fails_closed() {
    let err = NpmProtocol
        .resolve("react", "^18")
        .await
        .expect_err("test operation must fail");
    assert_unsupported(&err, "web", "resolve");
}

#[tokio::test]
async fn npm_download_fails_closed() {
    let entry = ResolvedEntry {
        name: "react".to_string(),
        version: "18.0.0".to_string(),
        deps: vec![],
        artifact_url: "https://registry.npmjs.org/react/-/react-18.0.0.tgz".to_string(),
        sha256: String::new(),
        extra_markers: vec![],
    };
    let err = NpmProtocol
        .download(&entry)
        .await
        .expect_err("test operation must fail");
    assert_unsupported(&err, "web", "fetch");
}
