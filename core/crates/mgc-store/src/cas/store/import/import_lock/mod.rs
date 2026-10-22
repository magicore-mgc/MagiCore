//! Import-lock owner wiring.
//! Wiring owner khóa import CAS.

mod import_lock;
pub(in crate::cas::store::import) use import_lock::lock_digest;
