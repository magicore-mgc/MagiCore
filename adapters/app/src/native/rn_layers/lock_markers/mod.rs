//! Export React Native tier and CocoaPods provenance marker helpers.
//! Export helper marker tier React Native và provenance CocoaPods.

mod lock_markers;
pub(super) use lock_markers::{cocoapods_registry_tag, tag_tier};
