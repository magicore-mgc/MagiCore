//! Web package tarball prefetch owner wiring.
//! Wiring owner prefetch tarball của package Web.

mod error;
mod prefetch;

pub(super) use error::PrefetchError;
pub use prefetch::prefetch_tarballs;
