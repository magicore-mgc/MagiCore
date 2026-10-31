//! v3 byte reproducibility: identical graphs must write identical bytes.
//! Byte tái lập v3: cùng graph phải ghi ra byte y hệt.
//!
//! Reported defect (P0): `rm mgc.lock && mgc install` run four times produced
//! four different sha256 sums. The body was identical; only
//! `[metadata] generated_at` (a wall-clock stamp) differed, and the default
//! v3 schema persisted `lockfile_hash = ""` (no integrity at all). These
//! tests pin the fix: writing the same resolved graph twice produces
//! byte-identical files, `lockfile_hash` is populated and stable, and any
//! material change to the graph moves the hash.
//! (Lỗi đã báo (P0): `rm mgc.lock && mgc install` bốn lần cho bốn sha256
//! khác nhau. Body giống nhau, chỉ `generated_at` khác và v3 mặc định lưu
//! `lockfile_hash = ""` (không có toàn vẹn). Test dưới đây khóa bản sửa:
//! ghi cùng graph hai lần cho byte y hệt, hash được điền và ổn định, mọi
//! thay đổi thực chất của graph đổi hash.)

use mgc_lockfile::{
    EcosystemTag, Lockfile, Package, recompute_lockfile_hash, serialize_lockfile,
    with_lockfile_hash,
};
use tempfile::TempDir;

/// One representative v3 row: classified ecosystem + registry source, the
/// shape the ownership gate requires before serialization.
/// (Một row v3 đại diện: ecosystem phân loại + registry source, đúng dạng
/// cổng ownership yêu cầu trước khi serialize.)
fn package(name: &str, version: &str) -> Package {
    let mut package = Package::new(
        name.to_string(),
        version.to_string(),
        format!("https://registry.npmjs.org/{name}/-/{name}-{version}.tgz"),
        "blake3-abc123".to_string(),
    );
    package.ecosystem = EcosystemTag::Web;
    package.owner_core = Some("web".to_string());
    package.registry = Some("npm://registry.npmjs.org".to_string());
    package
}

fn lockfile(packages: Vec<Package>) -> Lockfile {
    let mut lock = Lockfile::new();
    for package in packages {
        lock.add_package(package);
    }
    lock.root_dependencies = vec!["react@18.2.0".to_string()];
    lock
}

/// Full resolve→write cycle: mutate the document exactly like the install
/// lanes do (fresh `generated_at`, generator), seal, serialize and write.
/// (Chu trình resolve→write đầy đủ: sửa tài liệu như lane install vẫn làm
/// (generated_at mới, generator), đóng dấu, serialize và ghi.)
fn write_cycle(workdir: &TempDir, lock: &Lockfile) -> (Vec<u8>, String) {
    let mut lock = lock.clone();
    lock.metadata.generated_at = chrono::Utc::now().to_rfc3339();
    lock.metadata.generator = format!("mgc/{}", env!("CARGO_PKG_VERSION"));
    let sealed = with_lockfile_hash(&lock).expect("sealing a valid lock must succeed");
    let text = serialize_lockfile(&sealed).expect("serialization must succeed");
    std::fs::write(workdir.path().join("mgc.lock"), text.as_bytes())
        .expect("test operation must succeed");
    (text.into_bytes(), sealed.metadata.lockfile_hash)
}

fn sample() -> Lockfile {
    lockfile(vec![
        package("react", "18.2.0"),
        package("scheduler", "0.23.0"),
    ])
}

#[test]
fn two_write_cycles_produce_byte_identical_lockfiles() {
    let dir = TempDir::new().expect("test operation must succeed");
    let lock = sample();
    let (first_bytes, first_hash) = write_cycle(&dir, &lock);
    let (second_bytes, second_hash) = write_cycle(&dir, &lock);
    assert_eq!(
        first_bytes, second_bytes,
        "the same graph must write byte-identical mgc.lock files"
    );
    assert_eq!(
        first_hash, second_hash,
        "lockfile_hash must not drift between identical resolve cycles"
    );
}

