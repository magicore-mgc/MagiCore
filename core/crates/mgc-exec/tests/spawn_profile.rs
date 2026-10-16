//! Spawn-profile store tests — kiểm thử store spawn-profile.
//!
//! Exact-tuple lookup, version pinning, lock-drift expiry and unknown
//! schema rejection (Spec rev4 §6/§11).
//! (Tra cứu đúng bộ, ghim version, hết hạn khi lock trôi và từ chối schema
//! lạ (Spec rev4 §6/§11).)

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use mgc_exec::policy::LaneProfile;
use mgc_exec::spawn_profile::{
    SPAWN_PROFILE_SCHEMA, SpawnProfileFile, find_active_profile, store_profile,
};

fn sample() -> SpawnProfileFile {
    SpawnProfileFile {
        schema_version: SPAWN_PROFILE_SCHEMA.to_string(),
        mgc_version: "1.1.0-rc.9".to_string(),
        corpus_id: "corpus-2026-10-07".to_string(),
        lock_digest: Some("blake3-deadbeef".to_string()),
        captured_at: 1_000_000,
        profile: LaneProfile {
            framework: "next".to_string(),
            version: "16.3.8".to_string(),
            os: "darwin".to_string(),
            operation: "dev".to_string(),
            allowed_pm_invocations: vec!["npm --version".to_string()],
            requires_pm: vec![],
        },
    }
}

// Round-trip store then exact-tuple lookup hits.
// (Lưu rồi tra đúng bộ thì trúng.)
#[test]
fn roundtrip_lookup_hits() {
    let dir = tempfile::TempDir::new().unwrap();
    store_profile(dir.path(), &sample()).unwrap();
    let found = find_active_profile(
        dir.path(),
        "next",
        "16.3.8",
        "darwin",
        "dev",
        Some("blake3-deadbeef"),
    )
    .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().allowed_pm_invocations.len(), 1);
}

// Missing profile means level-2 tripwire-only (None, not an error).
// (Thiếu profile nghĩa là mức 2 tripwire-only (None, không phải lỗi).)
#[test]
fn missing_profile_is_none() {
    let dir = tempfile::TempDir::new().unwrap();
    assert_eq!(
        find_active_profile(dir.path(), "next", "16.3.8", "darwin", "dev", None).unwrap(),
        None
    );
}

// Version drift and lock drift both expire the profile.
// (Trôi version và trôi lock đều làm profile hết hạn.)
#[test]
fn drift_expires_profile() {
    let dir = tempfile::TempDir::new().unwrap();
    store_profile(dir.path(), &sample()).unwrap();
    assert_eq!(
        find_active_profile(
            dir.path(),
            "next",
            "16.4.0",
            "darwin",
            "dev",
            Some("blake3-deadbeef")
        )
        .unwrap(),
        None
    );
    assert_eq!(
        find_active_profile(
            dir.path(),
            "next",
            "16.3.8",
            "darwin",
            "dev",
            Some("blake3-other")
        )
        .unwrap(),
        None
    );
    assert_eq!(
        find_active_profile(dir.path(), "next", "16.3.8", "darwin", "dev", None).unwrap(),
        None
    );
}

// Unknown schema fails closed.
// (Schema lạ fail-closed.)
#[test]
fn unknown_schema_rejected() {
    let dir = tempfile::TempDir::new().unwrap();
    store_profile(dir.path(), &sample()).unwrap();
    let path = dir.path().join("next-dev-darwin.json");
    let mut text = std::fs::read_to_string(&path).unwrap();
    text = text.replace(SPAWN_PROFILE_SCHEMA, "spawn-profile-v99");
    std::fs::write(&path, text).unwrap();
    assert!(
        find_active_profile(
            dir.path(),
            "next",
            "16.3.8",
            "darwin",
            "dev",
            Some("blake3-deadbeef")
        )
        .is_err()
    );
}

// The recorder observes a real child tree (python parent + python child).
// (Recorder quan sát cây child thật (parent python + child python).)
#[test]
fn recorder_observes_live_subtree() {
    use mgc_exec::spawn_profile::record_subtree;
    let mut child = std::process::Command::new("python3")
        .args([
            "-c",
            "import subprocess, sys, time; subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(30)']); time.sleep(30)",
        ])
        .spawn()
        .unwrap();
    let observed = record_subtree(
        child.id(),
        std::time::Duration::from_secs(3),
        std::time::Duration::from_millis(200),
    );
    let _ = child.kill();
    let _ = child.wait();
    let py_count = observed
        .iter()
        .filter(|(name, _)| name.to_ascii_lowercase().contains("python"))
        .count();
    assert!(
        py_count >= 2,
        "expected parent+child python processes, saw: {observed:?}"
    );
}
