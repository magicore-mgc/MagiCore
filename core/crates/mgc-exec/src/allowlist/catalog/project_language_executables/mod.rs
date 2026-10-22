#[path = "project_language_executables.rs"]
mod project_language_executables;

pub(crate) use project_language_executables::PROJECT_LANGUAGE_EXECUTABLES;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
