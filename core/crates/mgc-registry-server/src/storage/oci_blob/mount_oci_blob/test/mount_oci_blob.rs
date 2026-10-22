#![allow(clippy::unwrap_used)]
//! Mounts a stored blob into another repository and handles a missing source.
//! Mount blob đã lưu sang repository khác và xử lý nguồn không tồn tại.

use crate::storage::RegistryStore;
use sha2::{Digest, Sha256};

#[tokio::test]
async fn cross_repository_mount_copies_blob_and_missing_source_is_false() {
    let tmp = tempfile::tempdir().unwrap();
    let store = RegistryStore::new(tmp.path()).await.unwrap();
    let bytes = b"mount-owner";
    let digest = format!("sha256:{}", hex::encode(Sha256::digest(bytes)));

    assert!(
        !store
            .mount_oci_blob("missing/repo", &digest, "destination/repo")
            .await
            .unwrap()
    );
    store
        .put_oci_blob("source/repo", &digest, bytes)
        .await
        .unwrap();
    assert!(
        store
            .mount_oci_blob("source/repo", &digest, "destination/repo")
            .await
            .unwrap()
    );
    assert_eq!(
        store
            .get_oci_blob("destination/repo", &digest)
            .await
            .unwrap(),
        Some(bytes.to_vec())
    );
}
