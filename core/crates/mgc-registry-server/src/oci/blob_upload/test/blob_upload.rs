use super::super::*;
use sha2::{Digest, Sha256};

#[tokio::test]
async fn upload_finalization_publishes_complete_chunk_sequence() {
    let tmp = tempfile::tempdir().unwrap();
    let store = RegistryStore::new(tmp.path()).await.unwrap();
    let repository = "ai/models/weights";
    let uuid = "test-upload";
    let bytes = b"first patch and final put bytes";
    let split = 10;
    store.create_oci_upload(repository, uuid).await.unwrap();
    store
        .append_oci_upload(repository, uuid, 0, &bytes[..split])
        .await
        .unwrap();
    let offset = store
        .oci_upload_offset(repository, uuid)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(offset, split as i64);
    store
        .append_oci_upload(repository, uuid, offset, &bytes[split..])
        .await
        .unwrap();
    let digest = format!("sha256:{}", hex::encode(Sha256::digest(bytes)));

    store
        .finalize_oci_upload(repository, uuid, &digest)
        .await
        .unwrap();

    assert_eq!(
        store.get_oci_blob(repository, &digest).await.unwrap(),
        Some(bytes.to_vec())
    );
    assert!(
        store
            .oci_upload_path(repository, uuid)
            .await
            .unwrap()
            .is_none()
    );
}
