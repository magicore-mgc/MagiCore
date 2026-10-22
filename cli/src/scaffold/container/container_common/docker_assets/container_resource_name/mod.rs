//! Container image-name owner wiring.
//! Wiring owner tên image container.

mod container_resource_name;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
pub(in crate::scaffold::container) use self::container_resource_name::container_resource_name;
