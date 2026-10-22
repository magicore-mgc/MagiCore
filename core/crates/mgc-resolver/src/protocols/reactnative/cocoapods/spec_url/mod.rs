//! CocoaPods spec URL owner wiring.
//! Nối owner URL spec CocoaPods.

#[path = "spec_url.rs"]
mod implementation;

pub use implementation::cocoapods_spec_url;
