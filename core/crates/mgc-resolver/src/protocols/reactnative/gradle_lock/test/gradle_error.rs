//! Gradle error-category compatibility tests.
//! Test tương thích phân loại lỗi Gradle.

use super::*;

#[test]
fn local_errors_keep_the_existing_other_category_and_text() {
    let malformed: MgError = GradleLockError::MalformedLine {
        line: 3,
        input: "bad:row".to_string(),
    }
    .into();
    assert!(
        matches!(malformed, MgError::Other(message) if message == "gradle.lockfile line 3 is not group:artifact:version[:configuration]: 'bad:row' (fail-closed)")
    );
    let conflict: MgError = GradleLockError::ConflictingVersions {
        coordinate: "a.b:c".to_string(),
        first_version: "1.0.0".to_string(),
        second_version: "2.0.0".to_string(),
    }
    .into();
    assert!(
        matches!(conflict, MgError::Other(message) if message == "a.b:c is pinned to both 1.0.0 and 2.0.0 in gradle.lockfile — refusing to guess (fail-closed)")
    );
}
