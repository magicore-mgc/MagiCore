//! Lib Lifecycle lifecycle E2E cases. / Test E2E lifecycle cho lib lifecycle.

use crate::native_ownership_matrix_e2e::support::{
    HermeticCrates, HermeticGo, HermeticMaven, HermeticNuget, HermeticPypi, MatrixSandbox,
    dotnet_manifest, go_manifest, java_manifest, lib_project, mgc_binary, native_add_cell_with_env,
    native_install_cell_with_env, python_manifest, rust_manifest, write,
};
use std::process::Command;
use tempfile::TempDir;

pub(crate) fn matrix_lib_python_add_install_list_frozen() {
    let setup = |d: &std::path::Path| {
        let (f, b) = python_manifest();
        lib_project(d, "python", f, &b);
    };
    let pypi = HermeticPypi::new(&[("six", "1.17.0")]);
    let env = [("MGC_PYPI_INDEX_URL", pypi.url.as_str())];
    native_add_cell_with_env(
        setup,
        &["add-lib", "six"],
        "pyproject.toml",
        "six",
        "six",
        &env,
    );
    native_install_cell_with_env(setup, &["add-lib", "six"], &["install-lib"], &env);
    // List is a spawn-free manifest read.
    {
        let project = TempDir::new().unwrap();
        setup(project.path());
        let sandbox = MatrixSandbox::new();
        let (code, out) = sandbox.run_with_env(&["add-lib", "six"], project.path(), &env);
        assert_eq!(code, Some(0), "setup add must succeed:\n{out}");
        let (code, out) = sandbox.run_with_env(&["list-lib"], project.path(), &env);
        assert_eq!(code, Some(0), "list-lib must succeed:\n{out}");
        assert!(out.contains("six"), "list must show six:\n{out}");
        sandbox.assert_no_spawn("list-lib");
    }
    // Frozen replays the lock with zero spawn.
    {
        let project = TempDir::new().unwrap();
        setup(project.path());
        let sandbox = MatrixSandbox::new();
        let (code, out) = sandbox.run_with_env(&["add-lib", "six"], project.path(), &env);
        assert_eq!(code, Some(0), "setup add must succeed:\n{out}");
        let (code, out) = sandbox.run_with_env(&["install", "--frozen"], project.path(), &env);
        assert_eq!(code, Some(0), "frozen install must succeed:\n{out}");
        sandbox.assert_no_spawn("install --frozen");
    }
    pypi.assert_required_routes_used();
}

pub(crate) fn matrix_lib_rust_add_install() {
    run_lib_rust_build_e2e("3");
}

pub(crate) fn matrix_lib_rust_v4_add_install_build() {
    run_lib_rust_build_e2e("v4");
}

