//! Container profile output facade — exports Docker assets and Web names.
//! Facade output profile container — xuất asset Docker và tên Web.

#[path = "docker_assets/mod.rs"]
mod docker_assets;
#[path = "web_resource_name/mod.rs"]
mod web_resource_name;

pub(super) use docker_assets::{
    append_docker_readme, container_compose, container_dockerignore, container_resource_name,
    write_container_entrypoint,
};
pub(super) use web_resource_name::web_container_resource_name;
