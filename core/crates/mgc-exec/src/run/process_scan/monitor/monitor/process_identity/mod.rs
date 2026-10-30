//! Process identity wiring / Nối module identity process.
mod process_identity;
pub(in crate::run::implementation) use process_identity::{
    current_process_scan_snapshot, live_descendant_identities, process_identity_is_live,
};
