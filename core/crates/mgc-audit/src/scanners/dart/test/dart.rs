//! Public Dart scanner route tests.
//! Test route public scanner Dart.

use super::*;

#[test]
fn parser_route_keeps_empty_lock_behavior() {
    let (pins, skipped) =
        read_pubspec_lock("sdks:\n  dart: '>=3.0.0'\n").expect("parse empty Dart lock fixture");
    assert!(pins.is_empty());
    assert!(skipped.is_empty());
}
