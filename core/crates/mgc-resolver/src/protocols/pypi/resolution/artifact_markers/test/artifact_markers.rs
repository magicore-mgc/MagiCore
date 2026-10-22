//! Marker values stay stable for wheel and source distribution records.
//! Giá trị marker giữ ổn định cho wheel và source distribution.

use super::super::{PypiFile, project_artifact_markers};

fn file(filename: &str, packagetype: &str, requires_python: Option<&str>) -> PypiFile {
    let required = requires_python
        .map(|value| format!(",\"requires_python\":\"{value}\""))
        .unwrap_or_default();
    serde_json::from_str(&format!(
        r#"{{"filename":"{filename}","packagetype":"{packagetype}"{required}}}"#
    ))
    .expect("test operation must succeed")
}

#[test]
fn wheel_marker_preserves_python_abi_and_platform_tags() {
    let markers = project_artifact_markers(&file(
        "demo-1.0.0-py3-none-any.whl",
        "bdist_wheel",
        Some(">=3.9"),
    ));

    assert_eq!(
        markers,
        vec![
            String::from("wheel:py3-none-any"),
            String::from("requires-python:>=3.9")
        ]
    );
}

#[test]
fn source_marker_preserves_filename_and_python_constraint() {
    let markers = project_artifact_markers(&file("demo-1.0.0.tar.gz", "sdist", Some(">=3.8")));

    assert_eq!(
        markers,
        vec![
            String::from("sdist:demo-1.0.0.tar.gz"),
            String::from("requires-python:>=3.8")
        ]
    );
}
