#![allow(clippy::unwrap_used)]
//! Rejects bytes whose declared OCI digest does not match their content.
//! Từ chối bytes có digest OCI khai báo không khớp nội dung.

use crate::storage::RegistryStore;
use sha2::{Digest, Sha256};

#[tokio::test]
async fn mismatched_digest_is_not_persisted() {
    let tmp = tempfile::tempdir().unwrap();
    let store = RegistryStore::new(tmp.path()).await.unwrap();
    let declared = format!("sha256:{}", hex::encode(Sha256::digest(b"expected")));

    let error = store
        .put_oci_blob("ai/models", &declared, b"different")
        .await
        .unwrap_err();
    let actual = format!("sha256:{}", hex::encode(Sha256::digest(b"different")));
    assert_eq!(
        error.to_string(),
        format!(
            concat!(
                "oci digest mismatch: declared '{declared}' ",
                "but bytes hash to '{actual}'"
            ),
            declared = declared,
            actual = actual
        )
    );
    assert!(
        store
            .oci_blob_path_for_test("ai/models", &declared)
            .await
            .unwrap()
            .is_none()
    );
}
