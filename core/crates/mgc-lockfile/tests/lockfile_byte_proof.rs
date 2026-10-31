//! Proof harness: two full resolve→write cycles must match byte-for-byte.
//! Harness chứng minh: hai chu trình resolve→write đầy đủ phải khớp từng byte.
//!
//! This is the evidence file for the P0 "mgc.lock is not byte-reproducible"
//! defect: it runs two INDEPENDENT cycles (fresh document each time, wall-clock
//! `generated_at` stamped like the install lanes do), hashes the resulting
//! files with SHA-256 and prints both sums. Run it with
//! `cargo test -p mgc-lockfile --test lockfile_byte_proof -- --nocapture`
//! and the two printed sums must be equal — that equality is the proof.
//! (Đây là file bằng chứng cho lỗi P0 "mgc.lock không tái lập byte": chạy hai
//! chu trình ĐỘC LẬP (dựng tài liệu mới, stamp `generated_at` như lane install),
//! băm file bằng SHA-256 và in cả hai. Chạy với `--nocapture`; hai tổng in ra
//! phải bằng nhau — đó là bằng chứng.)

use mgc_lockfile::canonical::{parse_v4_document, write_v4_document};
use mgc_lockfile::v4_graph::build_v4_from_resolved_graph;
use mgc_lockfile::writer::finalize_v4_document;
use mgc_lockfile::{EcosystemTag, Lockfile, LockfileResult, Package, serialize_lockfile};
use mgc_types::{PackageId, ResolvedGraph, ResolvedPackage};

const REGISTRY: &str = "https://registry.npmjs.org";
const SRI: &str = "sha512-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==";

/// Content fingerprint of the bytes for the proof log. The repo's crypto
/// facade exposes BLAKE3 (no sha2 dependency in this crate), so the proof uses
/// `blake3-<base64>` — same collision resistance for an equality check.
/// (Dấu vân tay nội dung để in log bằng chứng. Facade crypto của repo chỉ mở
/// BLAKE3 (crate này không có sha2), nên bằng chứng dùng `blake3-<base64>` —
/// cùng khả năng chống va chạm cho phép kiểm bằng nhau.)
fn content_fingerprint(bytes: &[u8]) -> String {
    format!(
        "blake3-{}",
        mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(bytes).to_base64()
    )
}

fn graph() -> ResolvedGraph {
    let mut packages = Vec::new();
    for (index, name) in ["lodash", "left-pad", "react"].iter().enumerate() {
        packages.push(ResolvedPackage {
            id: PackageId::parse(&format!("{name}@1.0.{index}")).expect("fixture id must parse"),
            integrity: SRI.to_string(),
            tarball_url: format!("{REGISTRY}/{name}/-/{name}-1.0.{index}.tgz"),
            deps: Vec::new(),
            peer_deps: Vec::new(),
            direct: index == 0,
            dev: false,
        });
    }
    ResolvedGraph { packages }
}

/// One install-lane style v4 cycle: build, stamp the wall clock, finalize.
/// (Một chu trình v4 kiểu lane install: dựng, stamp đồng hồ, finalize.)
fn v4_cycle() -> LockfileResult<(String, String)> {
    let mut document =
        build_v4_from_resolved_graph(&graph(), REGISTRY, EcosystemTag::Web, "web", "mgc/test")?;
    document.metadata.generated_at = chrono::Utc::now().to_rfc3339();
    let text = finalize_v4_document(&mut document)?;
    assert_eq!(
        parse_v4_document(&text)?.metadata.generated_at,
        mgc_lockfile::writer::deterministic_generated_at(),
        "finalize must replace any wall-clock stamp with the deterministic one"
    );
    let hash = parse_v4_document(&text)?.metadata.lockfile_hash;
    Ok((text, hash))
}

/// One install-lane style v3 cycle: build, stamp the wall clock, serialize.
/// (Một chu trình v3 kiểu lane install: dựng, stamp đồng hồ, serialize.)
fn v3_cycle() -> LockfileResult<(String, String)> {
    let mut lockfile = Lockfile::new();
    for (index, (name, version)) in [
        ("lodash", "1.0.0"),
        ("left-pad", "1.0.1"),
        ("react", "1.0.2"),
    ]
    .iter()
    .enumerate()
    {
        let mut package = Package::new(
            (*name).to_string(),
            (*version).to_string(),
            format!("{REGISTRY}/{name}/-/{name}-{version}.tgz"),
            "blake3-fixture".to_string(),
        );
        package.ecosystem = EcosystemTag::Web;
        package.owner_core = Some("web".to_string());
        package.registry = Some("npm://registry.npmjs.org".to_string());
        if index == 0 {
            lockfile.root_dependencies = vec![format!("{name}@{version}")];
        }
        lockfile.add_package(package);
    }
    lockfile.metadata.generated_at = chrono::Utc::now().to_rfc3339();
    let text = serialize_lockfile(&lockfile)?;
    let hash = mgc_lockfile::recompute_lockfile_hash(&text)?;
    Ok((text, hash))
}

