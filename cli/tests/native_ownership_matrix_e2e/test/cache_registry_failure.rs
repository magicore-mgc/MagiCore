//! Cache Registry Failure lifecycle E2E cases. / Test E2E lifecycle cho cache registry failure.

use crate::native_ownership_matrix_e2e::support::{
    HermeticPypi, MatrixSandbox, lib_project, python_manifest, write,
};
use tempfile::TempDir;

pub(crate) fn matrix_python_cached_reinstall_survives_registry_outage() {
    // Warm install only from a hermetic local registry, then remove that
    // registry and point the client at a closed loopback port. This proves
    // the test itself never depends on public PyPI while validating cache
    // reuse. `cargo test --offline` does not disable network inside tests.
    // (Chỉ warm từ registry giả local, sau đó tắt registry.)
    let project = TempDir::new().unwrap();
    lib_project(
        project.path(),
        "python",
        "pyproject.toml",
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\nrequires-python = \">=3.11\"\ndependencies = []\n",
    );
    let pypi = HermeticPypi::new(&[("mgc_offline_reinstall_pypkg", "9.9.9")]);
    let registry = pypi.url.clone();
    let sandbox = MatrixSandbox::new();
    let online = [("MGC_PYPI_INDEX_URL", registry.as_str())];
    let (code, out) = sandbox.run_with_env(
        &[
            "add-lib",
            "mgc-offline-reinstall-pypkg",
            "--version",
            "9.9.9",
        ],
        project.path(),
        &online,
    );
    assert_eq!(code, Some(0), "hermetic add must succeed:\n{out}");
    let (code, out) = sandbox.run_with_env(&["install-lib"], project.path(), &online);
    assert_eq!(code, Some(0), "hermetic warm install must succeed:\n{out}");
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap();
    assert!(
        lock.contains(&registry),
        "lock must prove it was populated from the local fixture registry:\n{lock}"
    );
    drop(pypi);
    let (code, out) = sandbox.run_with_env(
        &["install-lib"],
        project.path(),
        &[("MGC_PYPI_INDEX_URL", "http://127.0.0.1:1")],
    );
    assert_eq!(
        code,
        Some(0),
        "cached reinstall after registry outage must succeed from the verified wheel cache:\n{out}"
    );
    sandbox.assert_no_spawn("cached install-lib during registry outage");

    let (code, out) = sandbox.run_with_env(
        &["install", "--offline"],
        project.path(),
        &[("MGC_PYPI_INDEX_URL", "http://127.0.0.1:1")],
    );
    assert_eq!(
        code,
        Some(0),
        "offline install must replay the verified Python lock and cached wheel without network:\n{out}"
    );
    sandbox.assert_no_spawn("offline native Python install");
    let (code, out) = sandbox.run_with_env(
        &["install-lib", "--offline"],
        project.path(),
        &[("MGC_PYPI_INDEX_URL", "http://127.0.0.1:1")],
    );
    assert_eq!(
        code,
        Some(0),
        "install-lib must preserve the explicit offline flag through per-core dispatch:\n{out}"
    );
    sandbox.assert_no_spawn("offline install-lib native Python");

    let lock_path = project.path().join("mgc.lock");
    let lock_before_mismatch = std::fs::read(&lock_path).unwrap();
    let manifest_path = project.path().join("pyproject.toml");
    let manifest_before_mismatch = std::fs::read(&manifest_path).unwrap();
    let manifest_text = String::from_utf8(manifest_before_mismatch.clone()).unwrap();
    let stale_manifest = manifest_text.replacen("9.9.9", "9.8.9", 1);
    assert_ne!(
        stale_manifest, manifest_text,
        "fixture must change the manifest pin"
    );
    std::fs::write(&manifest_path, stale_manifest).unwrap();
    let (code, out) = sandbox.run_with_env(
        &["install", "--offline"],
        project.path(),
        &[("MGC_PYPI_INDEX_URL", "http://127.0.0.1:1")],
    );
    assert_ne!(
        code,
        Some(0),
        "offline install must reject a stale lock:\n{out}"
    );
    assert!(
        out.to_lowercase().contains("lock") && !out.to_lowercase().contains("network"),
        "stale offline lock must fail as a lock error, not fall back to network resolution:\n{out}"
    );
    assert_eq!(
        std::fs::read(&lock_path).unwrap(),
        lock_before_mismatch,
        "stale offline install must preserve lock bytes"
    );
    std::fs::write(&manifest_path, &manifest_before_mismatch).unwrap();

    let cold_sandbox = MatrixSandbox::new();
    let cold_project = TempDir::new().unwrap();
    // Copy only inputs, leaving the package cache behind — chỉ chép đầu vào, bỏ cache cũ.
    for file in ["mgc.toml", "pyproject.toml", "mgc.lock"] {
        std::fs::copy(project.path().join(file), cold_project.path().join(file)).unwrap();
    }
    assert!(
        !cold_project.path().join(".magicore").exists(),
        "cold project must not inherit the warmed project-local package cache"
    );
    let lock_before = std::fs::read(cold_project.path().join("mgc.lock")).unwrap();
    let manifest_before = std::fs::read(cold_project.path().join("pyproject.toml")).unwrap();
    let (code, out) = cold_sandbox.run_with_env(
        &["install", "--offline"],
        cold_project.path(),
        &[("MGC_PYPI_INDEX_URL", "http://127.0.0.1:1")],
    );
    assert_ne!(
        code,
        Some(0),
        "offline install with a cold cache must fail instead of fetching:\n{out}"
    );
    assert!(
        out.contains("offline") && out.contains("cache"),
        "cold offline failure must explain the missing cache entry:\n{out}"
    );
    assert_eq!(
        std::fs::read(cold_project.path().join("mgc.lock")).unwrap(),
        lock_before
    );
    assert_eq!(
        std::fs::read(cold_project.path().join("pyproject.toml")).unwrap(),
        manifest_before
    );
    assert!(
        !cold_sandbox.store_root().join("pypi").exists(),
        "cold offline preflight must not create a package store"
    );
    cold_sandbox.assert_no_spawn("cold offline native Python install");
}

