//! Behavior tests for the git_checkout owner.
//! Test hành vi cho owner git_checkout.

use super::super::*;
use mgc_resolver::protocols::ResolvedEntry;
use mgc_types::MgError;
use std::path::Path;

#[test]
fn git_materialization_remains_fail_closed_without_spawning_git() {
    let protocol = SwiftRegistryProtocol::with_registry("https://registry.test");
    let entry = ResolvedEntry {
        name: "scope/repo".into(),
        version: "1.0.0".into(),
        deps: Vec::new(),
        artifact_url: "https://git.test/scope/repo".into(),
        sha256: String::new(),
        extra_markers: Vec::new(),
    };

    let error = materialize_git_checkout(&protocol, &entry, None, Path::new("unused"))
        .expect_err("Git transport is not implemented and must stay unsupported");

    assert!(matches!(
        error,
        MgError::Unsupported {
            capability: "native Swift Git dependency materialization",
            ..
        }
    ));
}
