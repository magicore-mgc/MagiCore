use super::super::prelude::{RegistryStore, StatusCode};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub(in crate::oci::oci) fn validate_sha256_digest(digest: &str) -> Result<(), StatusCode> {
    let Some(hash) = digest.strip_prefix("sha256:") else {
        return Err(StatusCode::BAD_REQUEST);
    };
    if hash.len() != 64
        || !hash
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}

pub(in crate::oci::oci) fn validate_oci_reference(reference: &str) -> Result<(), StatusCode> {
    if reference.starts_with("sha256:") {
        return validate_sha256_digest(reference);
    }
    let valid_tag = !reference.is_empty()
        && reference.len() <= mgc_types::limits::OCI_TAG_MAX
        && reference
            .as_bytes()
            .first()
            .is_some_and(|first| first.is_ascii_alphanumeric() || *first == b'_')
        && reference
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_.-".contains(&byte));
    if valid_tag {
        Ok(())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

fn validate_oci_descriptor(value: &serde_json::Value) -> Result<(&str, &str, u64), StatusCode> {
    let media_type = value
        .get("mediaType")
        .and_then(serde_json::Value::as_str)
        .filter(|media_type| !media_type.is_empty())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let digest = value
        .get("digest")
        .and_then(serde_json::Value::as_str)
        .ok_or(StatusCode::BAD_REQUEST)?;
    validate_sha256_digest(digest)?;
    let size = value
        .get("size")
        .and_then(serde_json::Value::as_u64)
        .ok_or(StatusCode::BAD_REQUEST)?;
    Ok((media_type, digest, size))
}

pub(in crate::oci::oci) async fn validate_oci_manifest(
    store: &RegistryStore,
    repository: &str,
    body: &[u8],
) -> Result<(), StatusCode> {
    const MAX_INDEX_DEPTH: usize = 8;
    const OCI_IMAGE_MANIFEST: &str = "application/vnd.oci.image.manifest.v1+json";
    const DOCKER_IMAGE_MANIFEST: &str = "application/vnd.docker.distribution.manifest.v2+json";
    const OCI_IMAGE_INDEX: &str = "application/vnd.oci.image.index.v1+json";
    const DOCKER_MANIFEST_LIST: &str = "application/vnd.docker.distribution.manifest.list.v2+json";
    const MAX_INDEX_TREE_BYTES: u64 = 64 * 1024 * 1024;
    let supported = |media_type: &str| {
        matches!(
            media_type,
            OCI_IMAGE_MANIFEST | DOCKER_IMAGE_MANIFEST | OCI_IMAGE_INDEX | DOCKER_MANIFEST_LIST
        )
    };
    if body.len() as u64 > MAX_INDEX_TREE_BYTES {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    let mut pending = vec![(body.to_vec(), 0usize)];
    let mut manifest_metadata = HashMap::<String, (String, u64)>::new();
    let mut verified_blobs = HashMap::<String, u64>::new();
    let mut descriptor_budget = 10_000usize;
    let mut tree_bytes_budget = MAX_INDEX_TREE_BYTES - body.len() as u64;
    let mut verification_bytes_budget = crate::storage::max_oci_manifest_verify_bytes();
    while let Some((bytes, depth)) = pending.pop() {
        if depth > MAX_INDEX_DEPTH {
            return Err(StatusCode::BAD_REQUEST);
        }
        let manifest_size = bytes.len() as u64;
        let manifest: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|_| StatusCode::BAD_REQUEST)?;
        if manifest
            .get("schemaVersion")
            .and_then(serde_json::Value::as_u64)
            != Some(2)
        {
            return Err(StatusCode::BAD_REQUEST);
        }
        let media_type = manifest
            .get("mediaType")
            .and_then(serde_json::Value::as_str)
            .filter(|media_type| supported(media_type))
            .ok_or(StatusCode::BAD_REQUEST)?;
        let current_digest = format!("sha256:{}", hex::encode(Sha256::digest(&bytes)));
        manifest_metadata.insert(current_digest, (media_type.to_owned(), manifest_size));
        if matches!(media_type, OCI_IMAGE_INDEX | DOCKER_MANIFEST_LIST) {
            let descriptors = manifest
                .get("manifests")
                .and_then(serde_json::Value::as_array)
                .ok_or(StatusCode::BAD_REQUEST)?;
            if descriptors.len() > descriptor_budget {
                return Err(StatusCode::BAD_REQUEST);
            }
            for descriptor in descriptors {
                descriptor_budget -= 1;
                let (child_media_type, digest, expected_size) =
                    validate_oci_descriptor(descriptor)?;
                if !supported(child_media_type) {
                    return Err(StatusCode::BAD_REQUEST);
                }
                if let Some((known_media_type, known_size)) = manifest_metadata.get(digest) {
                    if known_media_type != child_media_type || *known_size != expected_size {
                        return Err(StatusCode::BAD_REQUEST);
                    }
                    continue;
                }
                let (child, stored_digest) = store
                    .get_oci_manifest_raw(repository, digest)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                    .ok_or(StatusCode::BAD_REQUEST)?;
                if child.len() as u64 != expected_size
                    || stored_digest != digest
                    || format!("sha256:{}", hex::encode(Sha256::digest(child.as_bytes()))) != digest
                {
                    return Err(StatusCode::BAD_REQUEST);
                }
                let child_value: serde_json::Value =
                    serde_json::from_str(&child).map_err(|_| StatusCode::BAD_REQUEST)?;
                if child_value
                    .get("mediaType")
                    .and_then(serde_json::Value::as_str)
                    != Some(child_media_type)
                {
                    return Err(StatusCode::BAD_REQUEST);
                }
                let child_size = child.len() as u64;
                if child_size > tree_bytes_budget {
                    return Err(StatusCode::PAYLOAD_TOO_LARGE);
                }
                tree_bytes_budget -= child_size;
                manifest_metadata
                    .insert(digest.to_owned(), (child_media_type.to_owned(), child_size));
                pending.push((child.into_bytes(), depth + 1));
            }
            continue;
        }

        let config = manifest.get("config").ok_or(StatusCode::BAD_REQUEST)?;
        let layers = manifest
            .get("layers")
            .and_then(serde_json::Value::as_array)
            .ok_or(StatusCode::BAD_REQUEST)?;
        let descriptor_count = layers.len().saturating_add(1);
        if descriptor_count > descriptor_budget {
            return Err(StatusCode::BAD_REQUEST);
        }
        descriptor_budget -= descriptor_count;
        for descriptor in std::iter::once(config).chain(layers) {
            let (_blob_media_type, digest, expected_size) = validate_oci_descriptor(descriptor)?;
            if let Some(verified_size) = verified_blobs.get(digest) {
                if *verified_size != expected_size {
                    return Err(StatusCode::BAD_REQUEST);
                }
                continue;
            }
            if expected_size > verification_bytes_budget {
                return Err(StatusCode::PAYLOAD_TOO_LARGE);
            }
            verification_bytes_budget -= expected_size;
            let verified = store
                .verify_oci_blob_descriptor(repository, digest, expected_size)
                .await
                .map_err(|error| {
                    tracing::error!("verify OCI blob {repository}/{digest}: {error:#}");
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            if !verified {
                return Err(StatusCode::BAD_REQUEST);
            }
            verified_blobs.insert(digest.to_owned(), expected_size);
        }
    }
    Ok(())
}
