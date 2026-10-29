//! install-lib owner wiring.
//! Wiring owner install-lib.

mod install_lib;

pub use install_lib::{InstallLibOutcome, InstallLibRequest, TRUST_KEY_ENV, execute};
