//! Ai Lifecycle lifecycle E2E cases. / Test E2E lifecycle cho ai lifecycle.

use crate::native_ownership_matrix_e2e::support::{
    HermeticPypi, MatrixSandbox, ai_project, native_add_cell_with_env,
    native_install_cell_with_env, write,
};
use tempfile::TempDir;

pub(crate) fn matrix_ai_python_native_list_remove_use_verified_adapter_without_tools() {
    let project = TempDir::new().unwrap();
    ai_project(project.path());
    let sandbox = MatrixSandbox::new();

    let (code, out) = sandbox.run(&["list-ai"], project.path());
    assert_eq!(code, Some(0), "native list-ai must proceed:\n{out}");
    sandbox.assert_no_spawn("list-ai (native ai/python)");

    let (code, out) = sandbox.run(&["remove-ai", "not-present"], project.path());
    assert_eq!(code, Some(0), "native remove-ai no-op must proceed:\n{out}");
    sandbox.assert_no_spawn("remove-ai (native ai/python)");
}

pub(crate) fn matrix_ai_python_add_install() {
    let pypi = HermeticPypi::new(&[("six", "1.17.0")]);
    let env = [("MGC_PYPI_INDEX_URL", pypi.url.as_str())];
    native_add_cell_with_env(
        ai_project,
        &["add-ai", "six"],
        "pyproject.toml",
        "six",
        "six",
        &env,
    );
    native_install_cell_with_env(ai_project, &["add-ai", "six"], &["install-ai"], &env);
    pypi.assert_required_routes_used();
}

pub(crate) fn matrix_ai_foreign_python_manifest_update_refuses_without_mutation() {
    let project = TempDir::new().unwrap();
    write(
        project.path(),
        "mgc.toml",
        "name = \"m\"\necosystem = \"ai\"\n[ai]\nframework = \"python-agent\"\n",
    );
    write(project.path(), "requirements.txt", "six==1.15.0\n");
    let manifest = project.path().join("requirements.txt");
    let before = std::fs::read(&manifest).unwrap();

    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run(&["update-ai", "six"], project.path());
    assert_ne!(
        code,
        Some(0),
        "foreign Python lane must refuse update:\n{out}"
    );
    assert!(
        out.contains("does not yet own the complete native dependency lifecycle"),
        "refusal must identify missing MGC ownership, not an unrelated package error:\n{out}"
    );
    assert_eq!(
        std::fs::read(&manifest).unwrap(),
        before,
        "refused foreign-lane update must not rewrite the requirements file"
    );
    sandbox.assert_no_spawn("AI update with requirements.txt");
}
