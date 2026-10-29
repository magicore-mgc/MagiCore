//! Immutable keg store owner wiring.
//! Wiring owner store keg bất biến.

mod keg;

pub use keg::{KEG_RECORD_FILE, KegRecord, KegStore, STAGING_PREFIX, read_record, write_record};
