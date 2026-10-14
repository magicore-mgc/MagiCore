//! E2E harness — black-box: chạy binary `mgc` / `mgc-registry` THẬT qua subprocess.
//! Không bao giờ gọi `cargo` con (tranh lock với cargo test → treo — bài học e2e cũ).
// (E2E harness — black-box: drives the REAL binaries via subprocess. Never spawns
// nested cargo — it deadlocks against the outer cargo test.)
//
// Yêu cầu: build trước bằng `cargo build -p mgc --bin mgc -p mgc-registry-server --bin mgc-registry`
// (Prerequisite: build both binaries first; harness fails fast with that hint.)

use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

/// Token admin dùng nội bộ cho registry local trong test.
pub const TEST_ADMIN_TOKEN: &str = "e2e-admin-token";

/// Root workspace (tests/e2e/../..).
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root must exist")
}

fn cargo_target_dir(root: &Path, invocation_dir: &Path, configured: Option<PathBuf>) -> PathBuf {
    match configured {
        Some(path) if path.is_absolute() => path,
        Some(path) => invocation_dir.join(path),
        None => root.join("target"),
    }
}

/// Cổng loopback trống; sandbox cấm socket → trả None để test skip êm.
// (Free loopback port; PermissionDenied → None so the test skips quietly.)
pub fn free_port() -> Option<u16> {
    match TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => Some(listener.local_addr().expect("bound listener").port()),
        Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => {
            eprintln!("skipping socket-backed e2e test in sandbox: {err}");
            None
        }
        Err(err) => panic!("failed to allocate e2e test port: {err}"),
    }
}

/// Tìm binary đã build: env `MGC_E2E_BIN_DIR` → `CARGO_TARGET_DIR` → target mặc định.
/// Không tìm thấy → panic kèm lệnh build (fail-fast, không đệ quy cargo).
// (Locate a prebuilt binary in the configured Cargo target directory; fail fast with build hint.)
pub fn resolve_bin(name: &str) -> PathBuf {
    let root = workspace_root();
    let mut candidates = Vec::new();
    if let Ok(dir) = std::env::var("MGC_E2E_BIN_DIR") {
        candidates.push(PathBuf::from(dir).join(name));
    }
    let invocation_dir = std::env::current_dir().expect("Cargo E2E current directory must exist");
    let target_dir = cargo_target_dir(
        &root,
        &invocation_dir,
        std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from),
    );
    candidates.push(target_dir.join("debug").join(name));
    candidates.push(target_dir.join("release").join(name));

    candidates
        .into_iter()
        .find(|p| p.exists())
        .unwrap_or_else(|| {
            panic!(
                "binary '{name}' not found — run `cargo build -p mgc --bin mgc -p mgc-registry-server --bin mgc-registry` first"
            )
        })
}

/// Quản lý vòng đời mgc-registry local cho 1 test.
// (Local registry server lifecycle for one test.)
pub struct RegistryServer {
    child: Child,
    port: u16,
}

impl RegistryServer {
    /// Spawn registry trên cổng chỉ định + store dir tạm.
    pub fn spawn(store_dir: &Path, port: u16) -> Self {
        let child = Command::new(resolve_bin("mgc-registry"))
            .arg("--port")
            .arg(port.to_string())
            .arg("--store-dir")
            .arg(store_dir)
            .arg("--admin-token")
            .arg(TEST_ADMIN_TOKEN)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn mgc-registry");
        Self { child, port }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    /// Chờ cổng listen (tối đa 15s) — sai thì panic rõ ràng.
    // (Wait until the port is listening — max 15s, then panic.)
    pub fn wait_ready(&self) {
        let deadline = Instant::now() + Duration::from_secs(15);
        while Instant::now() < deadline {
            if TcpListener::bind(("127.0.0.1", self.port)).is_err() {
                return; // port đã bị chiếm → server đang listen
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        panic!("mgc-registry did not become ready within 15s");
    }

    pub fn shutdown(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for RegistryServer {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Chạy `mgc <args>` trong `cwd` với env bổ sung → stdout nếu thành công.
/// Lỗi → panic kèm stderr đầy đủ (debug friendly).
// (Run the real `mgc` binary; panics with full stderr on failure.)
pub fn mgc(cwd: &Path, args: &[&str], envs: &[(&str, &str)]) -> String {
    let output = Command::new(resolve_bin("mgc"))
        .current_dir(cwd)
        .args(args)
        .env("MAGICORE_TEMPLATE_DIR", workspace_root().join("templates"))
        .envs(envs.iter().copied())
        .output()
        .expect("spawn mgc");
    assert!(
        output.status.success(),
        "mgc {:?} failed:\n{}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_directory_uses_configured_relative_or_absolute_path() {
        let fixture = tempfile::tempdir().expect("create portable target path fixture");
        let root = fixture.path().join("workspace");
        let invocation_dir = fixture.path().join("invocation");
        let absolute_target = fixture.path().join("absolute-target");

        assert_eq!(
            cargo_target_dir(&root, &invocation_dir, Some(absolute_target.clone())),
            absolute_target
        );
        assert_eq!(
            cargo_target_dir(
                &root,
                &invocation_dir,
                Some(PathBuf::from("build/target-cache"))
            ),
            invocation_dir.join("build/target-cache")
        );
        assert_eq!(
            cargo_target_dir(&root, &invocation_dir, None),
            root.join("target")
        );
    }
}
