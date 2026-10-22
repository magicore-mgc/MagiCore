#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn parses_all_anchor_mode_aliases_and_rejects_unknown_values() {
    assert_eq!(AnchorMode::parse("OFF"), Some(AnchorMode::Off));
    assert_eq!(AnchorMode::parse("false"), Some(AnchorMode::Off));
    assert_eq!(AnchorMode::parse("warn"), Some(AnchorMode::Warn));
    assert_eq!(AnchorMode::parse(""), Some(AnchorMode::Warn));
    assert_eq!(AnchorMode::parse("STRICT"), Some(AnchorMode::Require));
    assert_eq!(AnchorMode::parse("yes"), Some(AnchorMode::Require));
    assert_eq!(AnchorMode::parse("nope"), None);
}
