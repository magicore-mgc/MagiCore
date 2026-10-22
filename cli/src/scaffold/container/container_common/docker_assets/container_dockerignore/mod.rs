//! Shared dockerignore owner wiring.
//! Wiring owner dockerignore dùng chung.

mod container_dockerignore;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
pub(in crate::scaffold::container) use self::container_dockerignore::container_dockerignore;
