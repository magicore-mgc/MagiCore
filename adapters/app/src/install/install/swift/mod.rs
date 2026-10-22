//! Swift native install and lock-publication wiring.
//! Wiring install native Swift và công bố lock.

mod swift;

pub(super) use swift::run_swift_install;
