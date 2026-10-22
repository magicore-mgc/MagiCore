//! Package file containment wiring.
//! Wiring containment file package.

#[path = "path_containment.rs"]
mod implementation;

pub(super) use self::implementation::{canonicalize_if_present, resolve_file_inside_package};
