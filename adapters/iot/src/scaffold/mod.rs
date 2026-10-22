//! IoT project scaffolding facade and stable API exports.
//! Facade scaffold IoT và export API ổn định.

mod framework;
mod scaffold_project;

#[cfg(test)]
pub(crate) mod test;

pub use scaffold_project::scaffold_project;
