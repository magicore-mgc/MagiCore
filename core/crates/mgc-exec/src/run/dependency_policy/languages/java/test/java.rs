// Java build-tool offline guard tests.
// Test guard offline cho build tool Java.

use super::*;

#[test]
fn gradle_and_maven_require_their_offline_forms() {
    assert!(denial_reason("gradle", &[]).is_some());
    assert!(denial_reason("gradlew", &["--offline".into()]).is_none());
    assert!(denial_reason("mvn", &[]).is_some());
    assert!(denial_reason("mvnw", &["-o".into()]).is_none());
    assert!(denial_reason("mvn", &["--offline".into()]).is_none());
}
