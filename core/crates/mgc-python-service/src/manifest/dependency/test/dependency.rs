//! Simple Python requirement conversion tests.
//! Test chuyển đổi requirement Python đơn giản.

use super::super::error::PythonDependencyError;
use super::super::{parse_python_dependency, python_dependency_value};
use mgc_types::MgError;

#[test]
fn parses_supported_name_and_version_operators() {
    for (source, expected_name, expected_range) in [
        ("requests>=2", "requests", ">=2"),
        ("six==1.17.0", "six", "1.17.0"),
        ("urllib3~=2.0", "urllib3", "~2.0"),
    ] {
        let dependency = parse_python_dependency(source).expect("parse supported requirement");
        assert_eq!(dependency.name.as_str(), expected_name);
        assert_eq!(dependency.range.as_str(), expected_range);
    }
}

#[test]
fn rejects_requirement_syntax_the_shared_model_cannot_preserve() {
    for source in [
        "requests[socks]>=2",
        "requests>=2;python_version<'3.12'",
        "demo @ https://example.invalid/demo.whl",
        "requests>=2,<3",
    ] {
        assert!(parse_python_dependency(source).is_err(), "{source}");
    }
    let error = parse_python_dependency("requests[extra]").expect_err("reject extras");
    assert_eq!(
        error.to_string(),
        concat!(
            "PEP 508 extras, markers, URL references, and compound ",
            "constraints are unsupported by the native resolver"
        )
    );
    let source = match error {
        MgError::Anyhow(source) => source,
        other => panic!("expected typed dependency error, got {other}"),
    };
    assert!(source.downcast_ref::<PythonDependencyError>().is_some());

    let error = parse_python_dependency("requests>=2#").expect_err("reject invalid version token");
    assert_eq!(
        error.to_string(),
        "unsupported PEP 508 version requirement in 'requests>=2#'"
    );
    assert!(matches!(error, MgError::Anyhow(_)));
}

#[test]
fn serializes_star_and_exact_pin_without_widening_the_pin() {
    let unpinned = parse_python_dependency("requests").expect("parse unpinned requirement");
    assert_eq!(
        python_dependency_value(&unpinned).as_str(),
        Some("requests")
    );

    let pinned = parse_python_dependency("six==1.17.0").expect("parse exact pin");
    assert_eq!(
        python_dependency_value(&pinned).as_str(),
        Some("six==1.17.0")
    );
}
