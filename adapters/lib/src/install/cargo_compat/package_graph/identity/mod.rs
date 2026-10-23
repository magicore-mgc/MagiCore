//! Package identity owner.
//! Owner định danh package.

mod identity;

pub(crate) use identity::is_cargo_dependency_edge_marker;
pub(in crate::install) use identity::package_id;
