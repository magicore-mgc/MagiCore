//! CocoaPods protocol owner wiring and stable exports.
//! Nối owner CocoaPods và giữ export ổn định.

mod client;
mod dependencies;
#[path = "cocoapods.rs"]
mod implementation;
mod spec_url;

pub use implementation::{CocoaPodsProtocol, VerifiedPodSpec};
pub use spec_url::cocoapods_spec_url;
