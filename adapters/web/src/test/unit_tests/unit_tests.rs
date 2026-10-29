use std::path::Path;
use mgc_types::{ContentStoreProvider, DependencyResolver, ProjectDetector};
use mgc_types::adapter::AddOptions;
use crate::WebAdapter;
use mgc_types::{Manifest, MgError, PackageId, PackageName, Version, VersionRange};
use mgc_types::adapter::{InstallOptions, ResolvedGraph, ResolvedPackage, PackageAdapter};
// Tests mutate env single-threaded (edition 2024 unsafe rule) — test đổi env 1 luồng.
use super::*;

/// Registry origin used by install tests that seed cache entries.
/// Origin registry dùng bởi test install có seed entry cache.
pub(crate) const TEST_REGISTRY_URL: &str = "http://127.0.0.1:9";
use crate::audit::parse_advisory_bulk_response;
use base64::Engine;
use flate2::Compression;
use flate2::write::GzEncoder;
use mgc_lockfile::Lockfile;
use mgc_resolver::DependencyProvider;
use mgc_store::{Layout, PackageCache};
use mgc_types::DependencySpec;
use sha2::{Digest, Sha512};
use std::io::ErrorKind;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tar::{Builder, Header};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::cache::*;
use crate::install::extract::*;
use crate::install::fetch::{publish_downloaded_tarball, unique_tarball_staging_path};
use crate::install::materialize::*;
use crate::install::{should_run_lifecycle_scripts, trust_allows_script};
use crate::lockfile::*;
use crate::manifest::*;
use crate::provider::*;
use crate::update::*;

#[path = "../../install/extract/test/warm_cache_tests.rs"]
mod warm_cache_tests;

// P07 subject split: per-subject test modules live beside this file;
// registration for the moved tests (they run through these modules).
// (Tách test P07: các module test theo chủ đề nằm cạnh file này —
// đăng ký cho các test đã tách.)
#[path = "../audit_tests.rs"]
mod audit_tests;
#[path = "../cache_tests.rs"]
mod cache_tests;
#[path = "../install/mod.rs"]
mod install_tests;
#[path = "../../lockfile/test/lockfile_signed_write_tests.rs"]
mod lockfile_signed_write_tests;
#[path = "../../lockfile/test/lockfile_v3_write_tests.rs"]
mod lockfile_v3_write_tests;
#[path = "../../lockfile/test/lockfile_root_range_tests.rs"]
mod lockfile_root_range_tests;
#[path = "../../lockfile/test/lockfile_root_normalization_tests.rs"]
mod lockfile_root_normalization_tests;
#[path = "../../lockfile/test/lockfile_graph_closure_tests.rs"]
mod lockfile_graph_closure_tests;
#[path = "../../lockfile/test/lockfile_foreign_ecosystem_tests.rs"]
mod lockfile_foreign_ecosystem_tests;
#[path = "../../lockfile/test/lockfile_replay_tests.rs"]
mod lockfile_replay_tests;
#[path = "../../lockfile/test/lockfile_filesystem_safety_tests.rs"]
mod lockfile_filesystem_safety_tests;
#[path = "../../lockfile/test/lockfile_reader_format_tests.rs"]
mod lockfile_reader_format_tests;
#[path = "../../lockfile/test/lockfile_v4_schema_tests.rs"]
mod lockfile_v4_schema_tests;
#[path = "../../lockfile/test/lockfile_v4_merge_tests.rs"]
mod lockfile_v4_merge_tests;
#[path = "../../lockfile/test/lockfile_v4_signature_tests.rs"]
mod lockfile_v4_signature_tests;
#[path = "../manifest_tests.rs"]
mod manifest_tests;
#[path = "../policy_tests.rs"]
mod policy_tests;
#[path = "../resolution_tests/mod.rs"]
mod resolution_tests;
async fn bind_test_listener() -> Option<TcpListener> {
    match TcpListener::bind("127.0.0.1:0").await {
        Ok(listener) => Some(listener),
        Err(err) if err.kind() == ErrorKind::PermissionDenied => {
            eprintln!("skipping socket-backed test in sandbox: {err}");
            None
        }
        Err(err) => panic!("failed to bind socket-backed test listener: {err}"),
    }
}

/// Symlink-free temp dir (P0-A contract, 2026-09-13): the CAS now rejects
/// any symlinked ancestor, and macOS temp lives under /var — a SYSTEM
/// symlink. Create the temp dir under the CANONICALIZED temp base so every
/// store/CAS path in these tests is real. Mirrors the convention already
/// used by core/crates/mgc-store/tests/adversarial_integrity.rs.
/// (Temp dir không symlink (hợp đồng P0-A): CAS giờ từ chối mọi ancestor
/// là symlink, mà temp macOS nằm dưới /var — symlink HỆ THỐNG. Tạo temp
/// dưới base temp ĐÃ CANONICALIZE để mọi path store/CAS trong các test
/// này là đường dẫn thật. Phản chiếu quy ước mà test adversarial của
/// mgc-store đã dùng.)
fn tempdir_real() -> std::io::Result<tempfile::TempDir> {
    let canonical = std::env::temp_dir().canonicalize()?;
    tempfile::tempdir_in(canonical)
}

fn seed_cached_tarball(root: &Path, pkg: &PackageId) -> String {
    let package_json = format!(
        "{{\"name\":\"{}\",\"version\":\"{}\"}}",
        pkg.name_str(),
        pkg.version()
    );
    seed_cached_tarball_with_files(
        root,
        pkg,
        &[
            ("package/package.json", package_json.as_bytes()),
            ("package/index.css", b"@import 'tailwindcss';"),
        ],
    )
}

