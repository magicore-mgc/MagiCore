#[path = "env_key_validation.rs"]
mod env_key_validation;

pub(in crate::allowlist) use env_key_validation::is_valid_env_key;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
