//! Wires the private package_source owner modules.
//! Nối các module private thuộc owner package_source.

mod package_source;
pub(in crate::install) use package_source::{SwiftPackageSource, swift_package_source};
