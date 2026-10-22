#![allow(clippy::unwrap_used)]
//! Presence is true only while the stored blob file exists.
//! Chỉ báo tồn tại khi file blob lưu trên đĩa còn tồn tại.

use crate::storage::RegistryStore;
use sha2::{Digest, Sha256};

#[tokio::test]
async fn missing_backing_file_reports_not_present() {
    let tmp = tempfile::tempdir().unwrap();
    let store = RegistryStore::new(tmp.path()).await.unwrap();
    let bytes = b"presence-owner";
    let digest = format!("sha256:{}", hex::encode(Sha256::digest(bytes)));

    store
        .put_oci_blob("ai/models", &digest, bytes)
        .await
        .unwrap();
    assert!(store.oci_blob_exists("ai/models", &digest).await.unwrap());
    let path = store
        .oci_blob_path_for_test("ai/models", &digest)
        .await
        .unwrap()
        .unwrap();
    std::fs::remove_file(path).unwrap();

    assert!(!store.oci_blob_exists("ai/models", &digest).await.unwrap());
}