pub(crate) fn offline_flag_is_not_silently_dropped_for_other_cores() {
    let project = TempDir::new().unwrap();
    write(
        project.path(),
        "mgc.toml",
        "name = \"offline-app\"\necosystem = \"app\"\n[app]\nlanguage = \"flutter\"\n",
    );
    write(
        project.path(),
        "pubspec.yaml",
        "name: offline_app\ndependencies: {}\n",
    );
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run(&["install", "--offline"], project.path());
    assert_ne!(
        code,
        Some(0),
        "offline replay without a lock must fail explicitly:\n{out}"
    );
    assert!(
        out.to_lowercase().contains("mgc.lock"),
        "offline mode must reach the missing-lock gate:\n{out}"
    );
    assert!(!project.path().join("mgc.lock").exists());
    sandbox.assert_no_spawn("offline install without a lock");
}

pub(crate) fn matrix_registry_outage_fails_closed_no_hang() {
    // Dead registry + cold cache: honest network error, never a hang,
    // never a false success, never a spawn.
    let project = TempDir::new().unwrap();
    let (f, b) = python_manifest();
    lib_project(project.path(), "python", f, &b);
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(
        &["add-lib", "six"],
        project.path(),
        &[("MGC_PYPI_INDEX_URL", "http://127.0.0.1:1")],
    );
    assert_ne!(
        code,
        Some(0),
        "cold add against a dead registry must fail:\n{out}"
    );
    assert!(
        out.to_lowercase().contains("network")
            || out.to_lowercase().contains("connect")
            || out.to_lowercase().contains("refused"),
        "failure must name the network cause:\n{out}"
    );
    sandbox.assert_no_spawn("outage add-lib");
}

pub(crate) fn matrix_tampered_lock_fails_closed() {
    // Downgraded pin (out of manifest range): frozen must refuse, never
    // silently re-resolve.
    let project = TempDir::new().unwrap();
    let (f, b) = python_manifest();
    lib_project(project.path(), "python", f, &b);
    let pypi = HermeticPypi::new(&[("six", "1.17.0")]);
    let env = [("MGC_PYPI_INDEX_URL", pypi.url.as_str())];
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(&["add-lib", "six"], project.path(), &env);
    assert_eq!(code, Some(0), "setup add must succeed:\n{out}");
    let lock_path = project.path().join("mgc.lock");
    let tampered = std::fs::read_to_string(&lock_path)
        .unwrap()
        .replacen("1.17.0", "1.0.0", 1);
    assert_ne!(
        std::fs::read_to_string(&lock_path).unwrap(),
        tampered,
        "fixture must actually change"
    );
    std::fs::write(&lock_path, tampered).unwrap();
    let (code, out) = sandbox.run(&["install", "--frozen"], project.path());
    assert_ne!(
        code,
        Some(0),
        "frozen install on a tampered lock must fail:\n{out}"
    );
    sandbox.assert_no_spawn("tampered frozen install");
}

pub(crate) fn matrix_tampered_integrity_fails_verification() {
    // Corrupted integrity hash: verified fetch must reject, exit != 0.
    let project = TempDir::new().unwrap();
    let (f, b) = python_manifest();
    lib_project(project.path(), "python", f, &b);
    let pypi = HermeticPypi::new(&[("six", "1.17.0")]);
    let env = [("MGC_PYPI_INDEX_URL", pypi.url.as_str())];
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(&["add-lib", "six"], project.path(), &env);
    assert_eq!(code, Some(0), "setup add must succeed:\n{out}");
    let lock_path = project.path().join("mgc.lock");
    let body = std::fs::read_to_string(&lock_path).unwrap();
    let start = body.find("sha256-").expect("lock carries integrity");
    let mut tampered = body.clone();
    tampered.replace_range(start + 7..start + 8, "A");
    assert_ne!(body, tampered, "fixture must actually change");
    std::fs::write(&lock_path, tampered).unwrap();
    let (code, out) = sandbox.run(&["install-lib"], project.path());
    assert_ne!(
        code,
        Some(0),
        "install on corrupted integrity must fail:\n{out}"
    );
    assert!(
        out.to_lowercase().contains("integrity") || out.to_lowercase().contains("mismatch"),
        "failure must name integrity:\n{out}"
    );
    sandbox.assert_no_spawn("tampered install-lib");
}
