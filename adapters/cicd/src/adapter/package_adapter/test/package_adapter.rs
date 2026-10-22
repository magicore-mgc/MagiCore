#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Preserve manifest projection and fail-closed package listing.
//! Giữ projection manifest và list package fail-closed.

use crate::adapter::CicdAdapter;
use crate::provider::CicdProvider;
use mgc_types::MgError;
use mgc_types::adapter::PackageAdapter;
use tempfile::tempdir;

#[tokio::test]
async fn manifest_uses_project_directory_name() {
    let temp = tempdir().unwrap();
    let project = temp.path().join("cicd-fixture");
    std::fs::create_dir(&project).unwrap();
    let adapter = CicdAdapter {
        provider: CicdProvider::GithubActions,
    };
    let manifest = adapter.parse_manifest(&project).await.unwrap();
    assert!(manifest.name.contains("cicd-fixture"));
}

#[tokio::test]
async fn workflow_files_are_not_listed_as_installed_packages() {
    let project = tempdir().unwrap();
    let adapter = CicdAdapter {
        provider: CicdProvider::GithubActions,
    };
    let result = adapter.list(project.path()).await;
    assert!(matches!(
        result,
        Err(MgError::Unsupported {
            core: "cicd",
            capability: "list",
            ..
        })
    ));
}
