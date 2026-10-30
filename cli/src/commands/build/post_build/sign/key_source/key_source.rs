//! One signing key for every MagiCore command, with legacy compatibility.
//! Một khoá ký dùng chung cho mọi lệnh MagiCore, vẫn tương thích bản cũ.
//!
//! `mgc trust init` writes a multi-key keyring at `~/.magicore/keys/keyring.json`
//! (default key = the one `mgc trust sign` uses). `mgc build` used to read a
//! separate `$DATA/keyring.json` with a single `signing_key` seed, so a user who
//! ran `mgc trust init` believed they had signed and builds stayed unsigned.
//! Resolution order below makes the trust keyring authoritative; the legacy
//! shape is still honoured after a visible warning so existing users do not
//! silently lose signing.
//! `mgc trust init` ghi keyring nhiều khoá ở `~/.magicore/keys/keyring.json`
//! (khoá mặc định là khoá `mgc trust sign` dùng). `mgc build` từng đọc
//! `$DATA/keyring.json` riêng với một seed `signing_key`, nên ai chạy
//! `mgc trust init` tưởng đã ký mà build vẫn không ký. Thứ tự dưới đây đặt
//! keyring trust là nguồn chính; dạng cũ vẫn được tôn trọng kèm cảnh báo rõ để
//! người dùng hiện tại không mất chữ ký một cách âm thầm.

use std::path::{Path, PathBuf};

use anyhow::Result;
use mgc_crypto::keyring::Keyring;

/// Resolved signing identity plus how it was found.
/// Identity ký đã resolve kèm nguồn gốc của nó.
pub(crate) struct SigningKey {
    /// Signer built from the selected key.
    /// Signer dựng từ khoá đã chọn.
    pub signer: mgc_crypto::Ed25519Signer,
    /// Stable key identifier shown to the user (`key_id` or a legacy alias).
    /// Định danh khoá ổn định hiển thị cho người dùng (`key_id` hoặc alias cũ).
    pub key_id: String,
    /// Public key the signature will carry.
    /// Public key mà chữ ký sẽ mang.
    pub public_key_b64: String,
    /// File the key was read from.
    /// File đã đọc khoá.
    pub source: PathBuf,
    /// Whether a second, different keyring was also found.
    /// Có tìm thấy keyring thứ hai khác hay không.
    pub conflict: Option<PathBuf>,
}

impl std::fmt::Debug for SigningKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never print key material; the source path and key id are enough.
        // (Không bao giờ in khoá; đường dẫn nguồn và key id là đủ.)
        formatter
            .debug_struct("SigningKey")
            .field("signer", &"[REDACTED]")
            .field("key_id", &self.key_id)
            .field("source", &self.source)
            .field("conflict", &self.conflict)
            .finish()
    }
}

impl SigningKey {
    fn new(
        signer: mgc_crypto::Ed25519Signer,
        key_id: String,
        source: PathBuf,
        conflict: Option<PathBuf>,
    ) -> Self {
        let public_key_b64 = signer.public_key().to_base64();
        Self {
            signer,
            key_id,
            public_key_b64,
            source,
            conflict,
        }
    }
}

/// Why no signing key could be resolved; rendered by the caller in English.
/// Lý do không resolve được khoá ký; caller hiển thị bằng tiếng Anh.
#[derive(Debug, thiserror::Error)]
pub(crate) enum KeyResolutionError {
    #[error(
        "`mgc trust` keyring '{path}' could not be read: {error}; \
         fix it or re-run `mgc trust init`"
    )]
    TrustKeyringUnreadable { path: String, error: String },
    #[error("`mgc trust` keyring '{path}' has no default key; run `mgc trust list` to inspect it")]
    TrustKeyringEmpty { path: String },
    #[error("`mgc trust` keyring '{path}' uses an unsupported key format: {error}")]
    TrustKeyringUnusable { path: String, error: String },
    #[error("signing key '{path}' has no 'signing_key' field")]
    LegacyMissingField { path: String },
    #[error("signing key '{path}' must be 32 bytes, got {actual}")]
    LegacyWrongLength { path: String, actual: usize },
    #[error("signing key '{path}' is not valid hex: {error}")]
    LegacyBadHex { path: String, error: String },
    #[error("cannot read signing key '{path}': {error}")]
    KeyFileUnreadable { path: String, error: String },
    #[error(
        "no signing key found. Run `mgc trust init` once: `mgc build` signs with the default \
         key of the `mgc trust` keyring and writes the public key a release verifier pins"
    )]
    NoKey,
}

