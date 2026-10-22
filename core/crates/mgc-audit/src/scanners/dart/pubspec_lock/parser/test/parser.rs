//! Pub lock parser order tests.
//! Test thứ tự parser lockfile Pub.

use super::*;

#[test]
fn parses_multiple_sections_and_keeps_pin_order() {
    let raw = "packages:\n  first:\n    version: \"1\"\n  second:\n    version: \"2\"\nsdks:\n  dart: 3\n";
    let (pins, skipped) = parse(raw).expect("parse Pub lock fixture");
    let names = pins.iter().map(|pin| pin.name.as_str()).collect::<Vec<_>>();
    assert_eq!(names, vec!["first", "second"]);
    assert!(skipped.is_empty());
}

#[test]
fn ignores_package_shaped_lines_outside_packages_section() {
    let (pins, skipped) = parse("sdks:\n  package:\n    version: 1\n")
        .expect("ignore package-like content outside packages section");
    assert!(pins.is_empty());
    assert!(skipped.is_empty());
}
