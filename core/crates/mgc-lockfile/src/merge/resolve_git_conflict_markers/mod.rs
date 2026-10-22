//! Git conflict-marker resolver owner wiring.
//! Wiring owner xử lý Git conflict marker.

#[path = "resolve_git_conflict_markers.rs"]
mod implementation;
pub use self::implementation::resolve_git_conflict_markers;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
