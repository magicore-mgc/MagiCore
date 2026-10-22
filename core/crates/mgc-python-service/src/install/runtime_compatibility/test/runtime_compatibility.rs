//! Python site-directory and importable-wheel tests.
//! Test thư mục site và wheel import được của Python.

use mgc_types::{PackageId, PackageName, ResolvedGraph, ResolvedPackage, Version};

fn resolved(name: &str, version: &str, tarball_url: &str) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::new(
            PackageName::new(name).unwrap(),
            Version::parse(version).unwrap(),
        ),
        integrity: "sha256-test".to_string(),
        tarball_url: tarball_url.to_string(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    }
}

#[test]
fn rejects_untrusted_site_path_segments() {
    assert!(crate::install::python_site_dirname("six", "1.17.0").is_ok());
    for (name, version) in [
        ("../../outside", "1.0.0"),
        ("pkg", "1.0.0-../../outside"),
        ("pkg", r"1.0.0-..\outside"),
    ] {
        assert!(
            crate::install::python_site_dirname(name, version).is_err(),
            "must reject {name}@{version}"
        );
    }
}

#[test]
fn rejects_compiled_wheels_without_a_host_compatible_python_abi() {
    let graph = ResolvedGraph {
        packages: vec![resolved(
            "numpy",
            "2.0.0",
            "https://files.pythonhosted.org/numpy-2.0.0-cp312-cp312-macosx_14_0_arm64.whl",
        )],
    };
    let error = crate::install::validate_python_importable_graph(&graph).unwrap_err();
    assert!(matches!(&error, mgc_types::MgError::Other(_)));
    assert!(error.to_string().contains("MGC_PYTHON_VERSION"));
}

#[test]
fn accepts_pure_python_wheels_and_rejects_source_distributions() {
    let pure = ResolvedGraph {
        packages: vec![resolved(
            "six",
            "1.17.0",
            "https://files.pythonhosted.org/six-1.17.0-py3-none-any.whl",
        )],
    };
    assert!(crate::install::validate_python_importable_graph(&pure).is_ok());

    let source = ResolvedGraph {
        packages: vec![resolved(
            "example",
            "1.0.0",
            "https://files.pythonhosted.org/example-1.0.0.tar.gz",
        )],
    };
    assert!(crate::install::validate_python_importable_graph(&source).is_err());
}
