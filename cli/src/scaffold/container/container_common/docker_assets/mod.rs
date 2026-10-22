//! Shared Docker asset wiring and helper exports.
//! Wiring và export helper asset Docker dùng chung.

mod append_docker_readme;
mod container_compose;
mod container_dockerignore;
mod container_resource_name;
mod write_container_entrypoint;

pub(in crate::scaffold::container) use self::append_docker_readme::append_docker_readme;
pub(in crate::scaffold::container) use self::container_compose::container_compose;
pub(in crate::scaffold::container) use self::container_dockerignore::container_dockerignore;
pub(in crate::scaffold::container) use self::container_resource_name::container_resource_name;
pub(in crate::scaffold::container) use self::write_container_entrypoint::write_container_entrypoint;
