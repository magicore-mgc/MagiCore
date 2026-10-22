//! Match Cargo manifest roots to the MGC package graph.
//! So khớp root Cargo với package graph của MGC.

mod root_resolution;
pub(in crate::install) use root_resolution::resolve_manifest_roots;
