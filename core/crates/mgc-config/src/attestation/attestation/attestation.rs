//! Project attestation orchestration / Điều phối attest project.

use std::path::Path;

use crate::attestation::{
    ATTESTATION_FILE, Attestation, AttestationStatus, attestation_dir, blake3_hex,
    load_or_create_keyring, previous_key_for, read_config_bytes, sign_and_store, verify,
};

/// Attest a project core: sign `(core, mgc.toml digest)` and store the
/// attestation outside the project. Idempotent when the live project
/// already matches the stored attestation (unless `force`).
/// (Chứng thực core project: ký và lưu ngoài project. Idempotent khi
/// project hiện tại đã khớp chứng thực cũ (trừ khi `force`).)
pub fn attest(
    project_root: &Path,
    core: &str,
    keyring_path: &Path,
    identities_dir: &Path,
    key_id: Option<&str>,
    force: bool,
    rotate: bool,
) -> anyhow::Result<Attestation> {
    let core = crate::project::ProjectConfig::canonical_core(core);
    if !crate::project::ProjectConfig::is_known_core(&core) {
        anyhow::bail!("cannot attest unknown core '{core}'");
    }
    let live_core = crate::project::ProjectConfig::read_core_marker_identity(project_root)?
        .ok_or_else(|| anyhow::anyhow!("project has no core marker to attest"))?;
    if live_core != core {
        anyhow::bail!(
            "refusing to attest core '{core}' because the live project marker is '{live_core}'"
        );
    }
    let slot = attestation_dir(identities_dir, project_root);
    let path = slot.join(ATTESTATION_FILE);
    if !force && !rotate && path.is_file() {
        match verify(project_root, identities_dir)? {
            AttestationStatus::Valid => {
                let text = std::fs::read_to_string(&path)
                    .map_err(|error| anyhow::anyhow!("cannot read attestation: {error}"))?;
                let existing: Attestation = serde_json::from_str(&text)
                    .map_err(|error| anyhow::anyhow!("cannot parse attestation: {error}"))?;
                return Ok(existing);
            }
            AttestationStatus::DomainMismatch(reason) | AttestationStatus::Mismatch(reason) => {
                anyhow::bail!(
                    "existing core attestation does not match ({reason}); pass --re-attest after intentional changes"
                );
            }
            AttestationStatus::Absent => {}
        }
    }
    let config = read_config_bytes(project_root)?;
    let digest = blake3_hex(&config);
    let mut keyring = load_or_create_keyring(keyring_path)?;
    let mut key_pair = if let Some(id) = key_id {
        keyring
            .get_key(id)
            .ok_or_else(|| anyhow::anyhow!("key not found in keyring: {id}"))?
            .clone()
    } else {
        keyring
            .default_key()
            .ok_or_else(|| anyhow::anyhow!("keyring has no default key — run `mgc trust init`"))?
            .clone()
    };
    let mut prev_keys = Vec::new();
    if rotate {
        // Determine the post-rotation signing key FIRST, then record the
        // previous key exactly when it differs — comparing against a key
        // that is about to be replaced would silently drop history.
        // (Xác định key ký sau-rotation TRƯỚC, rồi mới ghi key cũ khi
        // khác — so với key sắp bị thay sẽ mất lịch sử.)
        // A fresh key for rotation: generate only when the caller did not
        // name a key (named-key rotation switches to that key instead).
        // (Key mới cho rotation: chỉ sinh khi caller không chỉ định key.)
        if key_id.is_none() {
            let fresh = mgc_crypto::keyring::KeyPair::generate()
                .map_err(|error| anyhow::anyhow!("cannot generate key: {error}"))?;
            let fresh_id = fresh.key_id.clone();
            keyring.add_key(fresh);
            keyring
                .set_default(&fresh_id)
                .map_err(|error| anyhow::anyhow!("cannot set default key: {error}"))?;
            keyring
                .save(keyring_path)
                .map_err(|error| anyhow::anyhow!("cannot save keyring: {error}"))?;
            key_pair = keyring
                .get_key(&fresh_id)
                .ok_or_else(|| anyhow::anyhow!("fresh key vanished from keyring"))?
                .clone();
        }
        if path.is_file() {
            // Keep exactly one generation back — rotation chains must
            // not accumulate unbounded trust history.
            // (Chỉ giữ 1 đời key cũ — chuỗi rotation không phình.)
            prev_keys.extend(previous_key_for(&path, &key_pair.key_id));
        }
    }
    sign_and_store(
        project_root,
        &core,
        &digest,
        &key_pair,
        prev_keys,
        &slot,
        &path,
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