/// Resolve the key a build should sign with, using the documented precedence.
///
/// 1. `MGC_SIGNING_KEYRING` — explicit path, either keyring shape.
/// 2. The `mgc trust` keyring (`~/.magicore/keys/keyring.json`), default key.
/// 3. The legacy MagiCore data-dir `keyring.json` (`signing_key` seed), warned.
/// 4. Nothing: the caller reports `NoKey` and the artefact stays unsigned.
/// 1. `MGC_SIGNING_KEYRING` — đường dẫn tường minh, cả hai dạng keyring.
/// 2. Keyring `mgc trust` (khoá mặc định).
/// 3. `keyring.json` kiểu cũ trong data dir (seed `signing_key`), có cảnh báo.
/// 4. Không có: caller báo `NoKey` và artefact không được ký.
pub(crate) fn resolve_signing_key(data_dir: &Path) -> Result<SigningKey> {
    let trust_path = trust_keyring_path()?;
    let legacy_path = data_dir.join(LEGACY_KEYRING_FILE);

    if let Some(explicit) = std::env::var_os(SIGNING_KEYRING_ENV).filter(|v| !v.is_empty()) {
        let explicit = PathBuf::from(explicit);
        let key = read_keyring_file(&explicit)?;
        let conflict = first_other_keyring(&explicit, &trust_path, &legacy_path);
        return Ok(SigningKey::new(key.signer, key.key_id, explicit, conflict));
    }

    let trust = read_optional_trust_keyring(&trust_path)?;
    let legacy = read_optional_legacy_keyring(&legacy_path)?;

    if let Some(trust) = trust {
        if legacy.is_some() {
            return Ok(SigningKey::new(
                trust.signer,
                trust.key_id,
                trust_path,
                Some(legacy_path),
            ));
        }
        return Ok(SigningKey::new(
            trust.signer,
            trust.key_id,
            trust_path,
            None,
        ));
    }
    if let Some(legacy) = legacy {
        return Ok(SigningKey::new(
            legacy.signer,
            legacy.key_id,
            legacy_path,
            None,
        ));
    }
    Err(KeyResolutionError::NoKey.into())
}

/// Environment variable that pins the keyring for build signing.
/// Biến môi trường ghim keyring dùng để ký build.
pub(crate) const SIGNING_KEYRING_ENV: &str = "MGC_SIGNING_KEYRING";

/// File name of the legacy single-seed keyring inside the MagiCore data dir.
/// Tên file keyring một-seed kiểu cũ trong data dir MagiCore.
const LEGACY_KEYRING_FILE: &str = "keyring.json";

/// Key identifier used when the key came from the legacy single-seed file.
/// Định danh khoá dùng khi khoá đến từ file một-seed kiểu cũ.
pub(crate) const LEGACY_KEY_ID: &str = "legacy:signing_key";

/// The `mgc trust` keyring path, i.e. the single source of truth for signing.
/// Đường dẫn keyring `mgc trust` — nguồn chân lý duy nhất cho việc ký.
pub(crate) fn trust_keyring_path() -> Result<PathBuf> {
    Ok(Keyring::default_path()?)
}

struct SelectedKey {
    signer: mgc_crypto::Ed25519Signer,
    key_id: String,
}

