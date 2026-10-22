//! Stable 3-way merge API wiring.
//! Wiring API merge lockfile 3 chiều ổn định.

#[path = "merge.rs"]
mod implementation;
mod merge_conflict;
mod package_index;
mod resolve_git_conflict_markers;

pub use self::implementation::merge3;
pub use self::merge_conflict::MergeConflict;
pub use self::resolve_git_conflict_markers::resolve_git_conflict_markers;
