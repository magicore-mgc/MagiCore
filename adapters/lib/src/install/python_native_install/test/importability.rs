//! Python artifact importability preflight tests.
//! Test preflight khả năng import artifact Python.

use super::support::*;

#[test]
fn python_install_rejects_unimportable_artifacts_during_preflight() {
    let mut compiled = resolved("numpy", "2.0.0");
    compiled.tarball_url =
        "https://files.pythonhosted.org/numpy-2.0.0-cp312-cp312-macosx_14_0_arm64.whl".to_string();
    let error = validate_python_importable_graph(&ResolvedGraph {
        packages: vec![compiled],
    })
    .unwrap_err();
    assert!(error.to_string().contains("MGC_PYTHON_VERSION"));

    let mut source = resolved("example", "1.0.0");
    source.tarball_url = "https://files.pythonhosted.org/example-1.0.0.tar.gz".to_string();
    assert!(
        validate_python_importable_graph(&ResolvedGraph {
            packages: vec![source],
        })
        .is_err()
    );

    let mut pure = resolved("six", "1.17.0");
    pure.tarball_url = "https://files.pythonhosted.org/six-1.17.0-py3-none-any.whl".to_string();
    assert!(
        validate_python_importable_graph(&ResolvedGraph {
            packages: vec![pure],
        })
        .is_ok()
    );
}