fn run_lib_rust_build_e2e(schema: &str) {
    let setup = |d: &std::path::Path| {
        let (f, b) = rust_manifest();
        lib_project(d, "rust", f, &b);
    };
    let registry = HermeticCrates::new();
    let env = [
        ("MGC_CRATES_INDEX_URL", registry.index_url.as_str()),
        ("MGC_CRATES_DOWNLOAD_URL", registry.download_url.as_str()),
    ];
    let project = TempDir::new().unwrap();
    setup(project.path());
    let config_path = project.path().join("mgc.toml");
    let mut config = std::fs::read_to_string(&config_path).unwrap();
    config.push_str(&format!("\n[lock]\nschema = \"{schema}\"\n"));
    std::fs::write(config_path, config).unwrap();
    std::fs::create_dir_all(project.path().join("src")).unwrap();
    std::fs::write(
        project.path().join("src/lib.rs"),
        "pub fn call_fixture() { mgcfixture::fixture(); }\n",
    )
    .unwrap();
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(&["add-lib", "mgcfixture"], project.path(), &env);
    assert_eq!(
        code,
        Some(0),
        "native Rust add ({schema}) must succeed:\n{out}"
    );
    let (code, out) = sandbox.run_with_env(&["install-lib"], project.path(), &env);
    assert_eq!(
        code,
        Some(0),
        "native Rust install ({schema}) must succeed:\n{out}"
    );

    let lock_before = std::fs::read(project.path().join("mgc.lock")).unwrap();
    let cargo_lock_before = std::fs::read(project.path().join("Cargo.lock")).ok();
    let cached_source = sandbox
        .store_root()
        .join("cargo/registry/src/mgcfixture-1.0.0/Cargo.toml");
    assert!(
        cached_source.is_file(),
        "online install must materialize Cargo source"
    );
    std::fs::remove_dir_all(cached_source.parent().unwrap()).unwrap();

    // An unreachable registry URL proves offline reinstall never falls back
    // to crates.io/local registry access. It must rematerialize from the
    // SHA-256-verified MagiCore Cargo cache while preserving both lockfiles.
    let closed_registry = [
        ("MGC_CRATES_INDEX_URL", "http://127.0.0.1:1"),
        ("MGC_CRATES_DOWNLOAD_URL", "http://127.0.0.1:1"),
    ];
    let (code, out) = sandbox.run_with_env(
        &["install-lib", "--offline"],
        project.path(),
        &closed_registry,
    );
    assert_eq!(
        code,
        Some(0),
        "offline Rust reinstall ({schema}) must use cache:\n{out}"
    );
    assert_eq!(
        std::fs::read(project.path().join("mgc.lock")).unwrap(),
        lock_before
    );
    assert_eq!(
        std::fs::read(project.path().join("Cargo.lock")).ok(),
        cargo_lock_before
    );
    assert!(
        cached_source.is_file(),
        "offline replay must restore Cargo source"
    );

    // Use a clean Cargo home and the real host toolchain to prove the build
    // consumes MGC's verified directory source without registry fallback.
    // (Dùng Cargo home sạch và toolchain thật để chứng minh build chỉ đọc
    // directory source đã verify của MGC, không fallback registry.)
    let mut build = Command::new(mgc_binary());
    build
        .args(["build", "--core", "lib"])
        .current_dir(project.path())
        .env("HOME", sandbox.home_dir.path())
        .env("CARGO_HOME", sandbox.home_dir.path().join(".cargo"))
        .env("MAGICORE_STORE_ROOT", sandbox.store_root())
        .env("MGC_CRATES_INDEX_URL", "http://127.0.0.1:1")
        .env_remove("MGC_COMPAT_RUNTIME");
    if let Some(rustup_home) = std::env::var_os("RUSTUP_HOME").or_else(|| {
        std::env::var_os("HOME").map(|home| {
            std::path::PathBuf::from(home)
                .join(".rustup")
                .into_os_string()
        })
    }) {
        build.env("RUSTUP_HOME", rustup_home);
    }
    let output = build.output().expect("run mgc build with real Cargo");
    let build_output = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.status.success(),
        "non-empty Rust build ({schema}) must use MGC's offline source:\n{build_output}"
    );
    assert_eq!(
        std::fs::read(project.path().join("mgc.lock")).unwrap(),
        lock_before,
        "Cargo build must not rewrite canonical mgc.lock"
    );
    sandbox.assert_no_spawn("Rust add/install/offline reinstall");
}

