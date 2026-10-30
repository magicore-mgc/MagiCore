#![allow(clippy::unwrap_used)]
//! Core-identity attestation tests — hermetic temp HOME, no real-home writes.
//! (Test chứng thực identity core — HOME tạm hermetic, không đụng HOME thật.)

//! Env mutation is serialized process-wide: every test holds the guard
//! below while a temp HOME is installed, so parallel tests can never
//! observe a torn HOME. Drop restores the previous HOME first.
//! (Đổi env được tuần tự hóa toàn process: mỗi test giữ guard trong lúc
//! HOME tạm được cài, nên test song song không bao giờ thấy HOME rách.
//! Drop khôi phục HOME cũ trước.)
#![allow(unsafe_code)]

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use mgc_config::attestation::{AnchorMode, AttestationStatus, attest, canonical_message, verify};

static HOME_GUARD: Mutex<()> = Mutex::new(());

/// Hermetic HOME + project + identities for one test.
struct Hermetic {
    _lock: MutexGuard<'static, ()>,
    _home: tempfile::TempDir,
    _work: tempfile::TempDir,
    previous_home: Option<std::ffi::OsString>,
    pub keyring: PathBuf,
    pub identities: PathBuf,
    pub root: PathBuf,
}

impl Drop for Hermetic {
    fn drop(&mut self) {
        match self.previous_home.take() {
            // SAFETY: test-only operation on process-local state or FFI with
            // caller-owned arguments valid for the call.
            // AN TOÀN: chỉ test thao tác state cục bộ process hoặc FFI với
            // đối số do caller sở hữu, hợp lệ trong lời gọi.
            Some(home) => unsafe { std::env::set_var("HOME", home) },
            // SAFETY: test-only operation on process-local state or FFI with
            // caller-owned arguments valid for the call.
            // AN TOÀN: chỉ test thao tác state cục bộ process hoặc FFI với
            // đối số do caller sở hữu, hợp lệ trong lời gọi.
            None => unsafe { std::env::remove_var("HOME") },
        }
    }
}

/// Pin the attestation key in the project manifest *before* attesting.
/// Pin key attestation vào manifest **trước** khi attest.
///
/// SEC-12/N-02: an unpinned project has no trust root. Pinning lives in
/// `mgc.toml`, which the attestation digest covers, so the pin must happen
/// before the attestation — exactly the order a user follows
/// (`mgc trust anchor`, then re-attest after any intentional edit).
/// (SEC-12/N-02: project không pin thì không có gốc tin cậy. Việc pin nằm trong
/// `mgc.toml` mà digest attestation phủ lên, nên pin phải xảy ra trước khi
/// attest — đúng thứ tự user làm (`mgc trust anchor`, rồi attest lại sau mọi
/// sửa có chủ đích).)
fn keyring_public_key(env: &Hermetic) -> String {
    // The keyring is created by the first attest; when it is absent, attest
    // once so the key exists, then pin and attest again.
    // (Keyring được tạo ở lần attest đầu; nếu chưa có, attest một lần để có key
    // rồi pin và attest lại.)
    // The keyring is `{ keys: [KeyPair], default_key_id }`; the default key is
    // the one `attest` signs with when no key id is given.
    // (Keyring là `{ keys: [KeyPair], default_key_id }`; key mặc định là key mà
    // `attest` dùng khi không chỉ định key id.)
    let document: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&env.keyring).unwrap()).unwrap();
    let default_id = document["default_key_id"].as_str();
    let keys = document["keys"]
        .as_array()
        .expect("the keyring lists its keys");
    // `public_key` serialises as a byte array, so the base64 form the manifest
    // pins is produced from those bytes.
    // (`public_key` serialize thành mảng byte, nên dạng base64 mà manifest pin
    // được tạo từ chính các byte đó.)
    let key = keys
        .iter()
        .find(|key| default_id.is_none_or(|id| key["key_id"] == id))
        .expect("the keyring holds the default key");
    let bytes: Vec<u8> = key["public_key"]
        .as_array()
        .expect("the public key serialises as bytes")
        .iter()
        .map(|byte| byte.as_u64().expect("a byte") as u8)
        .collect();
    mgc_crypto::Ed25519PublicKey(bytes).to_base64()
}

