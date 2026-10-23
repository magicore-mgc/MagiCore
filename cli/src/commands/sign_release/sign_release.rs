//! `sign-release-manifest` — sign a release manifest with the Ed25519
//! release key through the REVIEWED `ring` backend (mgc-crypto, locked
//! in Cargo.lock) — no pip, no network at signing time (P0: the old
//! python path `pip install`ed cryptography on every release).
//!
//! Contract (mirrors scripts/sign-release-manifest.py, which stays as
//! the independent cross-check oracle):
//! - key from `--key-hex` or `MGC_RELEASE_SIGNING_KEY` (64 hex chars).
//! - no key: loud warning + Ok (internal preview / transitional).
//! - no key + `MGC_RELEASE_REQUIRE_SIGNED=1`: hard Err (stable releases
//!   must never ship unsigned — the publish job sets this).
//! - the fresh signature is self-verified BEFORE writing `<manifest>.sig`.
//!
//! (Ký manifest release bằng backend `ring` đã review — không pip,
//! không mạng lúc ký.)

use anyhow::{Context, Result};

/// Sign `manifest_path` (default `manifest.json`), writing hex + newline
/// to `<manifest>.sig`. Pure file I/O + ring — hermetic and unit-tested.
/// (Ký file manifest, ghi chữ ký hex.)
pub fn run(manifest: Option<String>, key_hex: Option<String>) -> Result<()> {
    let seed_hex = key_hex
        .or_else(|| std::env::var("MGC_RELEASE_SIGNING_KEY").ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let require_signed = is_require_signed();
    run_with_key(
        &manifest.unwrap_or_else(|| "manifest.json".to_string()),
        seed_hex.as_deref(),
        require_signed,
    )
}

/// Core signing logic with explicit inputs (no env reads — directly
/// unit-testable).
/// (Logic ký với input tường minh — test được trực tiếp.)
pub fn run_with_key(manifest: &str, seed_hex: Option<&str>, require_signed: bool) -> Result<()> {
    let manifest_path = std::path::PathBuf::from(manifest);
    let Some(seed_hex) = seed_hex.filter(|s| !s.is_empty()) else {
        if require_signed {
            anyhow::bail!(
                "MGC_RELEASE_REQUIRE_SIGNED=1 but no signing key (--key-hex or MGC_RELEASE_SIGNING_KEY) — refusing to publish an unsigned stable release"
            );
        }
        eprintln!(
            "warning: no signing key — publishing UNSIGNED manifest (clients verify sha256 only)"
        );
        return Ok(());
    };
    let seed_bytes = hex_to_seed(seed_hex)?;
    let signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&seed_bytes)
        .map_err(|e| anyhow::anyhow!("invalid signing seed: {e:?}"))?;
    let manifest_bytes = std::fs::read(&manifest_path)
        .with_context(|| format!("read {}", manifest_path.display()))?;
    let sig = signer.sign(&manifest_bytes);
    // Self-verify before publishing the signature (a broken signer must
    // never ship a .sig file clients will reject).
    // (Tự verify trước khi publish chữ ký.)
    mgc_crypto::ed25519_signer::verify_signature(&signer.public_key(), &manifest_bytes, &sig)
        .map_err(|e| anyhow::anyhow!("self-verification of the fresh signature FAILED: {e:?}"))?;
    let sig_path = manifest_path.with_extension("json.sig");
    std::fs::write(&sig_path, format!("{}\n", hex::encode(&sig.0)))
        .with_context(|| format!("write {}", sig_path.display()))?;
    println!("manifest signature written (ring Ed25519, self-verified)");
    Ok(())
}

fn is_require_signed() -> bool {
    matches!(
        std::env::var("MGC_RELEASE_REQUIRE_SIGNED")
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes"
    )
}

fn hex_to_seed(hex_str: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(hex_str.trim())
        .map_err(|e| anyhow::anyhow!("signing key is not valid hex: {e}"))?;
    bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("signing key must be 64 hex chars (32-byte seed)"))
}

#[cfg(test)]
#[path = "../test/sign_release.rs"]
mod sign_release_tests;
