// Dart pub and Flutter build guard tests.
// Test guard pub Dart và build Flutter.

use super::*;

#[test]
fn dart_pub_is_denied_and_flutter_build_requires_no_pub() {
    assert!(denial_reason("dart", &["pub".into(), "get".into()], "pub").is_some());
    assert!(denial_reason("flutter", &[], "build").is_some());
    assert!(denial_reason("flutter", &["--no-pub".into()], "build").is_none());
    assert!(denial_reason("flutter", &[], "version").is_none());
}
