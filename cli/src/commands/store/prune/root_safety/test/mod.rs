//! Store root and database path tests.
//! Test root store và path DB.

#[path = "root_safety.rs"]
mod root_safety;
#[cfg(any(unix, windows))]
#[path = "symlink_support.rs"]
mod symlink_support;
