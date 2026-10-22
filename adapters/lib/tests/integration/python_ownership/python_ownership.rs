//! Lib Python ownership tests — kiểm thử ownership Python của Lib.
//! Owns pip refusal and foreign-lock fail-closed cases. — Sở hữu từ chối pip và lock ngoài fail-closed.

use mgc_lib_adapter::check_pip_allowed;
use mgc_types::adapter::PackageAdapter;
use mgc_types::capabilities::{ContentStoreProvider, DependencyResolver, LockfileProvider};

use super::support::tmp;

pub(crate) fn pip_allowlist_cannot_enable_external_package_manager() {
    let dir = tmp("pip-sec");
    std::fs::write(
        dir.join("mgc.toml"),
        "ecosystem = \"lib\"\n\n[lib]\nlanguage = \"python\"\npip_allowed_packages = [\"requests\", \"numpy\"]\n",
    )
    .unwrap();
    for name in ["requests", "numpy", "malicious-pkg"] {
        let err = check_pip_allowed(&dir, name).unwrap_err();
        assert!(err.to_string().contains("never invokes pip"));
        assert!(err.to_string().contains(name));
    }
}

pub(crate) async fn python_foreign_lock_keeps_native_mutation_and_install_unclaimed() {
    use mgc_types::capabilities::Capability;

    let dir = tmp("python-foreign-lock");
    std::fs::write(
        dir.join("pyproject.toml"),
        "[project]\nname = \"py-lib\"\ndependencies = [\"requests>=2.32\"]\n\n[tool.poetry.dependencies]\nrequests = \"^2.32\"\n",
    )
    .unwrap();
    std::fs::write(dir.join("poetry.lock"), "# foreign lock\n").unwrap();
    let adapter = mgc_lib_adapter::adapter_for_language(
        mgc_lib_adapter::LibLanguage::Python,
        &dir,
        None,
        None,
    )
    .unwrap();

    assert!(!adapter.manifest_owned());
    assert!(
        !adapter
            .capabilities()
            .contains(&Capability::DependencyResolver)
    );
    assert!(matches!(
        adapter.probe_dependency_resolver(),
        Err(mgc_types::MgError::Unsupported { .. })
    ));
    assert!(matches!(
        adapter.probe_lockfile_provider(),
        Err(mgc_types::MgError::Unsupported { .. })
    ));
    assert!(matches!(
        adapter.probe_content_store(),
        Err(mgc_types::MgError::Unsupported { .. })
    ));
    assert!(matches!(
        adapter.parse_manifest(&dir).await,
        Err(mgc_types::MgError::Unsupported { .. })
    ));
    assert!(matches!(
        adapter.list(&dir).await,
        Err(mgc_types::MgError::Unsupported { .. })
    ));
}
