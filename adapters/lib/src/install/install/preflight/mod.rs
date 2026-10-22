//! Lib native-install input preflight wiring.
//! Wiring preflight input install native của Lib.

mod preflight;

pub(in crate::install) use preflight::validate_native_install_inputs;
