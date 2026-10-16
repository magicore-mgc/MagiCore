//! Live product E2E for MagiCore's ESP32-C3 Rust install and build path.
//! E2E sản phẩm thật cho luồng cài và build Rust ESP32-C3 của MagiCore.

#![allow(clippy::unwrap_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

const ESP32_C3_TARGET: &str = "riscv32imc-unknown-none-elf";

fn mgc_binary() -> PathBuf {
    if let Some(value) = std::env::var_os("MGC_TEST_CLI_BINARY") {
        let path = PathBuf::from(value);
        if path.is_file() {
            eprintln!("MagiCore IoT product binary: {}", path.display());
            return path;
        }
        #[cfg(windows)]
        {
            let executable = path.with_extension("exe");
            if executable.is_file() {
                eprintln!("MagiCore IoT product binary: {}", executable.display());
                return executable;
            }
        }
        panic!(
            "MGC_TEST_CLI_BINARY does not identify an executable: {}",
            path.display()
        );
    }
    PathBuf::from(env!("CARGO_BIN_EXE_mgc"))
}

fn copy_fixture(destination: &Path) {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CLI crate has a repository parent")
        .join("esp32-feature-e2e");
    fs::create_dir_all(destination.join("src")).unwrap();
    fs::create_dir_all(destination.join(".cargo")).unwrap();
    for file in ["Cargo.toml", "mgc.toml", ".mgc.core"] {
        fs::copy(repository.join(file), destination.join(file)).unwrap();
    }
    fs::copy(
        repository.join(".cargo").join("config.toml"),
        destination.join(".cargo").join("config.toml"),
    )
    .unwrap();
    fs::copy(
        repository.join("src").join("main.rs"),
        destination.join("src").join("main.rs"),
    )
    .unwrap();
}

fn preserve_host_toolchain_paths(command: &mut Command) {
    let host_home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from);
    for (variable, directory) in [("CARGO_HOME", ".cargo"), ("RUSTUP_HOME", ".rustup")] {
        if let Some(path) = std::env::var_os(variable)
            .map(PathBuf::from)
            .or_else(|| host_home.as_ref().map(|home| home.join(directory)))
        {
            command.env(variable, path);
        }
    }

    // Use Cargo and rustc from the same rustup toolchain. Some developer
    // machines put a package-manager Rust ahead of rustup in PATH; that
    // binary may not contain the target installed by this E2E's rustup step.
    // (Dùng Cargo/rustc cùng toolchain rustup; Rust cài qua package manager
    // có thể đứng trước PATH nhưng thiếu target mà E2E vừa cài.)
    let rustc = Command::new("rustup")
        .args(["which", "rustc"])
        .output()
        .expect("locate the rustup-selected rustc for the IoT build E2E");
    assert!(
        rustc.status.success(),
        "rustup which rustc failed: {}",
        String::from_utf8_lossy(&rustc.stderr)
    );
    let rustc = String::from_utf8(rustc.stdout)
        .expect("rustup rustc path is UTF-8")
        .trim()
        .to_string();
    let toolchain_bin = PathBuf::from(&rustc)
        .parent()
        .expect("rustc executable has a toolchain directory")
        .to_path_buf();
    let mut paths = vec![toolchain_bin];
    if let Some(existing) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&existing));
    }
    command.env(
        "PATH",
        std::env::join_paths(paths).expect("join host toolchain PATH entries"),
    );

    #[cfg(target_os = "macos")]
    {
        // Some locally installed Rustup toolchains place libLLVM in the
        // toolchain's top-level `lib`, while rust-lld's @rpath points inside
        // `lib/rustlib/<host>/lib`. Expose the actual toolchain library dir
        // to the linker process so the real ESP32 link step can run.
        // (Một số Rustup toolchain đặt libLLVM ở `lib` cấp cao nhất, còn
        // @rpath của rust-lld trỏ vào `lib/rustlib/<host>/lib`.)
        if let Some(library_dir) = PathBuf::from(&rustc)
            .parent()
            .and_then(Path::parent)
            .map(|root| root.join("lib"))
            .filter(|directory| directory.join("libLLVM.dylib").is_file())
        {
            let mut library_paths = vec![library_dir];
            if let Some(existing) = std::env::var_os("DYLD_LIBRARY_PATH") {
                library_paths.extend(std::env::split_paths(&existing));
            }
            command.env(
                "DYLD_LIBRARY_PATH",
                std::env::join_paths(library_paths)
                    .expect("join host Rust toolchain library paths"),
            );
        }
    }
}

fn run_mgc(args: &[&str], project: &Path, home: &Path, data: &Path) -> Output {
    let mut command = Command::new(mgc_binary());
    command
        .args(args)
        .current_dir(project)
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env("MGC_DATA_DIR", data)
        .env_remove("MGC_TEST_CLI_BINARY")
        .env_remove("MAGICORE_STORE_ROOT")
        .env_remove("MGC_CRATES_INDEX_URL")
        .env_remove("MGC_CRATES_DOWNLOAD_URL")
        .env_remove("MGC_OFFLINE_MODE");
    preserve_host_toolchain_paths(&mut command);
    command.output().expect("spawn the built MagiCore CLI")
}