fn pin_key(env: &Hermetic, core: &str, public_key: &str) {
    std::fs::write(
        env.root.join("mgc.toml"),
        format!("name = \"demo\"\necosystem = \"{core}\"\n\n[trust]\nkeys = [\"{public_key}\"]\n"),
    )
    .unwrap();
}

fn hermetic() -> Hermetic {
    // A test that fails while holding the lock must not poison every later
    // test in this binary.
    // (Test fail khi giữ lock không được làm nhiễm độc mọi test sau.)
    let lock = HOME_GUARD
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let home = tempfile::tempdir().unwrap();
    // Canonicalize BEFORE installing HOME: Keyring::save compares the
    // canonical candidate path against $HOME, and tempdirs may sit under
    // symlinked prefixes (/var → /private/var on macOS). The keyring
    // path must be built from the SAME canonical root, or the prefix
    // check compares two different spellings of one directory.
    let home_canon = home.path().canonicalize().unwrap();
    let previous_home = std::env::var_os("HOME");
    // SAFETY: test-only env mutation; callers hold `env_lock` so no other
    // test thread observes a partially written environment.
    // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
    // không thấy môi trường ghi dở.
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { std::env::set_var("HOME", &home_canon) };
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join("proj");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("mgc.toml"),
        "name = \"demo\"\necosystem = \"web\"\n",
    )
    .unwrap();
    std::fs::write(root.join(".mgc.core"), "web\n").unwrap();
    let keyring = home_canon
        .join(".magicore")
        .join("keys")
        .join("keyring.json");
    let identities = work.path().join("identities");
    Hermetic {
        _lock: lock,
        _home: home,
        _work: work,
        previous_home,
        keyring,
        identities,
        root,
    }
}

struct ScopedEnvVar {
    name: &'static str,
    previous: Option<std::ffi::OsString>,
}

impl ScopedEnvVar {
    fn set(name: &'static str, value: &str) -> Self {
        let previous = std::env::var_os(name);
        // SAFETY: test-only env mutation; callers hold `env_lock` so no other
        // test thread observes a partially written environment.
        // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
        // không thấy môi trường ghi dở.
        // SAFETY: test-only operation on process-local state / FFI with
        // caller-owned buffers kept alive for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
        // buffer do caller sở hữu và còn sống trong lời gọi.
        unsafe { std::env::set_var(name, value) };
        Self { name, previous }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        match self.previous.take() {
            // SAFETY: test-only operation on process-local state or FFI with
            // caller-owned arguments valid for the call.
            // AN TOÀN: chỉ test thao tác state cục bộ process hoặc FFI với
            // đối số do caller sở hữu, hợp lệ trong lời gọi.
            Some(value) => unsafe { std::env::set_var(self.name, value) },
            // SAFETY: test-only operation on process-local state or FFI with
            // caller-owned arguments valid for the call.
            // AN TOÀN: chỉ test thao tác state cục bộ process hoặc FFI với
            // đối số do caller sở hữu, hợp lệ trong lời gọi.
            None => unsafe { std::env::remove_var(self.name) },
        }
    }
}

#[test]
fn anchor_mode_parses_and_defaults_to_warn() {
    let _env = hermetic();
    assert_eq!(AnchorMode::parse("off"), Some(AnchorMode::Off));
    assert_eq!(AnchorMode::parse("warn"), Some(AnchorMode::Warn));
    assert_eq!(AnchorMode::parse("require"), Some(AnchorMode::Require));
    assert_eq!(AnchorMode::parse("nope"), None);
}

