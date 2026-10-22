//! Container entrypoint writer owner wiring.
//! Wiring owner ghi entrypoint container.

mod write_container_entrypoint;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
pub(in crate::scaffold::container) use self::write_container_entrypoint::write_container_entrypoint;
