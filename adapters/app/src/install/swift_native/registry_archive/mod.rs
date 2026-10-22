//! Wires the private registry_archive owner modules.
//! Nối các module private thuộc owner registry_archive.

mod registry_archive;
pub(in crate::install) use registry_archive::install_registry_archive;