fn read_optional_trust_keyring(path: &Path) -> Result<Option<SelectedKey>> {
    if !path.exists() {
        return Ok(None);
    }
    let keyring =
        Keyring::load(path).map_err(|error| KeyResolutionError::TrustKeyringUnreadable {
            path: path.display().to_string(),
            error: error.to_string(),
        })?;
    let key = keyring
        .default_key()
        .ok_or_else(|| KeyResolutionError::TrustKeyringEmpty {
            path: path.display().to_string(),
        })?;
    let signer = key
        .signer()
        .map_err(|error| KeyResolutionError::TrustKeyringUnusable {
            path: path.display().to_string(),
            error: error.to_string(),
        })?;
    Ok(Some(SelectedKey {
        signer,
        key_id: key.key_id.clone(),
    }))
}

fn read_optional_legacy_keyring(path: &Path) -> Result<Option<SelectedKey>> {
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(read_legacy_file(path)?))
}

/// Read either keyring shape so an explicit `MGC_SIGNING_KEYRING` can point at
/// whichever file the project already uses.
/// Đọc một trong hai dạng keyring để `MGC_SIGNING_KEYRING` có thể trỏ tới file
/// project đang dùng.
fn read_keyring_file(path: &Path) -> Result<SelectedKey> {
    if Keyring::load(path).is_ok_and(|keyring| keyring.default_key().is_some()) {
        return read_optional_trust_keyring(path)?.ok_or_else(|| {
            KeyResolutionError::TrustKeyringEmpty {
                path: path.display().to_string(),
            }
            .into()
        });
    }
    read_legacy_file(path)
}

fn read_legacy_file(path: &Path) -> Result<SelectedKey> {
    let raw = std::fs::read(path).map_err(|error| KeyResolutionError::KeyFileUnreadable {
        path: path.display().to_string(),
        error: error.to_string(),
    })?;
    let value: serde_json::Value =
        serde_json::from_slice(&raw).map_err(|error| KeyResolutionError::KeyFileUnreadable {
            path: path.display().to_string(),
            error: error.to_string(),
        })?;
    let seed_hex = value
        .get("signing_key")
        .and_then(|k| k.as_str())
        .ok_or_else(|| KeyResolutionError::LegacyMissingField {
            path: path.display().to_string(),
        })?;
    let seed = decode_hex(seed_hex).map_err(|error| KeyResolutionError::LegacyBadHex {
        path: path.display().to_string(),
        error,
    })?;
    if seed.len() != 32 {
        return Err(KeyResolutionError::LegacyWrongLength {
            path: path.display().to_string(),
            actual: seed.len(),
        }
        .into());
    }
    let mut array = [0u8; 32];
    array.copy_from_slice(&seed);
    let signer = mgc_crypto::Ed25519Signer::from_seed(&array)?;
    Ok(SelectedKey {
        signer,
        key_id: LEGACY_KEY_ID.to_string(),
    })
}

/// Warn when a second, different keyring exists beside the one in use.
///
/// Returning the other path (instead of printing here) keeps the resolver free
/// of console output; the caller decides how to surface it.
/// Cảnh báo khi tồn tại keyring thứ hai khác bên cạnh bên đang dùng. Trả đường
/// dẫn còn lại (không in ở đây) để resolver không tự ghi console.
fn first_other_keyring(used: &Path, trust_path: &Path, legacy_path: &Path) -> Option<PathBuf> {
    [trust_path, legacy_path]
        .into_iter()
        .find(|candidate| candidate.exists() && *candidate != used)
        .map(Path::to_path_buf)
}

fn decode_hex(value: &str) -> std::result::Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) {
        return Err("hex string has odd length".to_string());
    }
    let bytes = value.as_bytes();
    bytes
        .chunks(2)
        .map(|pair| {
            std::str::from_utf8(pair)
                .map_err(|error| error.to_string())
                .and_then(|text| u8::from_str_radix(text, 16).map_err(|error| error.to_string()))
        })
        .collect()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
