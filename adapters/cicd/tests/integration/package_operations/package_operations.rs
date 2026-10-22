//! Package Operations behavior tests.
//! Test hành vi cho nhóm package operations.

use crate::integration::support::tmp;
use mgc_cicd_adapter::adapter_for;
use mgc_types::PackageName;
use mgc_types::adapter::{AddOptions, PackageAdapter};
use mgc_types::capabilities::{ContentStoreProvider, DependencyResolver};

pub async fn list_fails_closed_because_pipeline_files_are_not_packages() {
    let dir = tmp("list-not-packages");
    std::fs::write(dir.join(".gitlab-ci.yml"), "stages: [test]\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let error = adapter.list(&dir).await.unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            capability: "list",
            ..
        }
    ));
}

pub async fn install_fails_closed_because_no_package_semantics() {
    // CI/CD templates carry no registry packages: resolve AND install must
    // both fail closed with a typed Unsupported error — never Ok(default()).
    // Template CI/CD không có package registry: resolve VÀ install đều phải
    // fail-closed với error typed Unsupported — tuyệt đối không Ok(default()).
    let dir = tmp("install-fail-closed");
    std::fs::write(dir.join("wrangler.toml"), "name = \"w\"\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let manifest = a.parse_manifest(&dir).await.unwrap();

    let resolve_err = a.resolve(&manifest).await.unwrap_err();
    assert!(
        matches!(resolve_err, mgc_types::MgError::Unsupported { core, capability, .. }
            if core == "cicd" && capability == "resolve"),
        "resolve must be typed-unsupported, got: {resolve_err}"
    );

    let graph = mgc_types::ResolvedGraph::default();
    let install_err = a
        .install(&graph, &dir, Default::default())
        .await
        .unwrap_err();
    assert!(
        matches!(install_err, mgc_types::MgError::Unsupported { core, capability, .. }
            if core == "cicd" && capability == "install"),
        "install must be typed-unsupported, got: {install_err}"
    );
}

pub async fn add_fails_closed_with_error_mentioning_deploy() {
    let dir = tmp("add-fail");
    std::fs::write(dir.join("wrangler.toml"), "name = \"w\"\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let name = PackageName::new("my-service").unwrap();
    let err = a
        .add(&dir, &name, None, AddOptions::default())
        .await
        .unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("deploy") || msg.contains("cicd"),
        "error must mention deploy: {msg}"
    );
}

pub async fn remove_fails_closed() {
    let dir = tmp("remove-fail");
    std::fs::write(dir.join("wrangler.toml"), "name = \"w\"\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let name = PackageName::new("svc").unwrap();
    assert!(a.remove(&dir, &name).await.is_err());
}

pub async fn update_fails_closed() {
    let dir = tmp("update-fail");
    std::fs::write(dir.join("wrangler.toml"), "name = \"w\"\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    assert!(a.update(&dir, None).await.is_err());
}
