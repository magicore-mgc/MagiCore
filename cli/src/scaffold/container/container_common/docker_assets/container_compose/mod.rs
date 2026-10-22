//! Docker Compose document owner wiring.
//! Wiring owner tài liệu Docker Compose.

mod container_compose;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
pub(in crate::scaffold::container) use self::container_compose::container_compose;
