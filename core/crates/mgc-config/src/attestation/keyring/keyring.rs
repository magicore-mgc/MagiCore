//! Attestation keyring loading / Nạp keyring attestation.

use std::path::Path;

/// Load (or create) a file keyring at an explicit path — mirrors
/// `Keyring::init_if_not_exists` without pinning the home default, so
/// tests can isolate keys in tempdirs.
/// (Nạp (hoặc tạo) keyring tại path tường minh — như
/// `init_if_not_exists` nhưng không ghim home default để test cô lập.)
pub fn load_or_create_keyring(keyring_path: &Path) -> anyhow::Result<mgc_crypto::keyring::Keyring> {
    if keyring_path.exists() {
        return mgc_crypto::keyring::Keyring::load(keyring_path)
            .map_err(|error| anyhow::anyhow!("cannot load keyring: {error}"));
    }
    // Do NOT create parent dirs here: Keyring::save validates the path
    // before creating parents, and a pre-created parent changes the
    // canonicalization outcome (symlinked TMPDIR roots). Let save own it.
    // (Không tạo dir cha trước: save validate path trước khi tạo dir.)
    let mut keyring = mgc_crypto::keyring::Keyring::new();
    let key_pair = mgc_crypto::keyring::KeyPair::generate()
        .map_err(|error| anyhow::anyhow!("cannot generate key: {error}"))?;
    keyring.add_key(key_pair);
    keyring
        .save(keyring_path)
        .map_err(|error| anyhow::anyhow!("cannot save keyring: {error}"))?;
    Ok(keyring)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
