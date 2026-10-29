//! SHARED-STORE STRESS (Gate 11-B.2, Task D): real cross-process contention
//! on the SHARED cache and the per-project store. Three scenarios:
//!
//!   B — shared store: `MGC_STRESS_SHARED_N` (default 64) processes, each a
//!       DIFFERENT project, ALL pointed at ONE `MGC_CACHE_DIR` shared cache.
//!       This is cross-project contention on the shared extracted-package
//!       roots + tarball cache (the SQLite DB + CAS stay per-project at
//!       `<project>/.magicore/cache/web` — the store root is hardcoded by
//!       `project_cache_dir`, NOT env-overridable). Every racer must either
//!       succeed or fail cleanly; ≥1 must succeed; none may die by signal;
//!       every project's store must then doctor with 0 orphan/missing/corrupt.
//!
//!   C — interleave: `MGC_STRESS_INTERLEAVE_INSTALLS` installs (default 32) +
//!       `MGC_STRESS_INTERLEAVE_PRUNES` prunes (16) + `MGC_STRESS_INTERLEAVE_
//!       DOCTORS` doctors (16) run CONCURRENTLY on ONE project (real SQLite/
//!       CAS contention). Installs must succeed or be REFUSED cleanly (lock/
//!       busy); prunes/doctors must never crash; no corruption; final
//!       `doctor --repair` then `doctor` → HEALTHY.
//!
//!   D — same-digest concurrent: 2 processes install the same package@version
//!       into 2 DIFFERENT projects (same shared cache) simultaneously → both
//!       succeed, and each project's CAS holds EXACTLY the blake3 files for
//!       the fixture (2), proving content-addressing never double-stores.
//!
//! (STRESS STORE DÙNG CHUNG: tranh chấp cross-process THẬT trên cache DÙNG
//! CHUNG và store per-project. Ba kịch bản: B — `MGC_STRESS_SHARED_N` (mặc
//! định 64) process, mỗi process là MỘT project KHÁC, TẤT CẢ trỏ MỘT
//! `MGC_CACHE_DIR` shared. Đây là tranh chấp cross-project trên extracted-
//! root + tarball cache dùng chung (SQLite DB + CAS vẫn per-project tại
//! `<project>/.magicore/cache/web` — root store hardcode bởi
//! `project_cache_dir`, KHÔNG override bằng env). Mọi racer hoặc thành công
//! hoặc fail sạch; ≥1 phải thành công; không process nào chết vì signal;
//! store mỗi project doctor 0 orphan/missing/corrupt. C — interleave: N
//! install + N/2 prune + N/2 doctor chạy ĐỒNG THỜI trên MỘT project (tranh
//! chấp SQLite/CAS thật). Install thành công hoặc bị TỪ CHỐI sạch (lock/
//! busy); prune/doctor không bao giờ crash; không corruption; cuối
//! `doctor --repair` rồi `doctor` → HEALTHY. D — same-digest: 2 process cài
//! cùng package@version vào 2 project KHÁC (cùng shared cache) đồng thời →
//! cả hai thành công, CAS mỗi project chứa ĐÚNG số file blake3 của fixture
//! (2), chứng minh content-addressing không double-store.)

#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::thread;

use tempfile::TempDir;

// Default process counts (RULE §12 — env overrides, not inline magic).
// (Số process mặc định (RULE §12 — env override, không literal ẩn).)
const DEFAULT_SHARED_N: usize = 64;
const DEFAULT_INTERLEAVE_INSTALLS: usize = 32;
const DEFAULT_INTERLEAVE_PRUNES: usize = 16;
const DEFAULT_INTERLEAVE_DOCTORS: usize = 16;

fn shared_n() -> usize {
    std::env::var("MGC_STRESS_SHARED_N")
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_SHARED_N)
}

fn interleave_installs() -> usize {
    std::env::var("MGC_STRESS_INTERLEAVE_INSTALLS")
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_INTERLEAVE_INSTALLS)
}

fn interleave_prunes() -> usize {
    std::env::var("MGC_STRESS_INTERLEAVE_PRUNES")
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_INTERLEAVE_PRUNES)
}

fn interleave_doctors() -> usize {
    std::env::var("MGC_STRESS_INTERLEAVE_DOCTORS")
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_INTERLEAVE_DOCTORS)
}

/// True when the process died by a signal (Unix) or has no exit code
/// (Windows) — a crash, not a clean exit.
/// (True khi process chết vì signal (Unix) hoặc không có exit code (Windows)
/// — crash, không phải exit sạch.)
fn died_by_signal(status: &ExitStatus) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        status.signal().is_some()
    }
    #[cfg(not(unix))]
    {
        status.code().is_none()
    }
}

