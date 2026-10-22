//! Wire canonical project path resolution and nearest-owner tests.
//! Nối resolve path project canonical và test gần owner.

mod project_path;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::project_path::{canonical_root, resolve_project_path};
