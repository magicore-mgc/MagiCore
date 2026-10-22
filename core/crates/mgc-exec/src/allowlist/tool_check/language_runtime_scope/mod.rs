#[path = "language_runtime_scope.rs"]
mod language_runtime_scope;

pub(super) use language_runtime_scope::restrict_language_runtime_scope;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
