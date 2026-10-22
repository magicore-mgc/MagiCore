//! Hook path owner exports.
//! Export owner đường dẫn hook.

mod paths;

pub use paths::hooks_paths;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
