//! Homebrew-style keg/prefix transfer layer for the lib core.
//! Lớp luân chuyển keg/prefix kiểu Homebrew cho core lib.
//!
//! Every installed library version lives in an immutable keg; a prefix exposes
//! the active version through relative links; installing a new version never
//! deletes an older keg, so switching back stays possible.
//! Mỗi version library đã cài nằm trong một keg bất biến; prefix phơi version
//! active qua link tương đối; cài version mới không xoá keg cũ nên switch lại
//! luôn được.

mod active;
mod binary_manifest;
mod error;
mod guard;
mod immutability;
mod install;
mod keg;
mod layout;
mod lib_keg;
mod prefix;
mod switch;

pub use active::{ACTIVE_POINTER_FILE, ActiveState, pointer_path, read_active, write_active};
pub use binary_manifest::{
    BinaryEntry, BinaryManifest, MANIFEST_DOMAIN, load_manifest, manifest_message, verify_manifest,
};
pub use error::{KegError, KegResult};
pub use guard::{EntryState, KegGuard, Undo, capture_entry, restore_entry};
pub use immutability::{KegFingerprint, remove_keg, seal_keg, thaw_keg, verify_keg};
pub use install::{InstallOutcome, install_and_activate};
pub use keg::{KEG_RECORD_FILE, KegRecord, KegStore, STAGING_PREFIX, read_record, write_record};
pub use layout::{
    ACTIVE_LINK, CELLAR_DIR, KegLayout, OPT_DIR, PREFIX_DIR, PREFIX_ENTRY_PREFIX,
    validate_library_name, validate_relative, validate_version,
};
pub use lib_keg::{
    KegListing, lib_active, lib_install, lib_rollback, lib_switch, lib_switch_spec, list_kegs,
    split_spec,
};
pub use prefix::{
    PrefixLink, create_link_to, create_symlink, ensure_link_parents, link_target, place_link,
    plan_links, relative_target, remove_link,
};
pub use switch::{SwitchOutcome, active_state, rollback_active, switch_active};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