pub(crate) fn rust_lib_reuses_verified_shared_artifacts_across_project_roots() {
    let registry = HermeticCrates::with_download_expectation(Some(1));
    let env = [
        ("MGC_CRATES_INDEX_URL", registry.index_url.as_str()),
        ("MGC_CRATES_DOWNLOAD_URL", registry.download_url.as_str()),
    ];
    let sandbox = MatrixSandbox::new();

    for project_name in ["first-project", "second-project"] {
        let project = TempDir::new().unwrap();
        let (_, manifest) = rust_manifest();
        lib_project(project.path(), "rust", "Cargo.toml", &manifest);
        write(project.path(), ".mgc.core", "lib\n");

        let (code, output) = sandbox.run_with_env(&["add-lib", "mgcfixture"], project.path(), &env);
        assert_eq!(
            code,
            Some(0),
            "{project_name}: native add-lib must resolve the fixture crate:\n{output}"
        );
        let (code, output) = sandbox.run_with_env(&["install-lib"], project.path(), &env);
        assert_eq!(
            code,
            Some(0),
            "{project_name}: native install-lib must succeed:\n{output}"
        );
        assert!(project.path().join("mgc.lock").is_file());
        assert!(project.path().join("Cargo.lock").is_file());
        sandbox.assert_no_spawn("native Rust cross-project install");
    }

    let crate_cache = sandbox
        .store_root()
        .join("cargo/registry/cache/mgcfixture-1.0.0.crate");
    let crate_source = sandbox
        .store_root()
        .join("cargo/registry/src/mgcfixture-1.0.0/.cargo-checksum.json");
    assert!(
        crate_cache.is_file(),
        "archive must persist in shared Cargo cache"
    );
    assert!(
        crate_source.is_file(),
        "source must persist in shared Cargo store"
    );

    // Prepare a third project while the verified shared archive is intact;
    // `add-lib` itself includes an install pass for newly added packages.
    // (Tạo project thứ ba khi archive còn nguyên; `add-lib` cũng cài package.)
    let project = TempDir::new().unwrap();
    let (_, manifest) = rust_manifest();
    lib_project(project.path(), "rust", "Cargo.toml", &manifest);
    write(project.path(), ".mgc.core", "lib\n");
    let (code, output) = sandbox.run_with_env(&["add-lib", "mgcfixture"], project.path(), &env);
    assert_eq!(code, Some(0), "setup third project failed:\n{output}");

    // A tampered shared archive is an integrity error, never a cache miss
    // that silently falls back to downloading from the registry.
    // (Archive shared bị sửa phải báo integrity, không lén tải thay thế.)
    std::fs::write(&crate_cache, b"tampered archive").unwrap();
    let (code, output) = sandbox.run_with_env(&["install-lib"], project.path(), &env);
    assert_ne!(
        code,
        Some(0),
        "tampered shared archive must be refused:\n{output}"
    );
    assert!(
        output.contains("integrity mismatch"),
        "install must identify the corrupted shared artifact:\n{output}"
    );
    registry._mocks[1].assert();
}

pub(crate) fn matrix_lib_go_add_install() {
    let setup = |d: &std::path::Path| {
        let (f, b) = go_manifest();
        lib_project(d, "go", f, &b);
    };
    let proxy = HermeticGo::new();
    let env = [
        ("MGC_GO_PROXY_URL", proxy.url.as_str()),
        ("MGC_GO_SUM_URL", proxy.url.as_str()),
    ];
    native_add_cell_with_env(
        setup,
        &["add-lib", "github.com/google/uuid"],
        "go.mod",
        "github.com/google/uuid",
        "github.com/google/uuid",
        &env,
    );
    native_install_cell_with_env(
        setup,
        &["add-lib", "github.com/google/uuid"],
        &["install-lib"],
        &env,
    );
    proxy.assert_all_routes_used();
}

pub(crate) fn matrix_lib_dotnet_add_install() {
    let setup = |d: &std::path::Path| {
        let (f, b) = dotnet_manifest();
        lib_project(d, "dotnet", f, &b);
    };
    let registry = HermeticNuget::new();
    let env = [("MGC_NUGET_INDEX_URL", registry.index_url.as_str())];
    native_add_cell_with_env(
        setup,
        &["add-lib", "Newtonsoft.Json"],
        "m.csproj",
        "Newtonsoft.Json",
        "Newtonsoft.Json",
        &env,
    );
    native_install_cell_with_env(
        setup,
        &["add-lib", "Newtonsoft.Json"],
        &["install-lib"],
        &env,
    );
    registry.assert_all_routes_used();
}

pub(crate) fn matrix_lib_java_add_install() {
    let setup = |d: &std::path::Path| {
        let (f, b) = java_manifest();
        lib_project(d, "java", f, &b);
    };
    let registry = HermeticMaven::new();
    let env = [("MGC_MAVEN_REPO_URL", registry.url.as_str())];
    native_add_cell_with_env(
        setup,
        &["add-lib", "org.apache.commons:commons-lang3"],
        "pom.xml",
        "commons-lang3",
        "commons-lang3",
        &env,
    );
    native_install_cell_with_env(
        setup,
        &["add-lib", "org.apache.commons:commons-lang3"],
        &["install-lib"],
        &env,
    );
    registry.assert_all_routes_used();
}
