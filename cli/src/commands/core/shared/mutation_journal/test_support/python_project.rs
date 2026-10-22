//! Python project fixtures for mutation-journal tests.
//! Fixture project Python cho test mutation journal.

#[cfg(feature = "lib")]
use mgc_lockfile::project_lock::ProjectWriteLock;

pub(crate) fn python_fixture(root: &std::path::Path) {
    std::fs::write(
        root.join("pyproject.toml"),
        concat!(
            "[project]\nname = \"m\"\n",
            "version = \"0.1.0\"\nrequires-python = \">=3.11\"\n",
            "dependencies = [\"six==1.17.0\", \"attrs==23.1.0\"]\n",
        ),
    )
    .unwrap();
    std::fs::write(root.join("mgc.lock"), "LOCK-BEFORE").unwrap();
}

#[cfg(feature = "lib")]
pub(crate) fn test_adapter_and_lock(
    root: &std::path::Path,
) -> (
    mgc_lib_adapter::LibAdapter,
    mgc_lockfile::project_lock::ProjectWriteLock,
) {
    let adapter = mgc_lib_adapter::adapter_for(root, None, None)
        .unwrap()
        .expect("pyproject must detect a python lib adapter");
    let lock = ProjectWriteLock::acquire(root, std::time::Duration::from_secs(30)).unwrap();
    (adapter, lock)
}
