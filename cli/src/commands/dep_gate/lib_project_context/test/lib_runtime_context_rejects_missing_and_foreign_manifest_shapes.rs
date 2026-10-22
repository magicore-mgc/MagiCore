//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[cfg(feature = "lib")]
#[test]
fn lib_runtime_context_rejects_missing_and_foreign_manifest_shapes() {
    use crate::commands::dep_gate::lib_project_context;

    let cases = [
        (eco::TS, "package.json", "{}\n"),
        (
            eco::RUST,
            "Cargo.toml",
            "[package]\nname='x'\nversion='0.1.0'\n",
        ),
        (eco::GO, "go.mod", "module example.test/x\n\ngo 1.24\n"),
        (
            eco::PYTHON,
            "pyproject.toml",
            "[project]\nname='x'\nversion='0.1.0'\ndependencies=[]\n",
        ),
    ];
    for (ecosystem, manifest, contents) in cases {
        let missing = tempfile::tempdir().unwrap();
        std::fs::write(
            missing.path().join("mgc.toml"),
            format!("ecosystem='lib'\n\n[lib]\nlanguage='{ecosystem}'\n"),
        )
        .unwrap();
        assert!(
            matches!(
                owner_for(&lib_project_context(missing.path(), DepOp::Install)),
                DepOwner::Unsupported
            ),
            "missing {manifest} must not be advertised native"
        );

        let owned = tempfile::tempdir().unwrap();
        std::fs::write(
            owned.path().join("mgc.toml"),
            format!("ecosystem='lib'\n\n[lib]\nlanguage='{ecosystem}'\n"),
        )
        .unwrap();
        std::fs::write(owned.path().join(manifest), contents).unwrap();
        assert!(
            matches!(
                owner_for(&lib_project_context(owned.path(), DepOp::Install)),
                DepOwner::Native
            ),
            "owned {manifest} should remain native"
        );
    }

    let foreign_python = tempfile::tempdir().unwrap();
    std::fs::write(
        foreign_python.path().join("mgc.toml"),
        "ecosystem='lib'\n\n[lib]\nlanguage='python'\n",
    )
    .unwrap();
    std::fs::write(
        foreign_python.path().join("pyproject.toml"),
        "[project]\nname='x'\nversion='0.1.0'\ndependencies=[]\n",
    )
    .unwrap();
    std::fs::write(foreign_python.path().join("uv.lock"), "version = 1\n").unwrap();
    assert!(matches!(
        owner_for(&lib_project_context(foreign_python.path(), DepOp::Install)),
        DepOwner::Unsupported
    ));
}
