//! Preserve Python foreign-lock resolver refusal.
//! Giữ hành vi từ chối resolver với lock Python thuộc manager khác.

use crate::adapter::types::LibAdapter;
use crate::language::LibLanguage;
use mgc_types::capabilities::DependencyResolver;

#[test]
fn foreign_python_lock_is_rejected_by_resolver_probe() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pyproject.toml"),
        concat!(
            "[project]\n",
            "name = \"probe-fixture\"\n",
            "dependencies = [\"requests>=2\"]\n\n",
            "[tool.poetry.dependencies]\n",
            "requests = \"^2\"\n"
        ),
    )
    .unwrap();
    std::fs::write(project.path().join("poetry.lock"), "# foreign lock\n").unwrap();
    let adapter =
        LibAdapter::for_language(LibLanguage::Python, project.path(), None, None).unwrap();

    assert!(matches!(
        adapter.probe_dependency_resolver(),
        Err(mgc_types::MgError::Unsupported { .. })
    ));
}
