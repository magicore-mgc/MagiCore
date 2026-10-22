//! TOML ecosystem parser wiring.
//! Wiring parser ecosystem TOML.
mod ecosystem_from_config;
pub(in crate::project::project) use ecosystem_from_config::ecosystem_from_config;
pub(in crate::project::project::file_access) use ecosystem_from_config::ecosystem_from_config_typed;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
