//! Wires lockfile mutation authorization and its local error mapping.
//! Nối policy cho phép mutation lockfile và ánh xạ lỗi cục bộ.

mod error;
mod mutation_policy;

pub use self::mutation_policy::ensure_lockfile_mutation_allowed;
pub(in crate::writer) use self::mutation_policy::ensure_not_v4_legacy_write;
