#![allow(clippy::unwrap_used)]
//! Reads stored bytes only when their declared digest still matches.
//! Chỉ đọc bytes đã lưu khi digest khai báo vẫn khớp.

use crate::storage::RegistryStore;
use sha2::{Digest, Sha256};

#[tokio::test]
async fn valid_blob_round_trips_through_integrity_checked_read() {
    let tmp = tempfile::tempdir().unwrap();
    let store = RegistryStore::new(tmp.path()).await.unwrap();
    let bytes = b"oci-read-owner";
    let digest = format!("sha256:{}", hex::encode(Sha256::digest(bytes)));

    store
        .put_oci_blob("ai/models", &digest, bytes)
        .await
        .unwrap();
    assert_eq!(
        store.get_oci_blob("ai/models", &digest).await.unwrap(),
        Some(bytes.to_vec())
    );
}
