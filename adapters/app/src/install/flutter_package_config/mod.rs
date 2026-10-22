//! Flutter package metadata owner wiring.
//! Wiring owner metadata package Flutter.

mod flutter_package_config;
mod package_config;
mod package_graph;
mod project_uri;
mod types;
mod version_floor;

pub(super) use flutter_package_config::write_flutter_package_config;
#[cfg(test)]
pub(super) use package_config::write_flutter_package_config_with_sdk_root;
#[cfg(test)]
pub(super) use project_uri::flutter_project_root_uri;
