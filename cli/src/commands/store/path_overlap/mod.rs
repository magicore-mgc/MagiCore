//! Shared Store path-overlap owner wiring — nối owner overlap path Store dùng chung.

mod path_overlap;

pub(in crate::commands) use path_overlap::paths_overlap;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
