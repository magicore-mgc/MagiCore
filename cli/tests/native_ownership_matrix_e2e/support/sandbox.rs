//! Isolated process-spawn canary and matrix sandbox. / Canary spawn và sandbox ma trận cô lập.

use crate::native_ownership_matrix_e2e::support::{mgc_binary, write};
use std::process::Command;
use std::sync::OnceLock;
use tempfile::TempDir;

pub(crate) const ALL_TOOLS: &[&str] = &[
    "cargo",
    "rustc",
    "pip",
    "pip3",
    "uv",
    "python",
    "python3",
    "go",
    "dotnet",
    "mvn",
    "java",
    "gradle",
    "flutter",
    "dart",
    "swift",
    "pod",
    "xcodebuild",
    // Known package managers, build tools, runtimes, and shell launchers.
    // Dynamic or unlisted executables still require the static spawn audit.
    // (Canary phủ bộ tool đã biết; executable động phải qua static audit.)
    "node",
    "npm",
    "npx",
    "pnpm",
    "yarn",
    "bun",
    "bunx",
    "deno",
    "composer",
    "git",
    "terraform",
    "cdk",
    "pulumi",
    "wrangler",
    "pio",
    "platformio",
    "west",
    "cmd",
    "powershell",
    "pwsh",
    "where",
    "bash",
    "sh",
    "conda",
    "mamba",
    "micromamba",
    "poetry",
    "pdm",
    "pipx",
    "uvx",
    "cmake",
    "make",
    "ninja",
    "msbuild",
    "javac",
    "node-gyp",
];

pub(crate) struct MatrixSandbox {
    pub(crate) bin_dir: TempDir,
    pub(crate) home_dir: TempDir,
    pub(crate) canary_log: std::path::PathBuf,
}

pub(crate) struct CanaryExecutable {
    _directory: TempDir,
    path: std::path::PathBuf,
}

pub(crate) fn canary_executable() -> &'static std::path::Path {
    static CANARY: OnceLock<CanaryExecutable> = OnceLock::new();
    CANARY
        .get_or_init(|| {
            let directory = TempDir::new().expect("create native canary build directory");
            let source = directory.path().join("native_tool_canary.rs");
            let executable = directory.path().join(if cfg!(windows) {
                "native_tool_canary.exe"
            } else {
                "native_tool_canary"
            });
            std::fs::write(
                &source,
                r#"
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let tool = std::env::current_exe()
        .ok()
        .and_then(|path| path.file_stem().map(|stem| stem.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "unknown-tool".to_string());
    let directory = std::env::var_os("MGC_CANARY_LOG")
        .expect("MGC_CANARY_LOG must point to the isolated canary log directory");
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    let marker = std::path::PathBuf::from(directory)
        .join(format!("{tool}-{}-{nonce}", std::process::id()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(marker)
        .expect("create unique spawn marker");
    writeln!(file, "{tool}").expect("write spawn marker");
}
"#,
            )
            .expect("write native canary source");
            let compiler = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
            let output = Command::new(compiler)
                .arg("--edition=2021")
                .arg(&source)
                .arg("-o")
                .arg(&executable)
                .output()
                .expect("compile native process-spawn canary");
            assert!(
                output.status.success(),
                "compile native process-spawn canary failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            CanaryExecutable {
                _directory: directory,
                path: executable,
            }
        })
        .path
        .as_path()
}

impl MatrixSandbox {
    pub(crate) fn new() -> Self {
        let bin_dir = TempDir::new().unwrap();
        let home_dir = TempDir::new().unwrap();
        let log = bin_dir.path().join(".canary");
        std::fs::create_dir_all(&log).unwrap();
        let canary = canary_executable();
        for tool in ALL_TOOLS {
            let name = if cfg!(windows) {
                format!("{tool}.exe")
            } else {
                (*tool).to_string()
            };
            let executable = bin_dir.path().join(name);
            std::fs::copy(canary, &executable).unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&executable).unwrap().permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&executable, perms).unwrap();
            }
        }
        Self {
            bin_dir,
            home_dir,
            canary_log: log,
        }
    }

    pub(crate) fn run(&self, args: &[&str], cwd: &std::path::Path) -> (Option<i32>, String) {
        self.run_with_env(args, cwd, &[])
    }

    pub(crate) fn run_with_env(
        &self,
        args: &[&str],
        cwd: &std::path::Path,
        extra_env: &[(&str, &str)],
    ) -> (Option<i32>, String) {
        let mut cmd = Command::new(mgc_binary());
        cmd.args(args)
            .current_dir(cwd)
            .env("PATH", self.path_env())
            .env("HOME", self.home_dir.path())
            .env("MAGICORE_STORE_ROOT", self.store_root())
            .env("MGC_CANARY_LOG", self.canary_log.as_os_str())
            .env_remove("MGC_COMPAT_RUNTIME");
        #[cfg(windows)]
        cmd.env("USERPROFILE", self.home_dir.path())
            .env("APPDATA", self.home_dir.path().join("AppData/Roaming"))
            .env("LOCALAPPDATA", self.home_dir.path().join("AppData/Local"))
            .env("PATHEXT", ".COM;.EXE;.BAT;.CMD");
        for (k, v) in extra_env {
            cmd.env(k, v);
        }
        let out = cmd.output().expect("spawn mgc");
        (
            out.status.code(),
            format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            ),
        )
    }

    /// Resolve this sandbox's package-store root.
    /// Trả gốc package store riêng của sandbox này.
    pub(crate) fn store_root(&self) -> std::path::PathBuf {
        self.home_dir.path().join(".magicore/store")
    }

    pub(crate) fn path_env(&self) -> std::ffi::OsString {
        let mut paths = vec![self.bin_dir.path().to_path_buf()];
        if let Some(existing) = std::env::var_os("PATH") {
            paths.extend(std::env::split_paths(&existing));
        }
        std::env::join_paths(paths).unwrap()
    }

    pub(crate) fn assert_no_spawn(&self, what: &str) {
        let markers = std::fs::read_dir(&self.canary_log)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                let value = std::fs::read_to_string(entry.path()).unwrap_or_default();
                format!("{} ({})", entry.file_name().to_string_lossy(), value.trim())
            })
            .collect::<Vec<_>>();
        assert!(
            markers.is_empty(),
            "{what}: ZERO toolchain spawn allowed, got:\n{}",
            markers.join("\n")
        );
    }
}
