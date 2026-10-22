//! Safe ecosystem load composition wiring.
//! Wiring composition nạp ecosystem an toàn.
mod read_to_string_ecosystem;
pub(in crate::project::project) use read_to_string_ecosystem::read_to_string_ecosystem;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
