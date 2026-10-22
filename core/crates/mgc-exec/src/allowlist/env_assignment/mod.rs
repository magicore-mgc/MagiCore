#[path = "env_assignment.rs"]
mod env_assignment;
#[path = "env_key_validation/mod.rs"]
mod env_key_validation;

pub(super) use env_assignment::{is_env_assignment, parse_env_assignment};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
