//! Package Operations behavior tests.
//! Test hành vi cho nhóm package operations.

use crate::integration::support::tmp;
use mgc_cloud_adapter::adapter_for;
use mgc_types::PackageName;
use mgc_types::adapter::{AddOptions, PackageAdapter};
use mgc_types::capabilities::{ContentStoreProvider, DependencyResolver};

pub async fn parse_manifest_uses_dir_name_for_terraform() {
    let dir = tmp("my-infra-project");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    let a = adapter_for(&dir).unwrap().unwrap();
    let manifest = a.parse_manifest(&dir).await.unwrap();
    assert!(manifest.name.contains("my-infra-project"));
}

pub async fn terraform_package_adapter_install_fails_closed_without_spawning_terraform() {
    let dir = tmp("install-tf");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    let a = adapter_for(&dir).unwrap().unwrap();
    let _manifest = a.parse_manifest(&dir).await.unwrap();
    let graph = mgc_types::ResolvedGraph::default();
    let result = a.install(&graph, &dir, Default::default()).await;
    let error = result.expect_err("PackageAdapter must not spawn Terraform");
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            core: "cloud",
            capability: "install",
            ..
        }
    ));
}

pub async fn add_fails_closed_for_terraform() {
    let dir = tmp("add-tf");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    let a = adapter_for(&dir).unwrap().unwrap();
    let name = PackageName::new("aws_s3_bucket").unwrap();
    let err = a
        .add(&dir, &name, None, AddOptions::default())
        .await
        .unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("direct adapter mutation is disabled"),
        "adapter mutation must fail closed and require the CLI gateway: {msg}"
    );
}

pub async fn remove_fails_closed_for_terraform() {
    let dir = tmp("remove-tf");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    let a = adapter_for(&dir).unwrap().unwrap();
    let name = PackageName::new("aws_s3_bucket").unwrap();
    assert!(a.remove(&dir, &name).await.is_err());
}

pub async fn update_fails_closed_for_terraform() {
    let dir = tmp("update-tf");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    let a = adapter_for(&dir).unwrap().unwrap();
    assert!(a.update(&dir, None).await.is_err());
}

pub async fn terraform_list_fails_closed_instead_of_returning_an_empty_native_list() {
    let dir = tmp("list-tf");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    let adapter = adapter_for(&dir).unwrap().unwrap();
    let error = adapter.list(&dir).await.unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            core: "cloud",
            capability: "list",
            ..
        }
    ));
    assert!(
        !adapter
            .capabilities()
            .contains(&mgc_types::capabilities::Capability::ContentStoreProvider)
    );
    assert!(adapter.probe_content_store().is_err());
}

pub async fn pulumi_without_package_json_list_fails_closed() {
    let dir = tmp("list-pulumi-no-package");
    std::fs::write(dir.join("Pulumi.yaml"), "name: infra\nruntime: python\n").unwrap();
    let adapter = adapter_for(&dir).unwrap().unwrap();
    let error = adapter.list(&dir).await.unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            core: "cloud",
            capability: "list",
            ..
        }
    ));
}
