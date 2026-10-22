//! Publish dependency projection owner exports.
//! Export xử lý projection dependency cho publish.

mod dependency_fields;

pub use dependency_fields::dep_fields;

#[cfg(test)]
#[path = "test/dependency_fields.rs"]
mod tests;
