//! install library owner wiring — wiring owner install library.

#[path = "library.rs"]
mod implementation;
#[path = "install_lib/mod.rs"]
mod install_lib;

pub use implementation::*;
pub use install_lib::{
    InstallLibOutcome, InstallLibRequest, TRUST_KEY_ENV, execute as install_lib,
};
