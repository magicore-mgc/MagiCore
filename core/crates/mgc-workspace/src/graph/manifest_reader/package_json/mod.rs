//! Node package.json parser facade.
//! Facade bộ phân tích package.json của Node.

mod package_json;

pub(super) use package_json::read_package_json;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