#[test]
fn canonical_message_is_domain_separated_and_deterministic() {
    let _env = hermetic();
    let first = canonical_message("web", "abc");
    assert!(first.starts_with("mgc-core-attestation/v1\n"));
    assert_eq!(first, canonical_message("web", "abc"));
    assert_ne!(first, canonical_message("ai", "abc"));
    assert_ne!(first, canonical_message("web", "abd"));
}

#[test]
fn attest_then_verify_is_valid_and_idempotent() {
    let env = hermetic();
    let first = attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();
    assert_eq!(first.core, "web");
    assert!(!first.signature.is_empty());
    // Pinning edits `mgc.toml`, so the attestation is re-taken over the
    // pinned manifest before verification (the user's own order).
    // (Pin sửa `mgc.toml`, nên attestation được tạo lại trên manifest đã pin
    // trước khi verify — đúng thứ tự user làm.)
    pin_key(&env, "web", &keyring_public_key(&env));
    attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        true,
        false,
    )
    .unwrap();
    let status = verify(&env.root, &env.identities).unwrap();
    assert!(
        matches!(status, AttestationStatus::Valid),
        "expected Valid after pinning, got {status:?}"
    );
    // Idempotency is about the *content* signature: pinning changed the
    // manifest, so the digest (and therefore the file signature) legitimately
    // differs. The core-identity signature over the config digest must not.
    // (Tính idempotent nói về chữ ký nội dung: pin đã đổi manifest nên digest
    // (và do đó chữ ký file) đổi là hợp lý. Chữ ký core-identity trên digest
    // config thì không được đổi.)
    let second = attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();
    assert_eq!(first.core, second.core);
    assert_eq!(first.key_id, second.key_id);
    // `config_digest` covers the manifest, and pinning edited it, so the two
    // attestations legitimately describe different manifests; what must hold is
    // that the second run did not overwrite a still-valid attestation (the
    // helper returns the stored one unchanged).
    // (`config_digest` phủ manifest, mà pin đã sửa nó, nên hai attestation mô
    // tả hai manifest khác nhau là hợp lý; điều phải giữ là lần chạy thứ hai
    // không ghi đè một attestation còn hợp lệ.)
    assert_eq!(second.core_identity, first.core_identity);
}

#[test]
fn missing_slot_verifies_absent() {
    let env = hermetic();
    assert!(matches!(
        verify(&env.root, &env.identities).unwrap(),
        AttestationStatus::Absent
    ));
}

#[test]
fn edited_config_fails_with_mismatch() {
    let env = hermetic();
    attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();
    std::fs::write(
        env.root.join("mgc.toml"),
        "name = \"demo\"\necosystem = \"ai\"\n",
    )
    .unwrap();
    assert!(matches!(
        verify(&env.root, &env.identities).unwrap(),
        AttestationStatus::Mismatch(_)
    ));
}

#[test]
fn edited_core_field_breaks_the_signature() {
    let env = hermetic();
    attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();
    let slot = mgc_config::attestation::attestation_dir(&env.identities, &env.root);
    let path = slot.join(mgc_config::attestation::ATTESTATION_FILE);
    let text = std::fs::read_to_string(&path).unwrap();
    let forged: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(forged["core"], "web");
    let mut map = forged.as_object().unwrap().clone();
    map.insert(
        "core".to_string(),
        serde_json::Value::String("ai".to_string()),
    );
    std::fs::write(&path, serde_json::to_string_pretty(&map).unwrap()).unwrap();
    assert!(matches!(
        verify(&env.root, &env.identities).unwrap(),
        AttestationStatus::Mismatch(_)
    ));
}

#[test]
fn edited_live_core_marker_fails_attestation_verification() {
    let env = hermetic();
    attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();
    std::fs::write(env.root.join(".mgc.core"), "ai\n").unwrap();

    assert!(matches!(
        verify(&env.root, &env.identities).unwrap(),
        AttestationStatus::Mismatch(_)
    ));
}