fn seed_cached_tarball_with_files(root: &Path, pkg: &PackageId, files: &[(&str, &[u8])]) -> String {
    let layout = Layout::new(root.join(".magicore").join("cache").join("web"));
    std::fs::create_dir_all(layout.root()).unwrap();
    // SEC-01: production caches are scoped per registry origin; seeds must
    // land in the same scope the install pipeline reads.
    // SEC-01: cache production scope theo origin registry; seed phải nằm cùng
    // scope mà pipeline install đọc.
    let cache = PackageCache::for_registry(layout.cache_dir(), TEST_REGISTRY_URL).unwrap();
    let tarball_path = cache.tarball_path(pkg);
    if let Some(parent) = tarball_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let tarball = build_tarball_bytes(files);
    std::fs::write(&tarball_path, &tarball).unwrap();
    sri_sha512(&tarball)
}

fn seed_shared_tarball_with_files(root: &Path, pkg: &PackageId, files: &[(&str, &[u8])]) -> String {
    let layout = Layout::new(root.to_path_buf());
    std::fs::create_dir_all(layout.root()).unwrap();
    // SEC-01: production caches are scoped per registry origin; seeds must
    // land in the same scope the install pipeline reads.
    // SEC-01: cache production scope theo origin registry; seed phải nằm cùng
    // scope mà pipeline install đọc.
    let cache = PackageCache::for_registry(layout.cache_dir(), TEST_REGISTRY_URL).unwrap();
    let tarball_path = cache.tarball_path(pkg);
    if let Some(parent) = tarball_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let tarball = build_tarball_bytes(files);
    std::fs::write(&tarball_path, &tarball).unwrap();
    sri_sha512(&tarball)
}

fn seed_shared_metadata(root: &Path, package: &str, payload: serde_json::Value) {
    let path = root
        .join("metadata")
        .join("v2")
        .join("http___127_0_0_1_9") // khớp reg_key của mock registry url (:9)
        .join(package)
        .join("metadata.json");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, serde_json::to_vec(&payload).unwrap()).unwrap();
}

fn build_tarball_bytes(files: &[(&str, &[u8])]) -> Vec<u8> {
    let temp = tempfile::NamedTempFile::new().unwrap();
    let file = temp.reopen().unwrap();
    let encoder = GzEncoder::new(file, Compression::default());
    let mut builder = Builder::new(encoder);
    for (path, data) in files {
        write_tar_entry(&mut builder, path, data);
    }
    builder.finish().unwrap();
    let encoder = builder.into_inner().unwrap();
    encoder.finish().unwrap();
    std::fs::read(temp.path()).unwrap()
}

fn sri_sha512(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(hasher.finalize())
    )
}

fn restore_env_var(key: &str, previous: Option<std::ffi::OsString>) {
    if let Some(value) = previous {
        // SAFETY: test-only env mutation; callers hold `env_lock` so no other
        // test thread observes a partially written environment.
        // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
        // không thấy môi trường ghi dở.
        // SAFETY: test-only operation on process-local state / FFI with
        // caller-owned buffers kept alive for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
        // buffer do caller sở hữu và còn sống trong lời gọi.
        unsafe { std::env::set_var(key, value) };
    } else {
        // SAFETY: test-only env mutation; callers hold `env_lock` so no other
        // test thread observes a partially written environment.
        // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
        // không thấy môi trường ghi dở.
        // SAFETY: test-only operation on process-local state / FFI with
        // caller-owned buffers kept alive for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
        // buffer do caller sở hữu và còn sống trong lời gọi.
        unsafe { std::env::remove_var(key) };
    }
}

struct ScopedEnvVar {
    key: &'static str,
    previous: Option<std::ffi::OsString>,
}

impl ScopedEnvVar {
    fn set(key: &'static str, value: &str) -> Self {
        let previous = std::env::var_os(key);
        // SAFETY: test-only env mutation; callers hold `env_lock` so no other
        // test thread observes a partially written environment.
        // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
        // không thấy môi trường ghi dở.
        // SAFETY: test-only operation on process-local state / FFI with
        // caller-owned buffers kept alive for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
        // buffer do caller sở hữu và còn sống trong lời gọi.
        unsafe { std::env::set_var(key, value) };
        Self { key, previous }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        restore_env_var(self.key, self.previous.take());
    }
}

fn write_tar_entry(builder: &mut Builder<GzEncoder<std::fs::File>>, path: &str, data: &[u8]) {
    let mut header = Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    builder.append_data(&mut header, path, data).unwrap();
}

// ===== audit parse contract — npm Bulk Advisory API (Tech Lead P0-1
// 2026-09-09) =====
// Schema khớp npm Bulk Advisory endpoint THẬT (cùng shape pnpm audit dùng):
// registry KHÔNG trả `findings` — client đối chiếu vulnerable_versions với
// lockfile để dựng finding.

/// Minimal lockfile fixture with the requested packages.
/// Lockfile tối thiểu với các package đã yêu cầu.
fn audit_lockfile_fixture() -> Vec<(String, String)> {
    vec![
        ("lodash".to_string(), "4.17.12".to_string()),
        ("lodash".to_string(), "4.17.21".to_string()),
    ]
}

/// Real npm bulk advisory shape (recorded response contract):
/// {id, url, title, severity, vulnerable_versions} — NO findings field.
/// Shape thật của npm bulk advisory: KHÔNG có trường findings.
fn lodash_advisory_json() -> serde_json::Value {
    serde_json::json!({
        "id": 1102260,
        "url": "https://github.com/advisories/GHSA-35jh-8xhr-j9wr",
        "title": "Prototype Pollution in lodash",
        "severity": "high",
        "vulnerable_versions": "<4.17.21"
    })
}
