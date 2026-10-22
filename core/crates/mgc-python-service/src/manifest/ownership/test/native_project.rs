//! Native Python project ownership boundary tests.
//! Test ranh giới ownership project Python native.

use super::super::{supports_native_python_dependency_groups, supports_native_python_project};

#[test]
fn accepts_a_simple_project_and_rejects_foreign_lock_files() {
    let directory = tempfile::tempdir().expect("create project directory");
    std::fs::write(
        directory.path().join("pyproject.toml"),
        "[project]\nname='demo'\ndependencies=['requests>=2']\n",
    )
    .expect("write project manifest");
    assert!(supports_native_python_project(directory.path()));

    std::fs::write(directory.path().join("requirements.txt"), "requests>=2\n")
        .expect("write foreign dependency file");
    assert!(!supports_native_python_project(directory.path()));
}

#[test]
fn rejects_foreign_dependency_sources_and_unmodeled_groups() {
    for contents in [
        "[project]\nname='demo'\n[tool.uv.sources]\nrequests={git='https://example.invalid/r.git'}\n",
        "[project]\nname='demo'\n[dependency-groups]\ntest=['pytest']\n",
    ] {
        let directory = tempfile::tempdir().expect("create project directory");
        std::fs::write(directory.path().join("pyproject.toml"), contents)
            .expect("write project manifest");
        assert!(!supports_native_python_project(directory.path()));
    }
}

#[test]
fn dependency_group_policy_accepts_dev_strings_only() {
    let accepted: toml::Value = toml::from_str("[dependency-groups]\ndev=['pytest>=8']\n")
        .expect("parse supported dependency group");
    assert!(supports_native_python_dependency_groups(&accepted));

    let rejected: toml::Value =
        toml::from_str("[dependency-groups]\ndev=['pytest>=8']\ntest=['ruff']\n")
            .expect("parse unsupported dependency group");
    assert!(!supports_native_python_dependency_groups(&rejected));
}
