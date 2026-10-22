//! Rollback lifecycle E2E cases. / Test E2E lifecycle cho rollback.

use crate::native_ownership_matrix_e2e::support::{
    HermeticPypi, MatrixSandbox, lib_project, python_manifest,
};
use tempfile::TempDir;

pub(crate) fn matrix_remove_rolls_back_on_injected_post_write_failure() {
    // Deterministic transaction proof: fail immediately after the manifest
    // write and before any network/install work; rollback must restore the
    // original manifest and leave no mutation journal behind.
    let project = TempDir::new().unwrap();
    let (f, _) = python_manifest();
    lib_project(
        project.path(),
        "python",
        f,
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\nrequires-python = \">=3.11\"\ndependencies = [\"six==1.17.0\", \"attrs==23.1.0\"]\n",
    );
    let before = std::fs::read_to_string(project.path().join("pyproject.toml")).unwrap();
    assert!(
        before.contains("attrs"),
        "fixture must carry attrs:\n{before}"
    );
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(
        &["remove-lib", "attrs"],
        project.path(),
        &[("MGC_MUTATION_FAILPOINT", "after-manifest-write")],
    );
    assert_ne!(
        code,
        Some(0),
        "injected post-write failure must fail the remove:\n{out}"
    );
    assert!(
        out.contains("MGC_MUTATION_FAILPOINT"),
        "fault must be observable:\n{out}"
    );
    let after = std::fs::read_to_string(project.path().join("pyproject.toml")).unwrap();
    assert!(
        after.contains("attrs"),
        "rolled-back manifest must still carry attrs:\n{after}"
    );
    assert!(
        !project.path().join("mgc.lock").exists(),
        "rollback must preserve the prior absence of mgc.lock"
    );
    sandbox.assert_no_spawn("atomic remove-lib");
}

pub(crate) fn matrix_remove_rolls_back_hermetic_registry() {
    // Fully hermetic rollback (NO live PyPI): warm-install two fake
    // distributions from the fixture registry, break the artifact fetch,
    // remove with a cold store — the op must fail AND restore manifest
    // (both deps) plus byte-identical lock, with zero toolchain spawn.
    // (Rollback hermetic hoàn toàn — package giả, registry giả.)
    let project = TempDir::new().unwrap();
    lib_project(
        project.path(),
        "python",
        "pyproject.toml",
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\nrequires-python = \">=3.11\"\ndependencies = [\"mgc-race-pypkg-a==9.9.9\", \"mgc-race-pypkg-b==9.9.9\"]\n",
    );
    let pypi = HermeticPypi::new(&[("mgc_race_pypkg_a", "9.9.9"), ("mgc_race_pypkg_b", "9.9.9")]);
    let env = [("MGC_PYPI_INDEX_URL", pypi.url.as_str())];
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(&["install-lib"], project.path(), &env);
    assert_eq!(code, Some(0), "hermetic warm install must succeed:\n{out}");
    let lock_path = project.path().join("mgc.lock");
    let lock_body = std::fs::read_to_string(&lock_path).unwrap();
    assert!(
        lock_body.contains(&pypi.url),
        "lock must reference the fixture registry (hermetic proof):\n{lock_body}"
    );
    let lock_bytes_before = std::fs::read(&lock_path).unwrap();

    let cold = MatrixSandbox::new();
    let mut env = env.to_vec();
    env.push(("MGC_MUTATION_FAILPOINT", "before-complete"));
    let (code, out) = cold.run_with_env(&["remove-lib", "mgc-race-pypkg-b"], project.path(), &env);
    assert_ne!(
        code,
        Some(0),
        "injected failure after the hermetic install tail must fail:\n{out}"
    );
    assert!(
        out.contains("MGC_MUTATION_FAILPOINT"),
        "fault must be observable:\n{out}"
    );
    let manifest_after = std::fs::read_to_string(project.path().join("pyproject.toml")).unwrap();
    for dep in ["mgc-race-pypkg-a", "mgc-race-pypkg-b"] {
        assert!(
            manifest_after.contains(dep),
            "rolled-back manifest must still carry {dep}:\n{manifest_after}"
        );
    }
    assert_eq!(
        std::fs::read(&lock_path).unwrap(),
        lock_bytes_before,
        "rolled-back lock must be byte-identical"
    );
    cold.assert_no_spawn("hermetic remove-lib");
}
