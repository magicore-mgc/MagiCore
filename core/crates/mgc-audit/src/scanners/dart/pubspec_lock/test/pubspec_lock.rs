//! Pubspec lock parser entry tests.
//! Test entry parser pubspec.lock.

use super::*;

#[test]
fn pubspec_lock_entry_returns_pins_and_skip_reasons() {
    let raw = "packages:\n  http:\n    source: hosted\n    version: \"1.2.2\"\n";
    let (pins, skipped) = read(raw).expect("read Pub lock fixture");
    assert_eq!(pins.len(), 1);
    assert_eq!(pins[0].name, "http");
    assert!(skipped.is_empty());
}
