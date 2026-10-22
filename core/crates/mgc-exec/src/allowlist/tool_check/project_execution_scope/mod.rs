#[path = "project_execution_scope.rs"]
mod project_execution_scope;

pub(super) use project_execution_scope::is_project_execution_scope;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
