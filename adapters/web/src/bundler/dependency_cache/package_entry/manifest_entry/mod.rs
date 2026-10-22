//! Package manifest entry-selection wiring.
//! Wiring chọn entry từ manifest package.

#[path = "manifest_entry.rs"]
mod implementation;

pub(super) use self::implementation::resolve_manifest_entry;
