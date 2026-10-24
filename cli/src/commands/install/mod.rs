//! Generic CLI dependency-install command.
//! Lệnh cài dependency tổng quát của CLI.

mod install;
mod install_sbom;

pub(crate) use install_sbom::write_sbom as write_install_sbom;
mod locked_graph;
mod materialize;
mod mutation;
mod validation;
mod workspace;

pub use install::run;
pub(crate) use mutation::install_into_root_ws;
pub(crate) use workspace::discover_workspace_projects;
