#[path = "project_local_executable.rs"]
mod project_local_executable;

pub(super) use project_local_executable::is_project_local_executable;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
