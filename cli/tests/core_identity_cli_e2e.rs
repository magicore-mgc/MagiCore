//! Release-binary E2E for core identity attestation through user-facing CLI commands.
//! E2E binary release cho attestation identity core qua lệnh CLI người dùng.

#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

fn mgc_binary() -> PathBuf {
    if let Some(value) = std::env::var_os("MGC_TEST_CLI_BINARY") {
        let path = PathBuf::from(value);
        if path.is_file() {
            eprintln!("MagiCore core-identity product binary: {}", path.display());
            return path;
        }
        #[cfg(windows)]
        {
            let executable = path.with_extension("exe");
            if executable.is_file() {
                eprintln!(
                    "MagiCore core-identity product binary: {}",
                    executable.display()
                );
                return executable;
            }
        }
        panic!(
            "MGC_TEST_CLI_BINARY does not identify an executable: {}",
            path.display()
        );
    }
    let binary = PathBuf::from(env!("CARGO_BIN_EXE_mgc"));
    eprintln!("MagiCore core-identity test binary: {}", binary.display());
    binary
}

fn run_mgc(
    binary: &Path,
    project: &Path,
    home: &Path,
    data: &Path,
    args: &[&str],
    anchor_mode: Option<&str>,
) -> Output {
    let mut command = Command::new(binary);
    command
        .args(args)
        .current_dir(project)
        .env_remove("MGC_TEST_CLI_BINARY")
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env("MGC_DATA_DIR", data)
        .env_remove("MAGICORE_STORE_ROOT")
        .env_remove("MAGICORE_WEB_REGISTRY_URL")
        .env_remove("MAGICORE_WEB_ALLOWED_REGISTRIES")
        .env_remove("MGC_TRUST_ANCHOR");
    if let Some(mode) = anchor_mode {
        command.env("MGC_TRUST_ANCHOR", mode);
    }
    command
        .output()
        .expect("spawn the selected MagiCore CLI binary")
}

fn output_text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn cli_requires_a_trusted_core_marker_after_it_is_renamed() {
    let project = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();
    let data = TempDir::new().unwrap();
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"identity-probe\"\necosystem = \"web\"\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("package.json"),
        "{\"name\":\"identity-probe\",\"version\":\"1.0.0\",\"dependencies\":{}}\n",
    )
    .unwrap();
    std::fs::write(project.path().join(".mgc.core"), "web\n").unwrap();
    let binary = mgc_binary();

    let anchor = run_mgc(
        &binary,
        project.path(),
        home.path(),
        data.path(),
        &["trust", "anchor"],
        None,
    );
    assert!(
        anchor.status.success(),
        "mgc trust anchor failed:\n{}",
        output_text(&anchor)
    );

    let trusted_list = run_mgc(
        &binary,
        project.path(),
        home.path(),
        data.path(),
        &["list", "--core", "web"],
        Some("require"),
    );
    assert!(
        trusted_list.status.success(),
        "a valid anchored project must remain usable in require mode:\n{}",
        output_text(&trusted_list)
    );

    std::fs::rename(
        project.path().join(".mgc.core"),
        project.path().join(".mgc.core.disabled"),
    )
    .unwrap();
    let renamed_list = run_mgc(
        &binary,
        project.path(),
        home.path(),
        data.path(),
        &["list", "--core", "web"],
        Some("require"),
    );
    let text = output_text(&renamed_list);
    assert!(
        !renamed_list.status.success(),
        "renaming the trusted marker must fail closed:\n{text}"
    );
    assert!(
        text.contains("core identity attestation mismatch")
            && text.contains("project core marker is missing"),
        "the CLI must identify the missing trusted marker:\n{text}"
    );
    assert!(
        !project.path().join(".mgc.core").exists(),
        "the failed operation must not silently recreate a replacement marker"
    );
}
