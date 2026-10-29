//! Keg/prefix path layout owner wiring.
//! Wiring owner layout đường dẫn keg/prefix.

mod layout;

pub use layout::{
    ACTIVE_LINK, CELLAR_DIR, KegLayout, OPT_DIR, PREFIX_DIR, PREFIX_ENTRY_PREFIX,
    validate_library_name, validate_relative, validate_version,
};