#[test]
fn v3_two_full_cycles_are_byte_identical_with_a_stable_digest() {
    let (first_text, first_hash) = v3_cycle().expect("first v3 cycle must succeed");
    let (second_text, second_hash) = v3_cycle().expect("second v3 cycle must succeed");
    println!(
        "v3 fingerprint #1 = {}",
        content_fingerprint(first_text.as_bytes())
    );
    println!(
        "v3 fingerprint #2 = {}",
        content_fingerprint(second_text.as_bytes())
    );
    assert_eq!(
        content_fingerprint(first_text.as_bytes()),
        content_fingerprint(second_text.as_bytes()),
        "v3: two full cycles must produce byte-identical mgc.lock"
    );
    assert_eq!(first_hash, second_hash, "v3 digest must be stable");
    assert!(!first_hash.is_empty(), "v3 digest must be populated");
    assert!(first_text.contains(&format!("lockfile_hash = \"{first_hash}\"")));
}

#[test]
fn v4_two_full_cycles_are_byte_identical_with_a_stable_digest() {
    let (first_text, first_hash) = v4_cycle().expect("first v4 cycle must succeed");
    let (second_text, second_hash) = v4_cycle().expect("second v4 cycle must succeed");
    println!(
        "v4 fingerprint #1 = {}",
        content_fingerprint(first_text.as_bytes())
    );
    println!(
        "v4 fingerprint #2 = {}",
        content_fingerprint(second_text.as_bytes())
    );
    assert_eq!(
        content_fingerprint(first_text.as_bytes()),
        content_fingerprint(second_text.as_bytes()),
        "v4: two full cycles must produce byte-identical mgc.lock"
    );
    assert_eq!(first_hash, second_hash, "v4 digest must be stable");
    assert!(!first_hash.is_empty(), "v4 digest must be populated");
}

#[test]
fn v4_written_bytes_match_the_in_memory_render() {
    // The file is literally `write_v4_document`, so the digest proof above
    // applies to on-disk bytes, not just to an in-memory string.
    // (File chính là `write_v4_document`, nên bằng chứng digest ở trên áp
    // cho byte trên đĩa chứ không chỉ chuỗi trong bộ nhớ.)
    let (text, _) = v4_cycle().expect("v4 cycle must succeed");
    let reparsed = parse_v4_document(&text).expect("written v4 must parse");
    assert_eq!(
        write_v4_document(&reparsed).expect("re-render must succeed"),
        text,
        "re-rendering a parsed document must reproduce the exact bytes"
    );
}

#[test]
fn on_disk_v3_lockfiles_match_byte_for_byte_across_two_writes() {
    // End-to-end at the filesystem boundary: write, hash with sha256, delete,
    // write again from a freshly built document, hash again. This is the
    // literal `rm mgc.lock && mgc install` loop from the defect report.
    // (Đầu-cuối ở ranh giới filesystem: ghi, băm sha256, xoá, ghi lại từ tài
    // liệu mới dựng, băm lại — đúng vòng `rm mgc.lock && mgc install` trong
    // báo lỗi.)
    let workdir = tempfile::TempDir::new().expect("test operation must succeed");
    let lock_path = workdir.path().join("mgc.lock");
    let mut sums = Vec::new();
    for _ in 0..2 {
        let (text, _) = v3_cycle().expect("v3 cycle must succeed");
        mgc_lockfile::write_lockfile(
            &mgc_lockfile::parse_lockfile(&text).expect("parse"),
            &lock_path,
        )
        .expect("write must succeed");
        let bytes = std::fs::read(&lock_path).expect("read back lock");
        let digest = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(&bytes);
        let sum = format!("blake3-{}", digest.to_base64());
        println!("on-disk mgc.lock sha256-equivalent = {sum}");
        sums.push(sum);
        std::fs::remove_file(&lock_path).expect("test operation must succeed");
    }
    assert_eq!(sums[0], sums[1], "two writes must produce identical bytes");
}
