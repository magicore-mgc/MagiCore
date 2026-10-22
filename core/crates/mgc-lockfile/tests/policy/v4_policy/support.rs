// v4 trust policy + verification tests (design §3): modes, precedence,
// tamper fails always, unsigned/foreign-key per policy, sign roundtrip.
// (Test chính sách tin cậy + xác minh v4.)

use mgc_lockfile::EcosystemTag;
use mgc_lockfile::LockfileError;
use mgc_lockfile::canonical::{LockfileV4, PackageV4, payload_digest};
use mgc_lockfile::policy::{
    LockPolicyMode, enforce_policy, resolve_policy, sign_payload_digest, verify_v4_file,
    verify_v4_math,
};
use mgc_lockfile::schema::{CrossEdge, WorkspaceTopology};
use mgc_lockfile::v4::{PackageKey, VariantKey, canonical_name, canonical_version};

fn unsigned_doc() -> LockfileV4 {
    let mut lock = LockfileV4::new("mgc/1.2.0-test");
    lock.metadata.generated_at = "2026-09-17T00:00:00Z".to_string();
    lock.metadata.lockfile_hash = payload_digest(&lock.payload());
    lock
}

fn with_python_package(lock: &mut LockfileV4, source_id: &str, variant: VariantKey) {
    lock.packages.push(PackageV4 {
        key: PackageKey {
            ecosystem: EcosystemTag::Python,
            name: canonical_name(EcosystemTag::Python, "Requests"),
            version: canonical_version("2.31.0"),
            source_id: source_id.to_string(),
            variant,
        },
        edges: Vec::new(),
        markers: None,
        artifact: None,
        provenance: None,
        toolchain: None,
        scripts_policy: None,
        store_ref: None,
    });
}

fn refresh_digest(lock: &mut LockfileV4) {
    lock.metadata.lockfile_hash = payload_digest(&lock.payload());
}

// Serializes the env-touching tests in this file (process-global env is
// shared by parallel test threads).
// (Tuần tự hóa các test đụng env trong file này.)
static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn unsafe_env_set(key: &str, value: &str) {
    // SAFETY: test-only, called only while holding SERIAL, restored by
    // the caller before release. Edition 2024 made this unsafe; the
    // mutex makes it data-race-free.
    // (AN TOÀN: chỉ trong test, giữ SERIAL, caller phục hồi trước khi
    // nhả.)
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(key, value);
    }
}

fn unsafe_env_remove(key: &str) {
    #[allow(unsafe_code)]
    unsafe {
        std::env::remove_var(key);
    }
}

fn restore_env(key: &str, value: Option<std::ffi::OsString>) {
    // SAFETY: test-only, called while holding SERIAL to restore process env.
    // (AN TOÀN: chỉ trong test, đang giữ SERIAL để khôi phục env tiến trình.)
    #[allow(unsafe_code)]
    unsafe {
        if let Some(value) = value {
            std::env::set_var(key, value);
        } else {
            std::env::remove_var(key);
        }
    }
}