struct RegistryFixture {
    _server: mockito::ServerGuard,
    _mocks: Vec<mockito::Mock>,
    url: String,
    /// Tarball hits observed by the mock, which counts real HTTP requests.
    /// Số lượt trúng tarball mà mock quan sát — mock đếm request HTTP thật.
    tarball_hits: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl RegistryFixture {
    fn new() -> Self {
        let mut server = mockito::Server::new();
        let url = server.url();
        let metadata = serde_json::json!({
            "name": "is-odd",
            "dist-tags": { "latest": "3.0.1" },
            "versions": { "3.0.1": package_metadata(&url, "3.0.1") }
        });
        let metadata_mock = server
            .mock("GET", "/is-odd")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(metadata.to_string())
            .expect_at_least(1)
            .create();
        // Every tarball answer is counted, so a refetch is measured instead of
        // inferred; `expect_at_least(1)` keeps the mock answering if a bug
        // makes the install ask again.
        // (Mọi lần trả tarball đều được đếm để refetch được đo chứ không suy
        // đoán; `expect_at_least(1)` giữ mock trả lời nếu bug khiến install
        // hỏi lại.)
        let tarball_hits = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counter = tarball_hits.clone();
        let tarball_mock = server
            .mock("GET", "/is-odd/-/is-odd-3.0.1.tgz")
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(package_tarball())
            .expect_at_least(1)
            .with_body_from_request(move |_| {
                counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                package_tarball()
            })
            .create();

        Self {
            _server: server,
            _mocks: vec![metadata_mock, tarball_mock],
            url,
            tarball_hits,
        }
    }

    /// How many times the tarball endpoint was actually requested.
    /// Số lần endpoint tarball thực sự được gọi.
    fn tarball_requests(&self) -> usize {
        self.tarball_hits.load(std::sync::atomic::Ordering::SeqCst)
    }
}

fn package_metadata(registry_url: &str, version: &str) -> serde_json::Value {
    serde_json::json!({
        "name": "is-odd",
        "version": version,
        "dependencies": {},
        "dist": { "tarball": format!("{registry_url}/is-odd/-/is-odd-{version}.tgz"), "integrity": sri_of(&package_tarball()) }
    })
}

fn package_tarball() -> Vec<u8> {
    let package_json = serde_json::json!({
        "name": "is-odd",
        "version": "3.0.1",
        "main": "index.js"
    })
    .to_string();
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    let mut header = tar::Header::new_gnu();
    header.set_size(package_json.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    archive
        .append_data(&mut header, "package/package.json", package_json.as_bytes())
        .unwrap();
    let index = "module.exports = function isOdd(n){return Math.abs(n % 2) === 1;};";
    let mut header = tar::Header::new_gnu();
    header.set_size(index.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    archive
        .append_data(&mut header, "package/index.js", index.as_bytes())
        .unwrap();
    let encoder = archive.into_inner().unwrap();
    encoder.finish().unwrap()
}

/// SRI digest for a fixture tarball (SEC-01: integrity is mandatory).
/// (SRI của tarball fixture; SEC-01 bắt buộc integrity.)
fn sri_of(bytes: &[u8]) -> String {
    use base64::Engine as _;
    use sha2::{Digest, Sha512};
    format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(Sha512::digest(bytes))
    )
}

fn find_mgc_binary() -> String {
    std::env::var("CARGO_BIN_EXE_mgc")
        .expect("CARGO_BIN_EXE_mgc not set — run via `cargo test -p mgc`")
}

fn create_web_project(temp: &TempDir, name: &str) -> PathBuf {
    let project = temp.path().join(name);
    std::fs::create_dir_all(&project).unwrap();
    std::fs::write(
        project.join("package.json"),
        r#"{ "name": "stress-shared", "version": "1.0.0", "dependencies": { "is-odd": "3.0.1" } }"#,
    )
    .unwrap();
    std::fs::write(project.join(".mgc.core"), "web\n").unwrap();
    project
}

/// Install pointing `MGC_CACHE_DIR` at the SHARED cache (cross-project state)
/// while the per-project store stays at `<project>/.magicore/cache/web`.
/// (Install trỏ `MGC_CACHE_DIR` vào cache DÙNG CHUNG (trạng thái cross-project)
/// trong khi store per-project vẫn ở `<project>/.magicore/cache/web`.)
fn install_cmd(mgc: &str, project: &Path, registry_url: &str, shared_cache: &Path) -> Command {
    let mut cmd = Command::new(mgc);
    cmd.arg("--core").arg("web").arg("install");
    cmd.current_dir(project);
    cmd.env("MAGICORE_WEB_REGISTRY_URL", registry_url);
    cmd.env("MAGICORE_WEB_ALLOWED_REGISTRIES", registry_url);
    cmd.env("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1");
    cmd.env("MGC_CACHE_DIR", shared_cache);
    cmd
}

fn doctor_cmd(mgc: &str, project: &Path) -> Command {
    let mut cmd = Command::new(mgc);
    cmd.arg("store").arg("doctor").arg("--core").arg("web");
    cmd.current_dir(project);
    cmd
}

fn doctor_repair_cmd(mgc: &str, project: &Path) -> Command {
    let mut cmd = doctor_cmd(mgc, project);
    cmd.arg("--repair");
    cmd
}

/// The user-facing prune the doctor gates ("prune/install refuse until
/// fixed"): `mgc cache prune` on the WEB project cache — the same store the
/// installs/doctors touch. `--core web` selects the web project cache dir.
/// (Lệnh prune user-facing mà doctor chặn: `mgc cache prune` trên cache
/// project WEB — đúng store mà install/doctor chạm. `--core web` chọn cache
/// project web.)
fn prune_cmd(mgc: &str, project: &Path) -> Command {
    let mut cmd = Command::new(mgc);
    cmd.arg("--core")
        .arg("web")
        .arg("cache")
        .arg("prune")
        .arg("--target")
        .arg("project")
        .arg("--yes");
    cmd.current_dir(project);
    cmd
}

/// Count regular files under `cas/files/blake3/` (recursive) — one file per
/// distinct digest; content-addressing must never double-store the same blob.
/// (Đếm file thường dưới `cas/files/blake3/` (đệ quy) — một file mỗi digest
/// phân biệt; content-addressing không bao giờ double-store cùng blob.)
fn count_cas_blob_files(cas_dir: &Path) -> usize {
    fn walk(dir: &Path, count: &mut usize) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, count);
                } else if path.is_file() {
                    *count += 1;
                }
            }
        }
    }
    let mut count = 0;
    walk(cas_dir, &mut count);
    count
}

