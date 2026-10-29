//! `mgc lib` keg facade owner wiring.
//! Wiring owner facade keg `mgc lib`.

mod lib_keg;

pub use lib_keg::{
    KegListing, install_binary, lib_active, lib_install, lib_rollback, lib_switch, lib_switch_spec,
    list_kegs, split_spec,
};
