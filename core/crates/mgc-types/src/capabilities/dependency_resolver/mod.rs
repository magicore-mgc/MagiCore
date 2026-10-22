//! Dependency resolver capability module wiring.
//! Wiring module capability resolve dependency.

mod dependency_resolver;
pub use self::dependency_resolver::DependencyResolver;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