/// B — shared store: N different projects, one shared cache. Real
/// cross-project contention on extracted roots + tarballs.
/// (B — store dùng chung: N project khác nhau, một cache chung. Tranh chấp
/// cross-project thật trên extracted-root + tarball.)
#[test]
fn shared_store_many_projects_stay_consistent() {
    let n = shared_n();
    let temp = TempDir::new().unwrap();
    let mgc = find_mgc_binary();
    let registry = RegistryFixture::new();
    let shared_cache = temp.path().join("shared-cache");

    let projects: Vec<PathBuf> = (0..n)
        .map(|i| create_web_project(&temp, &format!("proj-{i}")))
        .collect();

    // Spawn every install CONCURRENTLY (real threads + real processes).
    // (Spawn mọi install ĐỒNG THỜI (thread thật + process thật).)
    let handles: Vec<_> = projects
        .iter()
        .cloned()
        .map(|project| {
            let mgc = mgc.clone();
            let url = registry.url.clone();
            let shared_cache = shared_cache.clone();
            thread::spawn(move || install_cmd(&mgc, &project, &url, &shared_cache).output())
        })
        .collect();

    let mut successes = 0usize;
    let mut crashed = 0usize;
    for handle in handles {
        let out = handle.join().unwrap();
        let out = out.unwrap_or_else(|e| panic!("shared install spawn failed: {e}"));
        if out.status.success() {
            successes += 1;
        }
        if died_by_signal(&out.status) {
            crashed += 1;
        }
    }

    assert!(
        successes >= 1,
        "at least one shared-store install must succeed"
    );
    assert_eq!(
        crashed, 0,
        "no install may die by signal (clean exit or clean error only)"
    );

    // Every project's store must be uncorrupted (0 orphan/missing/corrupt).
    // (Store mỗi project phải không hỏng (0 orphan/missing/corrupt).)
    for project in &projects {
        let doctor = doctor_cmd(&mgc, project).output().unwrap();
        let stdout = String::from_utf8_lossy(&doctor.stdout);
        assert!(
            stdout.contains("0 orphan claim(s)"),
            "shared store must not orphan claims: {stdout}"
        );
        assert!(
            stdout.contains("0 missing blob(s)"),
            "shared store must not lose blobs: {stdout}"
        );
        assert!(
            stdout.contains("0 corrupt blob(s)"),
            "shared store must not corrupt blobs: {stdout}"
        );
    }
}