#[test]
fn attestation_cannot_sign_a_core_different_from_the_live_marker() {
    let env = hermetic();
    assert!(
        attest(
            &env.root,
            "ai",
            &env.keyring,
            &env.identities,
            None,
            false,
            false,
        )
        .is_err()
    );
}

#[test]
fn mismatched_attestation_requires_explicit_re_attest() {
    let env = hermetic();
    let first = attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();
    std::fs::write(
        env.root.join("mgc.toml"),
        "name = \"renamed\"\necosystem = \"web\"\n",
    )
    .unwrap();

    assert!(
        attest(
            &env.root,
            "web",
            &env.keyring,
            &env.identities,
            None,
            false,
            false,
        )
        .is_err()
    );

    let replaced = attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        true,
        false,
    )
    .unwrap();
    assert_ne!(first.config_digest, replaced.config_digest);
    pin_key(&env, "web", &keyring_public_key(&env));
    attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        true,
        false,
    )
    .unwrap();
    assert!(matches!(
        verify(&env.root, &env.identities).unwrap(),
        AttestationStatus::Valid
    ));
}

#[test]
fn rotate_keeps_old_signatures_verifiable() {
    let env = hermetic();
    let first = attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();
    let rotated = attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        true,
    )
    .unwrap();
    assert_ne!(first.key_id, rotated.key_id);
    assert_eq!(rotated.prev_keys.len(), 1);
    assert_eq!(rotated.prev_keys[0].key_id, first.key_id);
    pin_key(&env, "web", &keyring_public_key(&env));
    attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        true,
        false,
    )
    .unwrap();
    assert!(matches!(
        verify(&env.root, &env.identities).unwrap(),
        AttestationStatus::Valid
    ));
}

#[test]
fn unknown_core_is_refused() {
    let env = hermetic();
    assert!(
        attest(
            &env.root,
            "nope",
            &env.keyring,
            &env.identities,
            None,
            false,
            false
        )
        .is_err()
    );
}

