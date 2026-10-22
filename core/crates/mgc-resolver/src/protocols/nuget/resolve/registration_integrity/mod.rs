//! Wires the private registration_integrity owner modules.
//! Nối các module private thuộc owner registration_integrity.

mod registration_integrity;
pub(super) use registration_integrity::catalog_sha512_marker;
