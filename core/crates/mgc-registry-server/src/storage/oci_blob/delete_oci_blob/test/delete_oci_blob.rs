#![allow(clippy::unwrap_used)]
//! Removes the blob row and backing file when deletion succeeds.
//! Xóa row và file blob khi thao tác gỡ file thành công.

use crate::storage::RegistryStore;
use sha2::{Digest, Sha256};

#[tokio::test]
async fn delete_removes_blob_and_database_mapping() {
    let tmp = tempfile::tempdir().unwrap();
    let store = RegistryStore::new(tmp.path()).await.unwrap();
    let bytes = b"delete-owner";
    let digest = format!("sha256:{}", hex::encode(Sha256::digest(bytes)));

    store
        .put_oci_blob("ai/models", &digest, bytes)
        .await
        .unwrap();
    let path = store
        .oci_blob_path_for_test("ai/models", &digest)
        .await
        .unwrap()
        .unwrap();
    assert!(store.delete_oci_blob("ai/models", &digest).await.unwrap());
    assert!(!std::path::Path::new(&path).exists());
    assert!(
        store
            .get_oci_blob("ai/models", &digest)
            .await
            .unwrap()
            .is_none()
    );
}
