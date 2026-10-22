//! Candidate rejection reasons stay tied to the selection rule.
//! Lý do loại candidate gắn đúng với rule chọn release.

use super::super::{ReleaseCandidate, candidate_incompatibility};
use crate::protocols::pypi::PypiFile;
use mgc_types::Version;

fn file(packagetype: &str) -> PypiFile {
    serde_json::from_str(&format!(
        r#"{{"filename":"demo.tar.gz","packagetype":"{packagetype}"}}"#
    ))
    .expect("test operation must succeed")
}

#[test]
fn requires_python_exclusion_precedes_file_compatibility() {
    let candidate = ReleaseCandidate {
        version: Version::parse("1.0.0").expect("test operation must succeed"),
        files: vec![file("sdist")],
    };

    assert_eq!(
        candidate_incompatibility(&candidate, ">=3.10", 3, 9),
        Some("1.0.0 requires >=3.10".to_string())
    );
}

#[test]
fn missing_installable_file_has_a_distinct_exclusion_reason() {
    let candidate = ReleaseCandidate {
        version: Version::parse("1.0.0").expect("test operation must succeed"),
        files: vec![file("bdist_egg")],
    };

    assert_eq!(
        candidate_incompatibility(&candidate, "", 3, 11),
        Some("1.0.0 has no artifact compatible with Python 3.11".to_string())
    );
}

#[test]
fn compatible_source_distribution_is_not_excluded() {
    let candidate = ReleaseCandidate {
        version: Version::parse("1.0.0").expect("test operation must succeed"),
        files: vec![file("sdist")],
    };

    assert_eq!(candidate_incompatibility(&candidate, ">=3.8", 3, 11), None);
}
