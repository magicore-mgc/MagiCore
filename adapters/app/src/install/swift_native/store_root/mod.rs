//! Wires the private store_root owner modules.
//! Nối các module private thuộc owner store_root.

mod store_root;
pub(in crate::install) use store_root::swift_store_root;
