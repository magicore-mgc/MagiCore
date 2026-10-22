//! Package identity and dependency reference owners — Owner định danh package và dependency

mod identity;

pub(crate) use self::identity::{legacy_bom_ref, resolve_legacy_dependency_ref, v4_bom_ref};
