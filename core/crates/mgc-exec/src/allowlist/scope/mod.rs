#[path = "scope.rs"]
mod scope;

pub use scope::{ExecutionScope, ScopeConstraints};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
