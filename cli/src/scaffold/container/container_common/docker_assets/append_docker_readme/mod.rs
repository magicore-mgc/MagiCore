//! Docker README append owner wiring.
//! Wiring owner bổ sung README Docker.

mod append_docker_readme;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
pub(in crate::scaffold::container) use self::append_docker_readme::append_docker_readme;
