//! Focused PyPI simple-index file record tests.
//! Test riêng record file của PyPI simple-index.

use crate::model::PypiFile;

#[test]
fn test_pypi_file_round_trips_optional_python_requirement() {
    let file = PypiFile {
        name: "pkg".to_string(),
        version: "1.0.0".to_string(),
        filename: "pkg-1.0.0.whl".to_string(),
        digest: "sha256:abc".to_string(),
        size: 42,
        requires_python: Some(">=3.9".to_string()),
    };
    let json = serde_json::to_value(&file).unwrap();
    let decoded: PypiFile = serde_json::from_value(json).unwrap();

    assert_eq!(decoded.name, "pkg");
    assert_eq!(decoded.requires_python.as_deref(), Some(">=3.9"));
}