#[test]
fn written_v3_lock_carries_a_non_empty_stable_hash() {
    let dir = TempDir::new().expect("test operation must succeed");
    let (bytes, recorded) = write_cycle(&dir, &sample());
    let text = String::from_utf8(bytes).expect("lockfile must be UTF-8");
    assert!(
        !recorded.is_empty(),
        "v3 must never persist an empty lockfile_hash"
    );
    assert!(
        recorded.starts_with("blake3-"),
        "unexpected digest shape: {recorded}"
    );
    assert!(
        text.contains(&format!("lockfile_hash = \"{recorded}\"")),
        "the recorded digest must be present in the written bytes"
    );
    assert_eq!(
        recompute_lockfile_hash(&text).expect("self-hash must be recomputable"),
        recorded,
        "a reader must recompute the same digest from the file alone"
    );
}

#[test]
fn changing_a_package_changes_the_v3_hash_and_bytes() {
    let dir = TempDir::new().expect("test operation must succeed");
    let (baseline_bytes, baseline_hash) = write_cycle(&dir, &sample());
    let (changed_bytes, changed_hash) = write_cycle(
        &dir,
        &lockfile(vec![
            package("react", "18.2.0"),
            package("scheduler", "0.24.0"),
        ]),
    );
    assert_ne!(
        baseline_hash, changed_hash,
        "a different package version must change lockfile_hash"
    );
    assert_ne!(
        baseline_bytes, changed_bytes,
        "a different package version must change the written bytes"
    );
}

#[test]
fn re_sealing_an_already_sealed_lock_is_idempotent() {
    let dir = TempDir::new().expect("test operation must succeed");
    let (bytes, recorded) = write_cycle(&dir, &sample());
    let text = String::from_utf8(bytes).expect("lockfile must be UTF-8");
    let parsed = mgc_lockfile::parse_lockfile(&text).expect("written lock must parse");
    let resealed = with_lockfile_hash(&parsed).expect("re-sealing must succeed");
    assert_eq!(
        resealed.metadata.lockfile_hash, recorded,
        "sealing a sealed lock must not move the digest"
    );
}

#[test]
fn raw_serialization_also_seals_the_digest() {
    // Any path that renders v3 bytes must ship integrity: the serializer
    // seals on its own, so a caller cannot forget to.
    // (Mọi đường render byte v3 phải mang toàn vẹn: serializer tự seal nên
    // caller không thể quên.)
    let text = serialize_lockfile(&sample()).expect("serialization must succeed");
    let parsed = mgc_lockfile::parse_lockfile(&text).expect("output must parse");
    assert!(!parsed.metadata.lockfile_hash.is_empty());
    assert_eq!(
        recompute_lockfile_hash(&text).expect("self-hash must be recomputable"),
        parsed.metadata.lockfile_hash
    );
}

#[test]
fn tampering_with_a_package_is_detected_by_the_self_hash() {
    let dir = TempDir::new().expect("test operation must succeed");
    let (bytes, recorded) = write_cycle(&dir, &sample());
    let text = String::from_utf8(bytes).expect("lockfile must be UTF-8");
    let tampered = text.replace("blake3-abc123", "blake3-evil000");
    assert_ne!(tampered, text, "fixture must actually change a package");
    let error = recompute_lockfile_hash(&tampered)
        .expect_err("a modified body must not reproduce the recorded digest");
    assert!(error.to_string().contains("lockfile_hash does not match"));
    assert!(
        tampered.contains(&recorded),
        "the recorded digest is what the reader compares against"
    );
}

#[test]
fn generated_at_is_a_deterministic_recording_time() {
    // No wall clock in the committed lock: the stamp is derived from the
    // content, so two cycles cannot disagree even across machines.
    // (Không có đồng hồ trong lock commit: stamp suy từ nội dung nên hai
    // chu trình không thể lệch dù khác máy.)
    let dir = TempDir::new().expect("test operation must succeed");
    let (bytes, _) = write_cycle(&dir, &sample());
    let text = String::from_utf8(bytes).expect("lockfile must be UTF-8");
    let parsed = mgc_lockfile::parse_lockfile(&text).expect("written lock must parse");
    let stamp = &parsed.metadata.generated_at;
    assert_eq!(stamp.len(), 20, "RFC 3339 stamp expected, got {stamp:?}");
    assert!(stamp.ends_with('Z'));
}