/// C — interleave: installs + prunes + doctors concurrently on ONE project.
/// (C — interleave: install + prune + doctor đồng thời trên MỘT project.)
#[test]
fn interleaved_install_prune_doctor_no_corruption() {
    let installs = interleave_installs();
    let prunes = interleave_prunes();
    let doctors = interleave_doctors();
    let temp = TempDir::new().unwrap();
    let mgc = find_mgc_binary();
    let registry = RegistryFixture::new();
    let shared_cache = temp.path().join("shared-cache");
    let project = create_web_project(&temp, "site");

    // Baseline populates the store so prune/doctor contend against live refs.
    // (Baseline lấp đầy store để prune/doctor tranh chấp với ref đang sống.)
    let out = install_cmd(&mgc, &project, &registry.url, &shared_cache)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "interleave baseline install must succeed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Spawn the full mix CONCURRENTLY.
    // (Spawn trọn hỗn hợp ĐỒNG THỜI.)
    let mut handles: Vec<thread::JoinHandle<(String, std::io::Result<std::process::Output>)>> =
        Vec::new();

    for i in 0..installs {
        let mgc = mgc.clone();
        let project = project.clone();
        let url = registry.url.clone();
        let shared_cache = shared_cache.clone();
        handles.push(thread::spawn(move || {
            (
                format!("install-{i}"),
                install_cmd(&mgc, &project, &url, &shared_cache).output(),
            )
        }));
    }
    for i in 0..prunes {
        let mgc = mgc.clone();
        let project = project.clone();
        handles.push(thread::spawn(move || {
            (format!("prune-{i}"), prune_cmd(&mgc, &project).output())
        }));
    }
    for i in 0..doctors {
        let mgc = mgc.clone();
        let project = project.clone();
        handles.push(thread::spawn(move || {
            (format!("doctor-{i}"), doctor_cmd(&mgc, &project).output())
        }));
    }

    let mut crashed = 0usize;
    let mut clean_failures = 0usize;
    for handle in handles {
        let (tag, out) = handle.join().unwrap();
        let out = out.unwrap_or_else(|e| panic!("{tag} spawn failed: {e}"));
        if died_by_signal(&out.status) {
            crashed += 1;
            eprintln!(
                "{tag} CRASHED (signal):\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
        if !out.status.success() {
            clean_failures += 1;
        }
    }
    assert_eq!(crashed, 0, "no interleave process may die by signal");

    // Final repair + health: any transient stale/busy must be recoverable.
    // (Repair + health cuối: mọi stale/busy tạm thời phải hồi phục được.)
    let repair = doctor_repair_cmd(&mgc, &project).output().unwrap();
    assert!(
        repair.status.success(),
        "interleave final doctor --repair must succeed:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&repair.stdout),
        String::from_utf8_lossy(&repair.stderr)
    );
    let final_doctor = doctor_cmd(&mgc, &project).output().unwrap();
    let final_stdout = String::from_utf8_lossy(&final_doctor.stdout);
    assert!(
        final_doctor.status.success()
            && final_stdout.contains("0 orphan claim(s)")
            && final_stdout.contains("0 missing blob(s)")
            && final_stdout.contains("0 corrupt blob(s)")
            && final_stdout.contains("0 stale staging gen(s)")
            && final_stdout.contains("HEALTHY"),
        "interleave store must recover to HEALTHY with no corruption:\n{final_stdout}\nclean failures observed: {clean_failures}"
    );
}

/// D — same-digest concurrent: two projects install the same package
/// simultaneously; both succeed and each CAS holds exactly the fixture's 2
/// blobs (no double-store).
/// (D — same-digest đồng thời: hai project cài cùng package đồng thời; cả
/// hai thành công và CAS mỗi project giữ đúng 2 blob của fixture (không
/// double-store).)
#[test]
fn concurrent_same_digest_does_not_double_store() {
    let temp = TempDir::new().unwrap();
    let mgc = find_mgc_binary();
    let registry = RegistryFixture::new();
    let shared_cache = temp.path().join("shared-cache");
    let p1 = create_web_project(&temp, "p1");
    let p2 = create_web_project(&temp, "p2");

    let h1 = {
        let mgc = mgc.clone();
        let p1 = p1.clone();
        let url = registry.url.clone();
        let shared_cache = shared_cache.clone();
        thread::spawn(move || install_cmd(&mgc, &p1, &url, &shared_cache).output())
    };
    let h2 = {
        let mgc = mgc.clone();
        let p2 = p2.clone();
        let url = registry.url.clone();
        let shared_cache = shared_cache.clone();
        thread::spawn(move || install_cmd(&mgc, &p2, &url, &shared_cache).output())
    };

    let s1 = h1.join().unwrap().unwrap();
    let s2 = h2.join().unwrap().unwrap();
    assert!(
        s1.status.success(),
        "first same-digest install must succeed:\n{}",
        String::from_utf8_lossy(&s1.stderr)
    );
    assert!(
        s2.status.success(),
        "second same-digest install must succeed:\n{}",
        String::from_utf8_lossy(&s2.stderr)
    );

    // The fixture has exactly 2 files (package.json + index.js) → exactly 2
    // distinct blake3 blobs. Any more proves a double-store.
    // (Fixture có đúng 2 file (package.json + index.js) → đúng 2 blob blake3
    // phân biệt. Nhiều hơn chứng tỏ double-store.)
    for project in [&p1, &p2] {
        let cas_dir = project.join(".magicore/cache/web/cas/files/blake3");
        let blobs = count_cas_blob_files(&cas_dir);
        assert_eq!(
            blobs, 2,
            "CAS must hold exactly the fixture's 2 blobs (no double-store), got {blobs}"
        );
        assert!(
            project.join("node_modules/is-odd/package.json").exists(),
            "materialization must exist for {}",
            project.display()
        );
    }
}

/// P-2.1/P-2.2: fifty projects with the same dependency set share one tarball
/// cache, so the tarball is fetched from the registry exactly once.
/// (P-2.1/P-2.2: năm mươi project cùng dependency set dùng chung một cache
/// tarball, nên tarball chỉ được tải từ registry đúng một lần.)
///
/// The second install of the same dependency set must not hit the registry at
/// all: mockito reports the request count per mock, so "no refetch" is measured
/// rather than assumed. Every project still gets its own materialized copy,
/// which is the isolation half of the criterion.
/// (Lần cài thứ hai cùng dependency set không được chạm registry: mockito báo
/// số request theo từng mock, nên "không tải lại" được đo chứ không giả định.
/// Mỗi project vẫn có bản materialize riêng — nửa isolation của tiêu chí.)
#[test]
fn fifty_projects_with_one_dependency_set_fetch_each_tarball_once() {
    let temp = TempDir::new().unwrap();
    let mgc = find_mgc_binary();
    let registry = RegistryFixture::new();
    let shared_cache = temp.path().join("shared-cache");

    let projects: Vec<PathBuf> = (1..=50)
        .map(|index| create_web_project(&temp, &format!("shared-{index}")))
        .collect();

    let first = install_cmd(&mgc, &projects[0], &registry.url, &shared_cache)
        .output()
        .unwrap();
    assert!(
        first.status.success(),
        "first install must succeed:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );
    // The mock only ever answers the tarball once; any later request would be
    // an unmatched 501 and the install below would fail, so success itself is
    // the measurement.
    // (Mock chỉ trả tarball đúng một lần; request sau sẽ là 501 không khớp và
    // lần cài dưới đây fail — nên chính việc thành công là phép đo.)
    let after_first = registry.tarball_requests();
    assert_eq!(after_first, 1, "the first install fetches the tarball once");

    for project in projects.iter().skip(1) {
        let output = install_cmd(&mgc, project, &registry.url, &shared_cache)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "install for {} must succeed:\n{}",
            project.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            project.join("node_modules/is-odd/package.json").exists(),
            "{} must be materialized from the shared cache",
            project.display()
        );
    }

    // P-2.1: forty-nine later installs added zero registry tarball requests.
    // (P-2.1: bốn mươi chín lần cài sau không thêm request tarball nào.)
    assert_eq!(
        registry.tarball_requests(),
        after_first,
        "projects 2..50 must reuse the cached tarball instead of refetching"
    );

    // P-2.2: each project keeps its own copy, so editing one cannot corrupt
    // another project's tree.
    // (P-2.2: mỗi project giữ bản riêng, nên sửa một project không thể làm hỏng
    // cây của project khác.)
    let first_manifest = projects[0].join("node_modules/is-odd/package.json");
    std::fs::write(&first_manifest, b"{\"tampered\":true}").unwrap();
    assert_ne!(
        std::fs::read(&first_manifest).unwrap(),
        std::fs::read(projects[1].join("node_modules/is-odd/package.json")).unwrap(),
        "projects must not share one mutable copy"
    );
}
