//! CocoaPods manifest facade wiring.
//! Wiring facade manifest CocoaPods.

mod podfile;
pub(super) use podfile::{parse_podfile, write_podfile};
