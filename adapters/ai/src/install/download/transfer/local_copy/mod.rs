//! Local transfer owner wiring.
//! Wiring owner truyền file local.

mod local_copy;
pub(in crate::install::download) use local_copy::copy_local_to_staging;