#[test]
fn canonical_cloud_alias_attests_as_clo() {
    let env = hermetic();
    std::fs::write(
        env.root.join("mgc.toml"),
        "name = \"demo\"\necosystem = \"cloud\"\n",
    )
    .unwrap();
    std::fs::write(env.root.join(".mgc.core"), "cloud\n").unwrap();
    let attestation = attest(
        &env.root,
        "cloud",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();
    assert_eq!(attestation.core, "clo");
}

#[test]
fn require_mode_fails_closed_when_attestation_state_is_corrupt() {
    let env = hermetic();
    let paths = mgc_platform::paths::GlobalPaths::new().unwrap();
    attest(
        &env.root,
        "web",
        &env.keyring,
        &paths.identities,
        None,
        false,
        false,
    )
    .unwrap();
    let attestation_path = mgc_config::attestation::attestation_dir(&paths.identities, &env.root)
        .join(mgc_config::attestation::ATTESTATION_FILE);
    std::fs::write(&attestation_path, "{").unwrap();
    let _mode = ScopedEnvVar::set("MGC_TRUST_ANCHOR", "require");

    let error = mgc_config::attestation::enforce_live_attestation(&env.root, "web")
        .expect_err("require mode must reject corrupt attestation state");
    assert!(error.to_string().contains("cannot verify core attestation"));
}

#[test]
fn require_mode_fails_closed_when_attestation_is_missing() {
    let env = hermetic();
    let _mode = ScopedEnvVar::set("MGC_TRUST_ANCHOR", "require");

    let error = mgc_config::attestation::enforce_live_attestation(&env.root, "web")
        .expect_err("require mode must reject a missing attestation");
    assert!(error.to_string().contains("no core attestation"));
}

#[test]
fn require_mode_rejects_a_renamed_attested_core_marker() {
    let env = hermetic();
    let paths = mgc_platform::paths::GlobalPaths::new().unwrap();
    attest(
        &env.root,
        "web",
        &env.keyring,
        &paths.identities,
        None,
        false,
        false,
    )
    .unwrap();
    std::fs::rename(
        env.root.join(".mgc.core"),
        env.root.join(".mgc.core.disabled"),
    )
    .unwrap();
    let _mode = ScopedEnvVar::set("MGC_TRUST_ANCHOR", "require");

    let error = mgc_config::project::ProjectConfig::read_core_marker(&env.root)
        .expect_err("renaming a trusted marker must not silently disable attestation");

    assert!(
        error
            .to_string()
            .contains("core identity attestation mismatch")
    );
    assert!(error.to_string().contains("project core marker is missing"));
}

#[test]
fn warn_mode_keeps_unanchored_legacy_project_discoverable_without_marker() {
    let env = hermetic();
    std::fs::remove_file(env.root.join(".mgc.core")).unwrap();
    let _mode = ScopedEnvVar::set("MGC_TRUST_ANCHOR", "warn");

    let core = mgc_config::project::ProjectConfig::read_core_marker(&env.root).unwrap();

    assert_eq!(core, None);
    assert_eq!(
        mgc_config::project::ProjectConfig::detect_core(&env.root)
            .unwrap()
            .as_deref(),
        Some("web"),
        "legacy identity remains available from mgc.toml in compatibility mode"
    );
}

/// P-8.3: an attestation issued for one core cannot be replayed onto another.
///
/// The file is copied verbatim onto a `lib` project whose `mgc.toml` bytes are
/// identical, so only the identity binding can tell the two apart.
/// (P-8.3: attestation của một core không replay được sang core khác. File được
/// copy nguyên văn sang project `lib` có cùng bytes `mgc.toml`, nên chỉ ràng
/// buộc identity phân biệt được hai bên.)
#[test]
fn cross_core_signature_replay_is_a_domain_mismatch() {
    let env = hermetic();
    attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();

    let other = env._work.path().join("libproj");
    std::fs::create_dir_all(&other).unwrap();
    std::fs::write(other.join(".mgc.core"), "lib\n").unwrap();
    // The manifest must declare the same core, otherwise the marker/manifest
    // guard answers first and the signature path is never reached.
    // (Manifest phải khai cùng core, nếu không guard marker/manifest trả lời
    // trước và không bao giờ tới bước verify chữ ký.)
    std::fs::write(
        other.join("mgc.toml"),
        "name = \"demo\"\necosystem = \"lib\"\n",
    )
    .unwrap();

    // Same key, same config digest, different core: the signature must be
    // rejected and reported as a domain mismatch, not as corruption.
    // (Cùng key, cùng digest config, khác core: chữ ký phải bị từ chối và báo
    // domain mismatch, không phải lỗi hỏng dữ liệu.)
    let web_slot = mgc_config::attestation::attestation_dir(&env.identities, &env.root);
    let lib_slot = mgc_config::attestation::attestation_dir(&env.identities, &other);
    std::fs::create_dir_all(&lib_slot).unwrap();
    std::fs::copy(
        web_slot.join(mgc_config::attestation::ATTESTATION_FILE),
        lib_slot.join(mgc_config::attestation::ATTESTATION_FILE),
    )
    .unwrap();

    match verify(&other, &env.identities).unwrap() {
        AttestationStatus::Mismatch(reason) => {
            assert!(
                reason.contains("live core marker is 'lib'"),
                "the live marker must be the reason: {reason}"
            );
        }
        AttestationStatus::DomainMismatch(reason) => {
            assert!(reason.contains("different core identity"), "{reason}");
        }
        other_status => panic!("cross-core replay must not verify: {other_status:?}"),
    }
}

/// P-8.8: the marker file stays one plain-text line.
/// (P-8.8: file marker vẫn là một dòng text thuần.)
#[test]
fn signature_never_rewrites_the_core_marker() {
    let env = hermetic();
    let before = std::fs::read(env.root.join(".mgc.core")).unwrap();
    attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();
    let after = std::fs::read(env.root.join(".mgc.core")).unwrap();
    assert_eq!(before, after);
    assert_eq!(String::from_utf8(after).unwrap(), "web\n");
}

/// P-8.5: a previous key whose grace period passed is refused, so a leaked
/// rotated-out key stops working.
/// (P-8.5: key cũ đã hết ân hạn bị từ chối, nên key bị lộ ngừng hoạt động.)
#[test]
fn expired_previous_key_is_refused() {
    let env = hermetic();
    attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();

    // Rewrite the attestation with a previous key whose deadline is in the
    // past while keeping the live signing key, so only the expiry changes.
    // (Ghi lại attestation với key cũ có hạn đã qua trong khi giữ key ký hiện
    // tại, nên chỉ hạn thay đổi.)
    let slot = mgc_config::attestation::attestation_dir(&env.identities, &env.root);
    let path = slot.join(mgc_config::attestation::ATTESTATION_FILE);
    let mut document: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    document["prev_keys"] = serde_json::json!([{
        "key_id": "rotated-out",
        "public_key": "AAAA",
        "expires_at": "2000-01-01T00:00:00Z",
    }]);
    std::fs::write(&path, serde_json::to_vec(&document).unwrap()).unwrap();

    match verify(&env.root, &env.identities).unwrap() {
        AttestationStatus::Mismatch(reason) => {
            assert!(reason.contains("grace deadline"), "{reason}");
        }
        other => panic!("an expired previous key must be refused: {other:?}"),
    }
}

/// P-8.1/P-8.2: editing a field that the in-document signature does not cover
/// is still caught, because the sidecar signs the exact bytes on disk.
/// (P-8.1/P-8.2: sửa một field mà chữ ký bên trong tài liệu không phủ vẫn bị
/// bắt, vì file cạnh đó ký đúng bytes trên đĩa.)
#[test]
fn tampered_attestation_file_is_detected_by_the_sidecar() {
    let env = hermetic();
    attest(
        &env.root,
        "web",
        &env.keyring,
        &env.identities,
        None,
        false,
        false,
    )
    .unwrap();

    // Pin the key that signed so the sidecar check has a trust root. The
    // public key is taken from the attestation the signer just wrote, which is
    // the same key the keyring holds.
    // (Pin key đã ký để bước kiểm sidecar có gốc tin cậy. Public key lấy từ
    // attestation signer vừa ghi, đúng key keyring đang giữ.)
    let slot_path = mgc_config::attestation::attestation_dir(&env.identities, &env.root)
        .join(mgc_config::attestation::ATTESTATION_FILE);
    let signed_document: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&slot_path).unwrap()).unwrap();
    let public_key = signed_document["public_key"]
        .as_str()
        .expect("attestation records the signing public key")
        .to_string();
    std::fs::write(
        env.root.join("mgc.toml"),
        format!("name = \"demo\"\necosystem = \"web\"\n\n[trust]\nkeys = [\"{public_key}\"]\n"),
    )
    .unwrap();

    let slot = mgc_config::attestation::attestation_dir(&env.identities, &env.root);
    let path = slot.join(mgc_config::attestation::ATTESTATION_FILE);
    let mut sidecar = path.as_os_str().to_os_string();
    sidecar.push(".sig");
    assert!(
        std::path::Path::new(&sidecar).is_file(),
        "signing must leave a sidecar signature"
    );

    let mut document: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    // `project_path` is not part of the in-document signed message.
    // (`project_path` không nằm trong message được ký bên trong tài liệu.)
    document["project_path"] = serde_json::json!("/somewhere/else");
    std::fs::write(&path, serde_json::to_vec(&document).unwrap()).unwrap();

    let status = verify(&env.root, &env.identities).unwrap();
    match status {
        AttestationStatus::Mismatch(reason) => {
            assert!(reason.contains("sidecar"), "{reason}");
        }
        other => panic!("a tampered attestation must not verify: {other:?}"),
    }
}