/// Exercise the built CLI against crates.io, then prove its lockfile is
/// accepted unchanged by the actual ESP32-C3 Cargo build.
/// Chạy CLI đã build trên crates.io, rồi xác minh Cargo build dùng lock nguyên vẹn.
#[test]
#[ignore = "requires live crates.io access and the ESP32-C3 Rust target"]
fn install_iot_matches_cargo_lock_and_builds_esp32c3() {
    let project = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();
    let data = TempDir::new().unwrap();
    copy_fixture(project.path());

    // The checked-in fixture contains only source input. Generated state must
    // come from the product commands exercised below, never a committed lock
    // or cache that could make this E2E pass without installing.
    // Fixture trong repo chỉ chứa source đầu vào. State phải do lệnh sản phẩm
    // bên dưới sinh ra, không được dựa vào lock/cache cũ để test pass giả.
    for generated_path in ["mgc.lock", "Cargo.lock", ".magicore", ".mgc-optimizer"] {
        assert!(
            !project.path().join(generated_path).exists(),
            "copied ESP32 fixture must start without generated {generated_path}"
        );
    }

    let install = run_mgc(&["install-iot"], project.path(), home.path(), data.path());
    assert!(
        install.status.success(),
        "mgc install-iot failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&install.stdout),
        String::from_utf8_lossy(&install.stderr)
    );

    let cargo_lock = project.path().join("Cargo.lock");
    let lock_before_build = fs::read(&cargo_lock).expect("install writes Cargo.lock");
    let mgc_lock = fs::read_to_string(project.path().join("mgc.lock"))
        .expect("install writes canonical mgc.lock");
    assert!(mgc_lock.contains("esp-hal"), "mgc.lock must pin esp-hal");

    // These are MagiCore's user-global, cross-project package locations.
    // The fixture starts empty; install must populate both the Cargo-layout
    // cache/source and the versioned CAS, not create a project-local package
    // copy. (Fixture bắt đầu sạch; package nằm trong store dùng chung.)
    let cargo_store = data.path().join("store/cargo/registry");
    let archive_dir = cargo_store.join("cache");
    let source_dir = cargo_store.join("src");
    let archive_count = fs::read_dir(&archive_dir)
        .expect("MagiCore writes downloaded Rust archives to the shared store")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "crate"))
        .count();
    let source_count = fs::read_dir(&source_dir)
        .expect("MagiCore materializes Rust sources in the shared store")
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .count();
    assert!(
        archive_count > 0,
        "shared store must contain verified .crate archives"
    );
    assert!(
        source_count > 0,
        "shared store must contain verified crate sources"
    );
    assert!(
        data.path().join("store/v3").is_dir(),
        "MagiCore CAS must be initialized"
    );

    let optimize = run_mgc(&["optimizer"], project.path(), home.path(), data.path());
    assert!(
        optimize.status.success(),
        "mgc optimizer failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&optimize.stdout),
        String::from_utf8_lossy(&optimize.stderr)
    );
    let optimizer_profile = project
        .path()
        .join(".mgc-optimizer/rust_cargo_profile.toml");
    let optimizer_config = fs::read_to_string(&optimizer_profile)
        .expect("Rust IoT optimizer writes a Cargo profile consumed by build");
    assert!(optimizer_config.contains("[profile.dev]"));
    assert!(optimizer_config.contains("[profile.release]"));

    let mut build_command = Command::new(mgc_binary());
    build_command
        .arg("build")
        .current_dir(project.path())
        .env("HOME", home.path())
        .env("USERPROFILE", home.path())
        .env("MGC_DATA_DIR", data.path())
        .env("CARGO_BUILD_TARGET", ESP32_C3_TARGET)
        .env_remove("MGC_TEST_CLI_BINARY")
        .env_remove("MAGICORE_STORE_ROOT")
        .env_remove("MGC_CRATES_INDEX_URL")
        .env_remove("MGC_CRATES_DOWNLOAD_URL");
    preserve_host_toolchain_paths(&mut build_command);
    let build = build_command
        .output()
        .expect("spawn the built MagiCore CLI");
    assert!(
        build.status.success(),
        "mgc build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    let build_output = format!(
        "{}{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    assert!(
        build_output.contains("Loaded Rust Lib optimizer config"),
        "mgc build must consume the generated optimizer profile:\n{build_output}"
    );
    let audit_log = fs::read_to_string(project.path().join(".magicore/exec.log"))
        .expect("mgc build records its Cargo process under project-local .magicore");
    assert!(
        audit_log.lines().any(|line| {
            serde_json::from_str::<serde_json::Value>(line)
                .ok()
                .is_some_and(|entry| {
                    entry.get("cmd").and_then(serde_json::Value::as_str) == Some("cargo")
                        && entry
                            .get("args")
                            .and_then(serde_json::Value::as_array)
                            .is_some_and(|args| args.iter().any(|arg| arg == "build"))
                })
        }),
        "project-local .magicore/exec.log must contain the audited Cargo build"
    );
    assert_eq!(
        fs::read(&cargo_lock).expect("Cargo.lock remains readable"),
        lock_before_build,
        "Cargo --locked build must not rewrite the product-generated lockfile"
    );
}
