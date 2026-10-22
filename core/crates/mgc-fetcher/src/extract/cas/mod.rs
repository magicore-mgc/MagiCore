//! Exports stable CAS extraction entry points.
//! Export các API giải nén CAS ổn định.

mod cas;
pub(super) use self::cas::ExtractionDurability;
pub use self::cas::{
    extract_tarball_to_cas_and_link, extract_tarball_to_cas_and_verify_existing_root,
    extract_tarball_to_rebuildable_cas_and_link,
    extract_tarball_to_rebuildable_cas_and_verify_existing_root,
};
#[cfg(test)]
mod test;
