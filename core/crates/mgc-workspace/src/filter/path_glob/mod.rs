//! Relative path-glob facade.
//! Facade glob đường dẫn tương đối.

mod path_glob;

pub(super) use path_glob::glob_match_path;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
